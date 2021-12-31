use ggez::{
    event,
    graphics::{self, Color, DrawMode, Mesh, DrawParam},
    GameResult,
    Context,
};
use crate::utils::{Vec2Wrap};

pub struct Player {
    pos: Vec2Wrap,
}

impl Player {
    pub fn new(pos: Vec2Wrap) -> Self {
        Player {
            pos,
        }
    }
}

impl event::EventHandler<ggez::GameError> for Player {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.pos,
            20.0,
            1.0,
            Color::WHITE,
        )?;

        graphics::draw(ctx, &mesh, DrawParam::default())?;
        Ok(())
    }
}
