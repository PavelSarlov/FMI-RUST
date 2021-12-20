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
};
use glam::*;

use std::{
    env,
    path,
    time,
    thread,
};

use project::{
    camera::Camera,
    vertex_wrap::VertexWrap,
};

struct MainState {
    screen_width: f32,
    screen_height: f32,
    fov_rad: f32,
    aspect_ratio: f32,
    far: f32,
    near: f32,
    camera: Camera,
}

impl MainState {
    fn new(_ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let fov_rad = 3.14159 * 0.5;
        let aspect_ratio = screen_height / screen_width;
        let far = 0.1;
        let near = 1000.0;
        let camera = Camera::new(Vec3::new(0.0,0.0,1.0), Vec3::new(0.0,1.0,0.0), Vec3::new(0.0,0.0,0.0));

        let s = MainState {
            screen_width,
            screen_height,
            fov_rad,
            aspect_ratio,
            far,
            near,
            camera,
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

        let texcoord_list = &[
            Vec2::new(0.0,0.0),
            Vec2::new(0.0,1.0),
            Vec2::new(1.0,0.0),
            Vec2::new(1.0,1.0),
        ];

        let vertex_list = &[Vec3::new(-0.5, -0.5, -0.5),
                         Vec3::new(-0.5, 0.5, -0.5),
                         Vec3::new(0.5, 0.5, -0.5),
                         Vec3::new(0.5, -0.5, -0.5),
                         Vec3::new(0.5, -0.5, 0.5),
                         Vec3::new(0.5, 0.5, 0.5),
                         Vec3::new(-0.5, 0.5, 0.5),
                         Vec3::new(-0.5, -0.5, 0.5)]; 

        let verts = &mut [
            VertexWrap::new(vertex_list[0], texcoord_list[1]),
            VertexWrap::new(vertex_list[1], texcoord_list[0]),
            VertexWrap::new(vertex_list[2], texcoord_list[2]),
            VertexWrap::new(vertex_list[3], texcoord_list[3]),

            VertexWrap::new(vertex_list[4], texcoord_list[1]),
            VertexWrap::new(vertex_list[5], texcoord_list[0]),
            VertexWrap::new(vertex_list[6], texcoord_list[2]),
            VertexWrap::new(vertex_list[7], texcoord_list[3]),

            VertexWrap::new(vertex_list[1], texcoord_list[1]),
            VertexWrap::new(vertex_list[6], texcoord_list[0]),
            VertexWrap::new(vertex_list[5], texcoord_list[2]),
            VertexWrap::new(vertex_list[2], texcoord_list[3]),

            VertexWrap::new(vertex_list[7], texcoord_list[1]),
            VertexWrap::new(vertex_list[0], texcoord_list[0]),
            VertexWrap::new(vertex_list[3], texcoord_list[2]),
            VertexWrap::new(vertex_list[4], texcoord_list[3]),

            VertexWrap::new(vertex_list[3], texcoord_list[1]),
            VertexWrap::new(vertex_list[2], texcoord_list[0]),
            VertexWrap::new(vertex_list[5], texcoord_list[2]),
            VertexWrap::new(vertex_list[4], texcoord_list[3]),

            VertexWrap::new(vertex_list[7], texcoord_list[1]),
            VertexWrap::new(vertex_list[6], texcoord_list[0]),
            VertexWrap::new(vertex_list[1], texcoord_list[2]),
            VertexWrap::new(vertex_list[0], texcoord_list[3]),
        ];
        let indices: &[u32] = &[
            // front
            0,1,2,
            2,3,0,

            // back
            4,5,6,
            5,7,4,

            // top
            8,9,10,
            10,11,8,

            // bottom
            12,13,14,
            14,15,12,

            // right
            16,17,18,
            18,19,16,

            // left
            20,21,22,
            22,23,20];
        let texture = Image::new(ctx, "/crate.png")?;

        let time_now = (time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_millis() % 1000) as f32;
        let rot_x = 3.14159 * time_now / 1000.0;
        let rot_y = 3.14159 * time_now / 1000.0;
        let rot_z = 3.14159 * time_now / 1000.0;

        let mat_world = Mat4::from_scale_rotation_translation(
            Vec3::new(0.5,0.5,0.5),
            Quat::from_rotation_y(rot_y),
            Vec3::new(0.0,0.0,1.0),
        );
        let mat_view = Mat4::look_at_lh(
            self.camera.forward,
            self.camera.center,
            self.camera.up);
        // let mat_proj = Mat4::orthographic_lh(
        //         0.0,800.0/400.0,600.0/400.0,0.0,
        //         -4.0,4.0
        //     );
        let mat_proj = Mat4::perspective_lh(
            self.fov_rad,
            self.aspect_ratio,
            self.near,
            self.far
        );

        for x in verts.iter_mut() {
            x.position = mat_world.transform_point3(x.position);
            x.position = mat_view.transform_point3(x.position);
            x.position = mat_proj.transform_point3(x.position);
            x.position = x.position + Vec3::new(1.0, 1.0, 0.0);
            x.position = x.position * Vec3::new(0.5*self.screen_width, 0.5*self.screen_height, 0.0);
        };

        // println!("{:?}", &verts);
        // std::thread::sleep(std::time::Duration::from_millis(1000));

        // for x in (0..(indices.len()-1)).step_by(3) {
        //     let tri = Mesh::new_polygon(
        //         ctx,
        //         graphics::DrawMode::stroke(1.0),
        //         &[verts[indices[x] as usize].clone(), verts[indices[x+1]].clone(), verts[indices[x+2]].clone()],
        //         Color::WHITE,
        //     )?;
        //     graphics::draw(ctx, &tri, graphics::DrawParam::default())?;
        // }

        let cube = Mesh::from_raw(
            ctx,
            verts,
            indices,
            Some(texture),
        )?;

        graphics::draw(ctx, &cube, ggez::graphics::DrawParam::default())?;

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
