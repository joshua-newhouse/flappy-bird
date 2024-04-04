#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_HIEGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;
const FRAME_DURATION_MILLIS: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    game_mode: GameMode,
    player: Dragon,
    frame_time: f32,
    score: i32,
    wall: Wall
}

impl State {
    fn new() -> Self {
        State {
            game_mode: GameMode::Menu,
            player: Dragon::new(5, 25),
            frame_time: 0.0,
            score: 0,
            wall: Wall::new(SCREEN_WIDTH, 0)
        }
    }

    fn transition_to_menu(&mut self) {
        self.game_mode = GameMode::Menu;
    }

    fn transition_to_playing(&mut self) {
        self.game_mode = GameMode::Playing;
    }

    fn transition_to_end(&mut self) {
        self.game_mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.player = Dragon::new(5, 25);
        self.wall = Wall::new(SCREEN_WIDTH, 0);
        self.score = 0;
        self.transition_to_playing();
    }
    
    fn render(&self, ctx: &mut BTerm) {
        ctx.print(0, 0, "Press SPACE to flap your dragon's wings");
        ctx.print(0, 1, &format!("Score: {}", self.score));
    
        self.player.render(ctx);
        self.wall.render(self.player.x, ctx);
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => (),
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION_MILLIS {
            self.frame_time = 0.0;
            self.player.apply_gravity_and_move();
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap_wings(),
                _ => (),
            }
        }
        
        self.render(ctx);

        if self.player.y > SCREEN_HIEGHT || self.wall.collision_detected(&self.player) {
            self.transition_to_end();
        }
        
        if self.player.x > self.wall.x {
            self.score += 1;
            self.wall = Wall::new(self.player.x + SCREEN_WIDTH, self.score);
        }        

    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(9, "You are dead!");
        ctx.print_centered(10, &format!("Your score was {}", self.score));
        ctx.print_centered(11, "Press any key to continue...");

        if let Some(_) = ctx.key {
            self.transition_to_menu();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.game_mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

struct Dragon {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Dragon {
    const TERMINAL_VELOCITY: f32 = 2.0;

    fn new(x: i32, y: i32) -> Self {
        Dragon {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn apply_gravity_and_move(&mut self) {
        if self.velocity < Self::TERMINAL_VELOCITY {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }

        self.x += 1;
    }

    fn flap_wings(&mut self) {
        self.velocity = if self.velocity > -2.0 { self.velocity - 0.9 } else { -2.0 };
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }
}

struct Wall {
    x: i32,
    gap_y: i32,
    half_size: i32,
}

impl Wall {
    fn new(x: i32, score: i32) -> Wall {
        let mut random = RandomNumberGenerator::new();
        Wall {
            x,
            gap_y: random.range(10, 40),
            half_size: i32::max(2, 20 - score) / 2,
        }
    }
    
    fn collision_detected(&self, dragon: &Dragon) -> bool {
        let gap_top = self.gap_y - self.half_size;
        let gap_bottom = self.gap_y + self.half_size;
    
        self.x == dragon.x && (dragon.y < gap_top || dragon.y > gap_bottom)
    }

    fn render(&self, player_x: i32, ctx: &mut BTerm) {
        let screen_x = self.x - player_x;
        let gap_top = self.gap_y - self.half_size;
        let gap_bottom = self.gap_y + self.half_size;

        for y in 0..SCREEN_HIEGHT {
            if y > gap_top && y < gap_bottom {
                continue;
            }

            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
