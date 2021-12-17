use ggez::event;
use ggez::graphics::{self, Color, Vertex};
use ggez::{
    Context, 
    ContextBuilder,
    GameResult,
    conf::{Conf, WindowMode},
};
use glam::*;

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
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 0.1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let list = &mut [VertexWrap(Vec3::new(-0.5, -0.5, -0.5)),
                   VertexWrap(Vec3::new(-0.5, 0.5, -0.5)),
                   VertexWrap(Vec3::new(0.5, 0.5, -0.5)),
                   VertexWrap(Vec3::new(0.5, -0.5, -0.5)),
                   VertexWrap(Vec3::new(0.5, -0.5, 0.5)),
                   VertexWrap(Vec3::new(0.5, 0.5, 0.5)),
                   VertexWrap(Vec3::new(-0.5, 0.5, 0.5)),
                   VertexWrap(Vec3::new(-0.5, -0.5, 0.5))]; 
        let indices: &[u32] = &[0,1,2,2,3,0,4,5,6,5,7,4,1,6,5,5,2,1,7,0,3,3,4,7,3,2,5,5,4,3,7,6,1,1,0,7];

        let mat_world = Mat4::from_scale_rotation_translation(
            Vec3::new(0.0,0.0,0.0),
            Quat::IDENTITY,
            Vec3::new(0.0,0.0,0.0),
            );
        let mat_view = Mat4::IDENTITY;
        // let mat_proj = Mat4::orthographic_lh(
        //         0.0,800.0/400.0,600.0/400.0,0.0,
        //         -4.0,4.0
        //     );
        let mat_proj = Mat4::perspective_lh(
                3.14159*0.5,
                600.0/800.0,
                -4.0, 4.0
            );

        for x in list.iter_mut() {
           *x = VertexWrap(mat_world.transform_point3(x.0));
           *x = VertexWrap(mat_view.transform_point3(x.0));
           *x = VertexWrap(mat_proj.transform_point3(x.0));
           // *x = VertexWrap(x.0 + Vec3::new(1.0, 1.0, 0.0));
           *x = VertexWrap(x.0 * Vec3::new(0.5*800.0, 0.5*600.0, 0.0));
        };

        println!("{:?}", list);
        // std::thread::sleep(std::time::Duration::from_millis(1000));

        let cube = graphics::Mesh::from_raw(
                ctx,
                list,
                indices,
                None
            )?;

        graphics::draw(ctx, &cube, (Vec2::new(0.0, 380.0),))?;

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

    let (ctx, event_loop) = ContextBuilder::new("test", "Window")
        .default_conf(conf.clone())
        .build()
        .unwrap();

    let state = MainState::new()?;

    event::run(ctx, event_loop, state)
}
