use ggez::{
    graphics::{self, DrawParam, Rect, DrawMode, Color, Mesh},
    GameResult,
    Context,
};
use crate::{
    utils::*,
    assets::*,
    consts::*,
};
use glam::f32::{Vec2};
use std::{
    f32::consts::PI,
    collections::{VecDeque},
};

pub trait Model {
    fn bbox(&self, sw: f32, sh: f32) -> graphics::Rect;

    fn update(&mut self, _delta_time: f32) -> GameResult;

    fn draw(&self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32)) -> GameResult;

    fn draw_bbox(&self, ctx: &mut Context, world_coords: (f32, f32)) -> GameResult;

    fn scale_to_screen(&self, sw: f32, sh: f32, image: Rect) -> Vec2;
}

#[derive(Clone, Debug)]
pub enum ActorState {
    BASE,
    SHOOT,
}

#[derive(Clone, Debug)]
pub enum ActorTag {
    PLAYER,
    ENEMY,
    FRIENDLY,
}

pub enum PlayerFacing {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(Clone, Debug)]
pub struct Actor {
    pub pos: Vec2Wrap,
    pub scale: Vec2,
    pub angle: f32,
    pub translation: Vec2,
    pub forward: Vec2,
    pub speed: f32,
    pub state: ActorState,
    pub health: f32,
    pub tag: ActorTag,
    pub bbox: f32,
    pub shoot_rate: f32,
    pub shoot_range: f32,
    pub shoot_timeout: f32,
    pub shots: Vec<Shot>,
}

impl Model for Actor {
    fn bbox(&self, sw: f32, sh: f32) -> Rect {
        let width = sw * self.bbox;
        let height = sh * self.bbox;
        Rect::new(self.pos.0.x - width / 2., self.pos.0.y + height / 2., width, height)
    }

    fn update(&mut self, _delta_time: f32) -> GameResult {
        self.pos.0 += self.translation * self.speed;
        self.shoot_timeout = f32::max(0., self.shoot_timeout - _delta_time);
        self.forward = Vec2::new(self.angle.sin(), self.angle.cos());

        let mut shots_gone = VecDeque::<usize>::new();

        for (i, shot) in self.shots.iter_mut().enumerate() {
            shot.update(_delta_time)?;
            if shot.pos.0.distance(shot.spawn_pos.0) > self.shoot_range { shots_gone.push_back(i); } 
        }

        for shot in shots_gone {
            self.shots.remove(shot);
        }

        match self.state {
            ActorState::SHOOT => {
                if self.shoot_timeout == 0. {
                    self.shoot()?;
                }
            },
            _ => (),
        }

        Ok(())
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32)) -> GameResult {
        let (sw, sh) = world_coords;
        let pos: Vec2Wrap = world_to_screen_space(sw, sh, self.pos.into()).into();
        let draw_params = DrawParam::default()
            .dest(pos)
            .scale(self.scale * self.scale_to_screen(sw, sh, assets.player_base.dimensions()))
            .rotation(self.angle)
            .offset([0.5, 0.5]);

        match self.state {
            ActorState::BASE => graphics::draw(ctx, &assets.player_base, draw_params)?,
            ActorState::SHOOT => graphics::draw(ctx, &assets.player_shoot, draw_params)?
        }

        self.draw_bbox(ctx, world_coords)?;

        for shot in self.shots.iter() {
            shot.draw(ctx, assets, world_coords)?;
        }

        Ok(())
    }

    fn draw_bbox(&self, ctx: &mut Context, world_coords: (f32, f32)) -> GameResult {
        let (sw, sh) = world_coords;
        let mut bbox = self.bbox(sw, sh);
        let screen_coords = world_to_screen_space(sw, sh, Vec2::new(bbox.x, bbox.y));
        bbox.x = screen_coords.x;
        bbox.y = screen_coords.y;

        let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2.0), bbox, Color::BLUE)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;

        Ok(())
    }

    fn scale_to_screen(&self, sw: f32, sh: f32, image: Rect) -> Vec2 {
        // Vec2::new(sw / image.w * 0.05, sh / image.h * 0.05)
        Vec2::new(sw * self.bbox / image.w, sh * self.bbox / image.h)
    }
}

impl Actor {
    pub fn shoot(&mut self) -> GameResult {
        self.shoot_timeout = 1. / self.shoot_rate;

        let shot = Shot {
            pos: self.pos,
            spawn_pos: self.pos,
            velocity: self.forward,
            bbox: SHOT_BBOX,
            speed: SHOT_SPEED,
            scale: Vec2::ONE,
        };

        self.shots.push(shot);

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Shot {
    pub pos: Vec2Wrap,
    pub spawn_pos: Vec2Wrap,
    pub velocity: Vec2,
    pub bbox: f32,
    pub speed: f32,
    pub scale: Vec2,
}

impl Model for Shot {
    fn bbox(&self, sw: f32, sh: f32) -> Rect {
        let width = sw * self.bbox;
        let height = sh * self.bbox;
        Rect::new(self.pos.0.x - width / 2., self.pos.0.y + height / 2., width, height)
    }

    fn update(&mut self, _delta_time: f32) -> GameResult {
        self.pos.0 += self.velocity * self.speed;

        Ok(())
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32)) -> GameResult {
        let (sw, sh) = world_coords;
        let pos: Vec2Wrap = world_to_screen_space(sw, sh, self.pos.into()).into();
        let draw_params = DrawParam::default()
            .dest(pos)
            .scale(self.scale * self.scale_to_screen(sw, sh, assets.player_base.dimensions()))
            .offset([0.5, 0.5]);

        graphics::draw(ctx, &assets.shot_base, draw_params)?;
        self.draw_bbox(ctx, world_coords)?;

        Ok(())
    }

    fn draw_bbox(&self, ctx: &mut Context, world_coords: (f32, f32)) -> GameResult {
        let (sw, sh) = world_coords;
        let mut bbox = self.bbox(sw, sh);
        let screen_coords = world_to_screen_space(sw, sh, Vec2::new(bbox.x, bbox.y));
        bbox.x = screen_coords.x;
        bbox.y = screen_coords.y;

        let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2.0), bbox, Color::BLUE)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;

        Ok(())
    }

    fn scale_to_screen(&self, sw: f32, sh: f32, image: Rect) -> Vec2 {
        // Vec2::new(sw / image.w * 0.05, sh / image.h * 0.05)
        Vec2::new(sw * self.bbox / image.w, sh * self.bbox / image.h)
    }
}
