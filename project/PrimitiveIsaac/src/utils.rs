use ggez::{
    mint::Point2,
    graphics::{Rect, Vertex},
};
use glam::f32::*;

#[derive(Clone, Debug, Copy)]
pub struct Vec2Wrap(pub Vec2);

impl Into<Point2<f32>> for Vec2Wrap {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.0.x,
            y: self.0.y
        }
    }
}

impl From<Vec2> for Vec2Wrap {
    fn from(v: Vec2) -> Self {
        Vec2Wrap(v)
    }
}

impl From<[f32; 2]> for Vec2Wrap {
    fn from(a: [f32; 2]) -> Self {
        Vec2Wrap(Vec2::from_slice(&a))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct MyVertex{
    pub pos: Vec2Wrap,
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

impl Into<Point2<f32>> for MyVertex {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.pos.0.x,
            y: self.pos.0.y,
        }
    }
}

impl Into<Vertex> for MyVertex {
    fn into(self) -> Vertex {
        Vertex {
            pos: self.pos.0.to_array(),
            uv: self.uv,
            color: self.color
        }
    }
}
