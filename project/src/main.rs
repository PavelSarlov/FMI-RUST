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

use project::camera::Camera;

#[derive(Clone, Debug)]
pub struct VertexWrap (Vec3);

impl Into<Vertex> for VertexWrap {
    fn into(self) -> Vertex {
        Vertex {
            pos: [self.0.x, self.0.y],
            uv: [1.0, 1.0],
            color: Color::WHITE.into(),
        }
    }
}

impl Into<Point2<f32>> for VertexWrap {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.0.x as f32,
            y: self.0.y as f32,
        }
    }
}

impl Into<Vec3> for VertexWrap {
    fn into(self) -> Vec3 {
        self.0
    }
}

impl From<Vec3> for VertexWrap {
    fn from(vec: Vec3) -> Self {
        VertexWrap(vec)
    }
}

struct MainState {
    screen_width: f32,
    screen_height: f32,
}

impl MainState {
    fn new(_ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;

        let s = MainState {
            screen_width,
            screen_height,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // if input::keyboard::is_key_pressed(ctx, KeyCode.A) {

        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let cam = Camera::new(
            Vec3::new(0.0,0.0,1.0),
            Vec3::new(0.0,1.0,0.0),
            Vec3::new(0.0,0.0,0.0),
        );

        let fov_rad = 3.14159 * 0.5;
        let aspect_ratio = self.screen_height / self.screen_width;
        let far = 0.1;
        let near = 1000.0;

        let verts = &mut [VertexWrap(Vec3::new(-0.5, -0.5, -0.5)),
                         VertexWrap(Vec3::new(-0.5, 0.5, -0.5)),
                         VertexWrap(Vec3::new(0.5, 0.5, -0.5)),
                         VertexWrap(Vec3::new(0.5, -0.5, -0.5)),
                         VertexWrap(Vec3::new(0.5, -0.5, 0.5)),
                         VertexWrap(Vec3::new(0.5, 0.5, 0.5)),
                         VertexWrap(Vec3::new(-0.5, 0.5, 0.5)),
                         VertexWrap(Vec3::new(-0.5, -0.5, 0.5))]; 
        let indices: &[u32] = &[0,1,2,2,3,0,4,5,6,5,7,4,1,6,5,5,2,1,7,0,3,3,4,7,3,2,5,5,4,3,7,6,1,1,0,7];
        let texture = Image::new(ctx, "/ferris.png")?;

        let time_now = (time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_millis() % 1000) as f32;
        let rot_x = 3.14159 * time_now / 1000.0;
        let rot_y = 3.14159 * time_now / 1000.0;
        let rot_z = 3.14159 * time_now / 1000.0;

        let mat_world = Mat4::from_scale_rotation_translation(
            Vec3::new(0.5,0.5,0.5),
            Quat::from_rotation_y(rot_y),
            Vec3::new(0.0,0.0,1.0),
        );
        let mat_view = Mat4::look_at_lh(cam.forward, cam.center, cam.up);
        // let mat_proj = Mat4::orthographic_lh(
        //         0.0,800.0/400.0,600.0/400.0,0.0,
        //         -4.0,4.0
        //     );
        let mat_proj = Mat4::perspective_lh(
            fov_rad,
            aspect_ratio,
            near,
            far
        );

        for x in verts.iter_mut() {
            *x = VertexWrap(mat_world.transform_point3(x.0));
            *x = VertexWrap(mat_view.transform_point3(x.0));
            *x = VertexWrap(mat_proj.transform_point3(x.0));
            *x = VertexWrap(x.0 + Vec3::new(1.0, 1.0, 0.0));
            *x = VertexWrap(x.0 * Vec3::new(0.5*self.screen_width, 0.5*self.screen_height, 0.0));
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
            None,
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
