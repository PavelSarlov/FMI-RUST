use glam::f32::*;
use ggez::{
    mint::Point2,
    graphics::Color,
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
    pub color: Color,
}

impl Triangle {
    pub fn new(verts: &[Vec3Wrap; 3]) -> Self {
        Triangle {
            verts: verts.clone(),
            color: Color::new(1.0,1.0,1.0,1.0),
        }
    }
}

pub fn perspective_lh(fov_rad: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    Mat4::from_cols(
        Vec4::X * (aspect_ratio * fov_rad),
        Vec4::Y * fov_rad,
        Vec4::new(0.0, 0.0, (far / (far - near)), 1.0),
        Vec4::new(0.0, 0.0, (-far * near) / (far - near), 0.0),
    )
}

