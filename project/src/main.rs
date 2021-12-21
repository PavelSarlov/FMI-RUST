#![allow(unused_imports)]

use ggez::{
    graphics::{self,Color,Vertex,Image,Mesh},
    event::{self, Axis, Button, KeyCode, KeyMods, MouseButton},
    Context, 
    ContextBuilder,
    GameResult,
    conf::{Conf, WindowMode},
    filesystem,
    mint::{Point2},
    input,
    timer,
};
use glam::f32::*;

use std::{
    env,
    path,
    time,
    thread,
};

use project::{
    camera::Camera,
    triangle::*,
};

struct MainState {
    screen_width: f32,
    screen_height: f32,
    fov_rad: f32,
    aspect_ratio: f32,
    far: f32,
    near: f32,
    camera: Camera,
    theta: f32,
}

impl MainState {
    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let fov_rad = 3.14159 * 0.5;
        let aspect_ratio = screen_height / screen_width;
        let far = 0.1;
        let near = 1000.0;
        let camera = Camera::new(Vec3::new(0.0,0.0,1.0), Vec3::new(0.0,1.0,0.0), Vec3::new(0.0,0.0,-1.0));

        let s = MainState {
            screen_width,
            screen_height,
            fov_rad,
            aspect_ratio,
            far,
            near,
            camera,
            theta: 0.0,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.camera.rotate_y(0.1);
        }
        else if input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.camera.rotate_y(-0.1);
        }

        if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.camera.translate_right(0.1);
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.camera.translate_right(-0.1);
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.camera.translate_forward(-0.1);
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.camera.translate_forward(0.1);
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.camera.translate_up(0.1);
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::LShift) {
            self.camera.translate_up(-0.1);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // println!("{:?}", &verts);
        // std::thread::sleep(std::time::Duration::from_millis(1000));

        self.theta += 0.001 * timer::delta(ctx).as_millis() as f32;

        let tris = &mut [
            // south
            Triangle::new(&[Vec3Wrap(Vec3::new(0.0, 0.0, 0.0)), Vec3Wrap(Vec3::new(0.0, 1.0, 0.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 0.0))]),
            Triangle::new(&[Vec3Wrap(Vec3::new(0.0, 0.0, 0.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 0.0)), Vec3Wrap(Vec3::new(1.0, 0.0, 0.0))]),

            // east
            Triangle::new(&[Vec3Wrap(Vec3::new(1.0, 0.0, 0.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 0.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 1.0))]),
            Triangle::new(&[Vec3Wrap(Vec3::new(1.0, 0.0, 0.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 1.0)), Vec3Wrap(Vec3::new(1.0, 0.0, 1.0))]),

            // north
            Triangle::new(&[Vec3Wrap(Vec3::new(1.0, 0.0, 1.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 1.0)), Vec3Wrap(Vec3::new(0.0, 1.0, 1.0))]),
            Triangle::new(&[Vec3Wrap(Vec3::new(1.0, 0.0, 1.0)), Vec3Wrap(Vec3::new(0.0, 1.0, 1.0)), Vec3Wrap(Vec3::new(0.0, 0.0, 1.0))]),

            // west
            Triangle::new(&[Vec3Wrap(Vec3::new(0.0, 0.0, 1.0)), Vec3Wrap(Vec3::new(0.0, 1.0, 1.0)), Vec3Wrap(Vec3::new(0.0, 1.0, 0.0))]),
            Triangle::new(&[Vec3Wrap(Vec3::new(0.0, 0.0, 1.0)), Vec3Wrap(Vec3::new(0.0, 1.0, 0.0)), Vec3Wrap(Vec3::new(0.0, 0.0, 0.0))]),

            // top
            Triangle::new(&[Vec3Wrap(Vec3::new(0.0, 1.0, 0.0)), Vec3Wrap(Vec3::new(0.0, 1.0, 1.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 1.0))]),
            Triangle::new(&[Vec3Wrap(Vec3::new(0.0, 1.0, 0.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 1.0)), Vec3Wrap(Vec3::new(1.0, 1.0, 0.0))]),

            // bottom
            Triangle::new(&[Vec3Wrap(Vec3::new(0.0, 0.0, 0.0)), Vec3Wrap(Vec3::new(0.0, 0.0, 1.0)), Vec3Wrap(Vec3::new(1.0, 0.0, 1.0))]),
            Triangle::new(&[Vec3Wrap(Vec3::new(0.0, 0.0, 0.0)), Vec3Wrap(Vec3::new(1.0, 0.0, 1.0)), Vec3Wrap(Vec3::new(1.0, 0.0, 0.0))]),
        ];

        let mat_world = Mat4::from_scale_rotation_translation(
            Vec3::new(0.5,0.5,0.5),
            Quat::from_rotation_x(self.theta) * Quat::from_rotation_y(self.theta),
            Vec3::new(0.0,0.0,1.0),
        );
        let mat_view = Mat4::look_at_lh(
            self.camera.forward,
            self.camera.center,
            self.camera.up);
        let mat_proj = Mat4::perspective_lh(
            self.fov_rad,
            self.aspect_ratio,
            self.near,
            self.far
        );

        for tri in tris.iter_mut() {
            for vert in tri.verts.iter_mut() {
                *vert = mat_world.transform_point3((*vert).into()).into();
                *vert = mat_view.transform_point3((*vert).into()).into();
                *vert = mat_proj.transform_point3((*vert).into()).into();
                *vert = (vert.0 + Vec3::new(1.0, 1.0, 0.0)).into();
                *vert = (vert.0 * Vec3::new(0.5*self.screen_width, 0.5*self.screen_height, 0.0)).into();
            }

            let poly = Mesh::new_polygon(
                ctx,
                graphics::DrawMode::stroke(1.0),
                &tri.verts,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &poly, graphics::DrawParam::default())?;
        };

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let conf = Conf::new()
        .window_mode(WindowMode {
            width: 800.0,
            height: 600.0,
            ..Default::default()
        });

    let (mut ctx, event_loop) = ContextBuilder::new("test", "Window")
        .default_conf(conf.clone())
        .build()
        .unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let state = MainState::new(&mut ctx, &conf)?;

    event::run(ctx, event_loop, state)
}
