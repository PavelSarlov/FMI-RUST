use ggez::{
    graphics::{self},
    Context,
    GameResult,
    event,
    conf::{Conf,WindowMode},
    ContextBuilder,
    filesystem,
};
use std::{
    env,
    path,
    f32::consts::PI,
};
use glam::f32::{Vec2};

use PrimitiveIsaac::player::*;

struct MainState {
    screen_width: f32,
    screen_height: f32,
    player: Player,
}

impl MainState {
    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let player = Player::new(Vec2::new(screen_width/2.0, screen_height/2.0).into());

        let s = MainState {
            screen_width, 
            screen_height,           
            player,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.player.draw(ctx);

        graphics::present(ctx)?;
        Ok(())
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
