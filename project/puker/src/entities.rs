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
    collections::{VecDeque},
};

pub trait Model: std::fmt::Debug {
    fn bbox(&self, sw: f32, sh: f32) -> graphics::Rect;

    fn update(&mut self, _delta_time: f32) -> GameResult;

    fn draw(&self, ctx: &mut Context, assets: &Assets, screen: (f32, f32)) -> GameResult;

    fn draw_bbox(&self, ctx: &mut Context, screen: (f32, f32)) -> GameResult {
        let (sw, sh) = screen;
        let mut bbox = self.bbox(sw, sh);
        let screen_coords = world_to_screen_space(sw, sh, Vec2::new(bbox.x, bbox.y));
        bbox.x = screen_coords.x;
        bbox.y = screen_coords.y;

        let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2.0), bbox, Color::BLUE)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;

        Ok(())
    }

    fn scale_to_screen(&self, sw: f32, sh: f32, image: Rect) -> Vec2;
}

pub trait Shooter {
    fn shoot(&mut self) -> GameResult;
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
}

#[derive(Clone, Debug, Copy)]
pub struct ActorProps {
    pub pos: Vec2Wrap,
    pub scale: Vec2,
    pub translation: Vec2,
    pub forward: Vec2,
    pub bbox: f32,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub props: ActorProps,
    pub speed: f32,
    pub state: ActorState,
    pub health: f32,
    pub shoot_rate: f32,
    pub shoot_range: f32,
    pub shoot_timeout: f32,
    pub shots: Vec<Shot>,
}

impl Model for Player {
    fn bbox(&self, sw: f32, sh: f32) -> Rect {
        let width = sw * self.props.bbox;
        let height = sh * self.props.bbox;
        Rect::new(self.props.pos.0.x - width / 2., self.props.pos.0.y + height / 2., width, height)
    }

