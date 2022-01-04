use ggez::{
    graphics::{self, Vertex},
    Context,
    GameResult,
};
use glam::f32::*;
use crate::utils::{Vec2Wrap, MyVertex};


pub struct Assets {
    pub player_base: graphics::Image,
    pub player_shoot: graphics::Image,
    pub shot_base: graphics::Image,
    pub _BOX_VERTS: [MyVertex; 4],
    pub _BOX_INDICES: [u32; 6],
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let player_base = graphics::Image::new(ctx, "/player_base.png")?;
        let player_shoot = graphics::Image::new(ctx, "/player_shoot.png")?;
        let shot_base = graphics::Image::new(ctx, "/shot_base.png")?;
        let _BOX_VERTS = [MyVertex{pos: Vec2Wrap::from([0., 0.]), uv: [0., 0.], color: [1.,1.,1.,1.]}, MyVertex{pos: Vec2Wrap::from([1., 0.]), uv: [1., 0.], color: [1.,1.,1.,1.]},
                          MyVertex{pos: Vec2Wrap::from([1., 1.]), uv: [1., 1.], color: [1.,1.,1.,1.]}, MyVertex{pos: Vec2Wrap::from([0., 1.]), uv: [0., 1.], color: [1.,1.,1.,1.]}];
        let _BOX_INDICES = [0, 1, 2, 2, 3, 0];

        Ok(Self {
            player_base,
            player_shoot,
            shot_base,
            _BOX_VERTS,
            _BOX_INDICES,
        })
    }
}

// pub trait Sprite: Debug {
//     fn draw(&mut self, center: Point2<f32>, ctx: &mut Context) -> GameResult<()>;
//     fn width(&self, ctx: &mut Context) -> f32;
//     fn height(&self, ctx: &mut Context) -> f32;
// }

// impl Sprite for TextSprite {
//     fn draw(&mut self, top_left: Point2<f32>, ctx: &mut Context) -> GameResult<()> {
//         graphics::draw(ctx, &self.text, graphics::DrawParam::default().dest(top_left))
//     }

//     fn width(&self, ctx: &mut Context) -> f32 { self.text.width(ctx) }
//     fn height(&self, ctx: &mut Context) -> f32 { self.text.height(ctx) }
// }
