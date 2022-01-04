use ggez::{
    graphics::{self},
    Context,
    GameResult,
    event,
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

use PrimitiveIsaac::{
    entities::*,
    assets::*,
};

struct MainState {
    screen_width: f32,
    screen_height: f32,
    assets: Assets,
    player: Player,
}

impl MainState {
    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let player = Player::new(
            Vec2::new(screen_width/2.0, screen_height/2.0).into(),
            Vec2::new(0.5, 0.5),
            0.,
            Vec2::new(1., 0.),
            20.
        );

        let s = MainState {
            screen_width, 
            screen_height,           
            assets,
            player,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.player.draw(ctx, &self.assets);

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymod: input::keyboard::KeyMods, _repeat: bool) {
        match keycode {
            event::KeyCode::W | event::KeyCode::Up => {
                self.player.translation = Vec2::new(0., -1.);
                self.player.angle = PI / 2.;
            }
            event::KeyCode::S | event::KeyCode::Down => {
                self.player.translation = Vec2::new(0., 1.);
                self.player.angle = 3. * PI / 2.;
            }
            event::KeyCode::A | event::KeyCode::Left => {
                self.player.translation = Vec2::new(-1., 0.);
                self.player.angle = PI;
            }
            event::KeyCode::D | event::KeyCode::Right => {
                self.player.translation = Vec2::new(1., 0.);
                self.player.angle = 0.;
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymod: input::keyboard::KeyMods) {
        match keycode {
            event::KeyCode::W | event::KeyCode::Up | event::KeyCode::S | event::KeyCode::Down | event::KeyCode::A | event::KeyCode::Left | event::KeyCode::D | event::KeyCode::Right => {
                self.player.translation = Vec2::ZERO;
            }
            _ => (),
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