    fn update(&mut self, _delta_time: f32) -> GameResult {
        self.props.pos.0 += self.props.translation * self.speed;
        self.shoot_timeout = f32::max(0., self.shoot_timeout - _delta_time);

        let mut shots_gone = VecDeque::<usize>::new();

        for (i, shot) in self.shots.iter_mut().enumerate() {
            shot.update(_delta_time)?;
            if shot.props.pos.0.distance(shot.spawn_pos.0) > self.shoot_range { shots_gone.push_back(i); } 
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

    fn draw(&self, ctx: &mut Context, assets: &Assets, screen: (f32, f32)) -> GameResult {
        let (sw, sh) = screen;
        let pos: Vec2Wrap = world_to_screen_space(sw, sh, self.props.pos.into()).into();
        let draw_params = DrawParam::default()
            .dest(pos)
            .scale(self.props.scale * self.scale_to_screen(sw, sh, assets.player_base.dimensions()))
            .offset([0.5, 0.5]);

        for shot in self.shots.iter() {
            shot.draw(ctx, assets, screen)?;
        }

        match self.state {
            ActorState::BASE => graphics::draw(ctx, &assets.player_base, draw_params)?,
            ActorState::SHOOT => {
                match self.props.forward.to_array() {
                    [ 1., 0.] => graphics::draw(ctx, &assets.player_shoot_east, draw_params)?,
                    [-1., 0.] => graphics::draw(ctx, &assets.player_shoot_west, draw_params)?,
                    [0.,  1.] => graphics::draw(ctx, &assets.player_shoot_north, draw_params)?,
                    [0., -1.] => graphics::draw(ctx, &assets.player_shoot_south, draw_params)?,
                    _ => graphics::draw(ctx, &assets.player_base, draw_params)?,
                }
            },
            _ => ()
        }

        self.draw_bbox(ctx, screen)?;

        Ok(())
    }

    fn scale_to_screen(&self, sw: f32, sh: f32, image: Rect) -> Vec2 {
        // Vec2::new(sw / image.w * 0.05, sh / image.h * 0.05)
        Vec2::new(sw * self.props.bbox / image.w, sh * self.props.bbox / image.h)
    }
}

impl Shooter for Player {
    fn shoot(&mut self) -> GameResult {
        self.shoot_timeout = 1. / self.shoot_rate;
        let shot_dir = (self.props.forward + 0.5 * (self.props.translation * Vec2::new(self.props.forward.y, self.props.forward.x).abs())).normalize();

        let shot = Shot {
            props: ActorProps {
                pos: self.props.pos,
                forward: shot_dir,
                translation: shot_dir,
                bbox: SHOT_BBOX,
                scale: Vec2::ONE,
            },
            spawn_pos: self.props.pos,
            speed: SHOT_SPEED,
        };

        self.shots.push(shot);

        Ok(())
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Shot {
    pub props: ActorProps,
    pub speed: f32,
    pub spawn_pos: Vec2Wrap,
}

impl Model for Shot {
    fn bbox(&self, sw: f32, sh: f32) -> Rect {
        let width = sw * self.props.bbox;
        let height = sh * self.props.bbox;
        Rect::new(self.props.pos.0.x - width / 2., self.props.pos.0.y + height / 2., width, height)
    }

    fn update(&mut self, _delta_time: f32) -> GameResult {
        self.props.pos.0 += self.props.forward * self.speed;

        Ok(())
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets, screen: (f32, f32)) -> GameResult {
        let (sw, sh) = screen;
        let pos: Vec2Wrap = world_to_screen_space(sw, sh, self.props.pos.into()).into();
        let draw_params = DrawParam::default()
            .dest(pos)
            .scale(self.props.scale * self.scale_to_screen(sw, sh, assets.player_base.dimensions()))
            .offset([0.5, 0.5]);

        graphics::draw(ctx, &assets.shot_base, draw_params)?;
        self.draw_bbox(ctx, screen)?;

        Ok(())
    }

    fn scale_to_screen(&self, sw: f32, sh: f32, image: Rect) -> Vec2 {
        // Vec2::new(sw / image.w * 0.05, sh / image.h * 0.05)
        Vec2::new(sw * self.props.bbox / image.w, sh * self.props.bbox / image.h)
    }
}

#[derive(Clone, Debug)]
pub struct EnemyMask {
    pub props: ActorProps,
    pub speed: f32,
    pub state: ActorState,
    pub health: f32,
    pub shoot_rate: f32,
    pub shoot_range: f32,
    pub shoot_timeout: f32,
    pub shots: Vec<Shot>,
}

impl Model for EnemyMask {
    fn bbox(&self, sw: f32, sh: f32) -> Rect {
        let width = sw * self.props.bbox;
        let height = sh * self.props.bbox;
        Rect::new(self.props.pos.0.x - width / 2., self.props.pos.0.y + height / 2., width, height)
    }

    fn update(&mut self, _delta_time: f32) -> GameResult {
        self.props.pos.0 += self.props.translation * self.speed;
        self.shoot_timeout = f32::max(0., self.shoot_timeout - _delta_time);

        let mut shots_gone = VecDeque::<usize>::new();

        for (i, shot) in self.shots.iter_mut().enumerate() {
            shot.update(_delta_time)?;
            if shot.props.pos.0.distance(shot.spawn_pos.0) > self.shoot_range { shots_gone.push_back(i); } 
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

    fn draw(&self, ctx: &mut Context, assets: &Assets, screen: (f32, f32)) -> GameResult {
        let (sw, sh) = screen;
        let pos: Vec2Wrap = world_to_screen_space(sw, sh, self.props.pos.into()).into();
        let draw_params = DrawParam::default()
            .dest(pos)
            .scale(self.props.scale * self.scale_to_screen(sw, sh, assets.player_base.dimensions()))
            .offset([0.5, 0.5]);

        match self.state {
            ActorState::BASE => graphics::draw(ctx, &assets.enemy_mask_base, draw_params)?,
            _ => ()
        }

        self.draw_bbox(ctx, screen)?;

        for shot in self.shots.iter() {
            shot.draw(ctx, assets, screen)?;
        }

        Ok(())
    }

    fn scale_to_screen(&self, sw: f32, sh: f32, image: Rect) -> Vec2 {
        // Vec2::new(sw / image.w * 0.05, sh / image.h * 0.05)
        Vec2::new(sw * self.props.bbox / image.w, sh * self.props.bbox / image.h)
    }
}

impl Shooter for EnemyMask {
    fn shoot(&mut self) -> GameResult {
        self.shoot_timeout = 1. / self.shoot_rate;
        let shot_dir = (self.props.forward + 0.5 * (self.props.translation * Vec2::new(self.props.forward.y, self.props.forward.x).abs())).normalize();

        let shot = Shot {
            props: ActorProps {
                pos: self.props.pos,
                forward: shot_dir,
                translation: shot_dir,
                bbox: SHOT_BBOX,
                scale: Vec2::ONE,
            },
            spawn_pos: self.props.pos,
            speed: SHOT_SPEED,
        };

        self.shots.push(shot);

        Ok(())
    }
}
