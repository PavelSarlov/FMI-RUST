use glam::*;
use ggez::{};

pub struct Camera {
    pub forward: Vec3,
    pub up: Vec3,
    pub center: Vec3,
}

impl Camera {
    pub fn new(forward: Vec3, up: Vec3, center: Vec3) -> Self {
        Camera {
            forward: forward.clone(),
            up: up.clone(),
            center: center.clone()
        }
    }

//     pub fn set_forward(&mut self, forward: Vec3) {
//         self.forward = forward.clone();
//     }

//     pub fn set_up(&mut self, up: Vec3) {
//         self.up = up.clone();
//     }

//     pub fn set_center(&mut self, center: Vec3) {
//         self.center = center.clone();
//     }
}
