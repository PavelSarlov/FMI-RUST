use glam::f32::*;
use ggez::{
    mint::Point2
};

#[derive(Clone, Copy, Debug)]
pub struct Vec3Wrap(pub Vec3); 

impl Into<Point2<f32>> for Vec3Wrap {
    fn into(self) -> Point2<f32> {
        Point2::<f32> {
            x: self.0.x,
            y: self.0.y,
        }
    }
}

impl Into<Vec3> for Vec3Wrap {
    fn into(self) -> Vec3 {
        self.0
    }
}

impl From<Vec3> for Vec3Wrap {
    fn from(v: Vec3) -> Self {
        Vec3Wrap(v)
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub verts: [Vec3Wrap; 3],
}

impl Triangle {
    pub fn new(verts: &[Vec3Wrap; 3]) -> Self {
        Triangle {
            verts: verts.clone(),
        }
    }
}
