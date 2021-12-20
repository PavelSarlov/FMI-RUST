use glam::*;
use ggez::{
    graphics::{self,Color,Vertex},
    mint::{Point2},
};

#[derive(Clone, Debug)]
pub struct VertexWrap {
    pub position: Vec3,
    pub texcoord: Vec2,
}

impl VertexWrap {
    pub fn new(position: Vec3, texcoord: Vec2) -> Self {
        VertexWrap {
            position,
            texcoord,
        }
    }
}

impl Into<Vertex> for VertexWrap {
    fn into(self) -> Vertex {
        Vertex {
            pos: [self.position.x, self.position.y],
            uv: [self.texcoord.x, self.texcoord.y],
            color: Color::WHITE.into(),
        }
    }
}

impl Into<Point2<f32>> for VertexWrap {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.position.x as f32,
            y: self.position.y as f32,
        }
    }
}

impl Into<Vec3> for VertexWrap {
    fn into(self) -> Vec3 {
        self.position
    }
}
