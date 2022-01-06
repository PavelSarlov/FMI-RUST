use ggez::{
    graphics::{self},
    Context,
    GameResult,
    event::{self,KeyCode,MouseButton,EventHandler},
    conf::{Conf,WindowMode},
    ContextBuilder,
    filesystem,
    input,
    timer,
};
use std::{
    env,
    path,
    f32::consts::PI,
};
use glam::f32::{Vec2};

use puker::{
    entities::*,
    assets::*,
    utils::*,
    dungeon::*,
};

struct MainState {
    screen_width: f32,
    screen_height: f32,
    assets: Assets,
    player: Player,
    dungeon: Dungeon,
    cur_room: usize,
}

impl MainState {
    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        input::mouse::set_cursor_grabbed(ctx, true)?;

        let assets = Assets::new(ctx)?;
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let player = Player::new(
            Vec2::new(0., 0.).into(),
            Vec2::new(0.5, 0.5),
            0.,
            Vec2::new(0., 0.),
            Vec2::new(1., 0.),
            5.
        );
        let dungeon = Dungeon::generate_dungeon();
        let cur_room = Dungeon::get_start_room();

        let s = MainState {
            screen_width, 
            screen_height,           
            assets,
            player,
            dungeon,
            cur_room,
        };

        Ok(s)
    }

    fn mouse_relative_angle(&self, mouse: Vec2) -> f32 {
        let ppos = self.player.get_pos();
        let m = screen_to_world_space(self.screen_width, self.screen_height, mouse);

        let dx = m.x - ppos.x;
        let dy = m.y - ppos.y;

        if f32::abs(dx) > f32::abs(dy) {
            return f32::signum(dx) * PI / 2.;
        }
        (f32::signum(dy) * PI + PI) / 2.
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.player.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.dungeon.draw_room(ctx, &self.assets, (self.screen_width, self.screen_height), self.cur_room)?;

        self.player.draw(ctx, &self.assets, (self.screen_width, self.screen_height))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: input::keyboard::KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::W => {
                self.player.translation.y = 1.;
            },
            KeyCode::S => {
                self.player.translation.y = -1.;
            },
            KeyCode::A => {
                self.player.translation.x = -1.;
            },
            KeyCode::D => {
                self.player.translation.x = 1.;
            },
            KeyCode::Up => {
                self.player.angle = 0.;
                self.player.state = PlayerState::SHOOT;
            },
            KeyCode::Down => {
                self.player.angle = PI;
                self.player.state = PlayerState::SHOOT;
            },
            KeyCode::Left => {
                self.player.angle = -PI / 2.;
                self.player.state = PlayerState::SHOOT;
            },
            KeyCode::Right => {
                self.player.angle = PI / 2.;
                self.player.state = PlayerState::SHOOT;
            },
            KeyCode::Space => {
            },
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: input::keyboard::KeyMods) {
        match keycode {
            KeyCode::W | KeyCode::S | KeyCode::A | KeyCode::D => {
                self.player.translation = Vec2::ZERO;
            },
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                self.player.angle = 0.;
                self.player.state = PlayerState::BASE;
            },
            _ => (),
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                self.player.angle = self.mouse_relative_angle(Vec2::new(x, y));
                self.player.state = PlayerState::SHOOT;
            },
            _ => (),
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        match button {
            MouseButton::Left => {
                self.player.angle = 0.;
                self.player.state = PlayerState::BASE;
            },
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if input::mouse::button_pressed(_ctx, MouseButton::Left) {
            self.player.angle = self.mouse_relative_angle(Vec2::new(x, y));
        }
    }
}

fn main() -> GameResult {
    let conf = Conf::new()
        .window_mode(WindowMode {
            width: 800.0,
            height: 600.0,
            ..Default::default()
        });

    let (mut ctx, event_loop) = ContextBuilder::new("PrimitiveIsaac", "Window")
        .default_conf(conf.clone())
        .build()
        .unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let state = MainState::new(&mut ctx, &conf)?;

    event::run(ctx, event_loop, state)
}
