use ggez::{
    event,
    graphics::{self, Color, DrawMode, Mesh, DrawParam, Rect, Vertex},
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

    fn update(&mut self);

    fn draw(&self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32)) -> GameResult;
}

#[derive(Clone, Debug)]
pub enum PlayerState {
    BASE,
    SHOOT,
}

#[derive(Clone, Debug)]
pub struct Player {
    pos: Vec2Wrap,
    pub scale: Vec2,
    pub angle: f32,
    pub translation: Vec2,
    pub forward: Vec2,
    pub cur_speed: f32,
    pub top_speed: f32,
    pub state: PlayerState,
    pub health: f32,
}

impl Player {
    pub fn new(pos: Vec2Wrap, scale: Vec2, angle: f32, translation: Vec2, forward: Vec2, top_speed: f32) -> Self {
        Player {
            pos,
            scale,
            angle,
            translation,
            forward,
            cur_speed: 0.,
            top_speed,
            state: PlayerState::BASE,
            health: 5.,
        }
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos.0
    }
}

impl Model for Player {
    fn bounding_box(&self) -> graphics::Rect {
        graphics::Rect::new(self.pos.0.x, self.pos.0.y, 15., 15.)
    }

    fn update(&mut self) {
        self.pos.0 += self.translation * self.top_speed;
        // self.pos.0 += self.translation * self.cur_speed;
        // self.cur_speed = f32::max(f32::min(Vec2::dot(self.translation, Vec2::ONE) * 0.1 + self.cur_speed, self.top_speed), 0.);
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32)) -> GameResult {
        let (sw, sh) = world_coords;
        let pos: Vec2Wrap = world_to_screen_space(sw, sh, self.pos.into()).into();
        let draw_params = DrawParam::default()
            .dest(pos)
            .scale(self.scale)
            .rotation(self.angle)
            .offset([0.5, 0.5]);

        match self.state {
            PlayerState::BASE => graphics::draw(ctx, &assets.player_base, draw_params)?,
            PlayerState::SHOOT => graphics::draw(ctx, &assets.player_shoot, draw_params)?
        }

        Ok(())
    }
}

pub struct Shot {
    pos: Vec2Wrap,
    velocity: Vec2,
}
