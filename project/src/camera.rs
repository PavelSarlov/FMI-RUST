use glam::*;
// use ggez::{};

#[derive(Debug, Clone)]
pub struct Camera {
    pub forward: Vec3,
    pub up: Vec3,
    pub center: Vec3,
    pub right: Vec3,
}

impl Camera {
    pub fn new(forward: Vec3, up: Vec3, center: Vec3) -> Self {
        Camera {
            forward: forward.clone(),
            up: up.clone(),
            center: center.clone(),
            right: up.cross(forward),
        }
    }

    pub fn rotate_x(&mut self, angle_rad: f32) {
        let rot_mat_x = Mat4::from_rotation_x(angle_rad);
        self.forward = rot_mat_x.transform_point3(self.forward);
        self.up = rot_mat_x.transform_point3(self.up);
    }

    pub fn rotate_y(&mut self, angle_rad: f32) {
        let rot_mat_y = Mat4::from_rotation_y(angle_rad);
        self.forward = rot_mat_y.transform_point3(self.forward);
    }

    pub fn translate(&mut self, translation: Vec3) {
        let mat_translate = Mat4::from_translation(translation * self.up + translation * self.forward + translation * self.right);
        self.center = mat_translate.transform_point3(self.center);
        self.up = mat_translate.transform_point3(self.up);
        self.forward = mat_translate.transform_point3(self.forward);
        self.right = mat_translate.transform_point3(self.right);
    }
}

