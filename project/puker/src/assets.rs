use ggez::{
    graphics::{Image},
    Context,
    GameResult,
};

#[derive(Clone,Debug)]
pub struct Assets {
    pub player_base: Image,
    pub player_shoot: Image,
    pub shot_base: Image,
    pub enemy_mask_base: Image,
    pub room_base: Image,
    pub door_north: Image,
    pub door_south: Image,
    pub door_east: Image,
    pub door_west: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let player_base = Image::new(ctx, "/player_base.png")?;
        let player_shoot = Image::new(ctx, "/player_shoot.png")?;
        let shot_base = Image::new(ctx, "/shot_base.png")?;
        let enemy_mask_base = Image::new(ctx, "/enemy_mask_base.png")?;
        let room_base = Image::new(ctx, "/room_base.png")?;
        let door_north = Image::new(ctx, "/door_north.png")?;
        let door_east = Image::new(ctx, "/door_east.png")?;
        let door_south = Image::new(ctx, "/door_south.png")?;
        let door_west = Image::new(ctx, "/door_west.png")?;

        Ok(Self {
            player_base,
            player_shoot,
            shot_base,
            enemy_mask_base,
            room_base,
            door_north,
            door_south,
            door_east,
            door_west,
        })
    }
}

// pub trait Sprite: Debug {
//     fn draw(&mut self, center: Point2<f32>, ctx: &mut Context) -> GameResult<()>;
//     fn width(&self, ctx: &mut Context) -> f32;
//     fn height(&self, ctx: &mut Context) -> f32;
// }

// impl Sprite for TextSprite {
//     fn draw(&mut self, top_left: Point2<f32>, ctx: &mut Context) -> GameResult<()> {
//         graphics::draw(ctx, &self.text, graphics::DrawParam::default().dest(top_left))
//     }

//     fn width(&self, ctx: &mut Context) -> f32 { self.text.width(ctx) }
//     fn height(&self, ctx: &mut Context) -> f32 { self.text.height(ctx) }
// }
