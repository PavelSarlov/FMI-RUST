use ggez::{
    event,
    graphics::{self, Color, DrawMode, Mesh, DrawParam, Rect},
    GameResult,
    Context,
};
use crate::{
    utils::*,
    assets::*,
};
use glam::f32::{Vec2, Mat3};
use std::f32::consts::PI;

pub trait Model {
    fn bounding_box(&self) -> graphics::Rect;

    fn translate(&mut self);

    fn rotate(&mut self, angle: f32);

    fn scale(&mut self, scale: Vec2);

    // fn resolve_collision(&mut self, );
}


pub struct Player {
    pub pos: Vec2Wrap,
    pub scale: Vec2,
    pub angle: f32,
    pub translation: Vec2,
    pub forward: Vec2,
    pub speed: f32,
    pub is_shooting: bool,
}

impl Player {
    pub fn new(pos: Vec2Wrap, scale: Vec2, angle: f32, forward: Vec2, speed: f32) -> Self {
        Player {
            pos,
            scale,
            angle,
            translation: Vec2::ZERO,
            forward,
            speed,
            is_shooting: false,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult {
        let draw_params = DrawParam::default();
            // .dest(self.pos)
            // .scale(self.scale)
            // .rotation(self.angle - PI / 2.);
        let mut verts = assets._BOX_VERTS.clone();
        for v in verts.iter_mut() {
            v.pos.0 *= 100.;
        }

        let mesh = Mesh::from_raw(
            ctx,
            &verts,
            &assets._BOX_INDICES,
            Some(assets.player_base.clone())
        )?;

        graphics::draw(ctx, &mesh, draw_params)?;
        Ok(())
    }
}

impl Model for Player {
    fn bounding_box(&self) -> graphics::Rect {
        graphics::Rect::new(self.pos.0.x, self.pos.0.y, 15., 15.)
    }

    fn translate(&mut self) {
        self.pos.0 += self.speed * self.translation;
    }

    fn rotate(&mut self, angle: f32) {
        self.angle = angle;
        self.forward = Mat3::from_angle(angle).transform_vector2(Vec2::new(1., 0.));
    }

    fn scale(&mut self, scale: Vec2) {
        self.scale = scale;
    }
}
