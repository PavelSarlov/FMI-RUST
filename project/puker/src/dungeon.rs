use ggez::{
    graphics::{self, DrawParam, Rect, MeshBuilder},
    GameResult,
    Context
};
use crate::{
    assets::*,
    entities::*,
    utils::Errors,
    consts::*,
};
use std::{
    collections::{VecDeque},
    fmt::{Formatter, Display},
    f32::consts::PI,
};
use rand::{thread_rng, Rng};
use glam::f32::Vec2;

#[derive(Clone, Copy, Hash, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for Direction {}

#[derive(Debug)]
pub struct Room {
    pub enemies: Vec<Box<dyn Model>>,
    pub grid_num: usize,
    pub doors: [Option<Door>; 4],
}

impl Room {
    pub fn update(&mut self, _delta_time: f32) -> GameResult {
        for enemy in self.enemies.iter_mut() {
            enemy.update(_delta_time)?;
        }

        Ok(())
    }
    
    pub fn draw(&self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32)) -> GameResult {
        let (sw, sh) = world_coords;
        let draw_params = DrawParam::default()
            .dest([sw / 2., sh / 2.])
            .scale(Room::get_room_scale(sw, sh, assets.room_base.dimensions()))
            .offset([0.5, 0.5]);
        
        graphics::draw(ctx, &assets.room_base, draw_params)?;

        // for (i, v) in self.doors.iter().enumerate() {
        //     match i {
        //         0 => { graphics::draw(ctx, &assets.door_base, draw_params)?; },
        //         1 => { graphics::draw(ctx, &assets.door_east, draw_params)?; },
        //         2 => { graphics::draw(ctx, &assets.door_south, draw_params)?; },
        //         _ => { graphics::draw(ctx, &assets.door_west, draw_params)?; },
        //     }
        // }
        //
        for enemy in self.enemies.iter() {
            enemy.draw(ctx, assets, world_coords)?;
        }

        Ok(())
    }

    fn get_room_scale(sw: f32, sh: f32, image: Rect) -> [f32; 2] {
        [sw / image.w, sh / image.h]
    }

    fn generate_room(grid_num: usize, screen: (f32, f32)) -> Room {
        let (sw, sh) = screen;

        let enemies: Vec<Box<dyn Model>> = vec![
            Box::new(EnemyMask {
                props: ActorProps {
                    pos: Vec2::new(220., 200.).into(),
                    scale: Vec2::ONE,
                    translation: Vec2::ZERO,
                    forward: Vec2::ZERO,
                    bbox: ENEMY_BBOX,
                },
                speed: ENEMY_SPEED,
                health: ENEMY_HEALTH,
                state: ActorState::BASE,
                shoot_rate: ENEMY_SHOOT_RATE,
                shoot_range: ENEMY_SHOOT_RANGE,
                shoot_timeout: ENEMY_SHOOT_TIMEOUT,
                shots: Vec::new(),
            }),
        ];
        let doors = [None; 4];

        Room {
            enemies,
            grid_num,
            doors,
        }
    }
}

#[derive(Debug)]
pub struct Dungeon {
    grid: [usize; GRID_ROWS * GRID_COLS],
    rooms: Vec<Room>,
}

impl Dungeon {
    pub fn generate_dungeon(screen: (f32, f32)) -> Self {
        let level = 1;
        let room_count = thread_rng().gen_range(0..2) + 5 + level * 2;
        let mut grid = [0; GRID_ROWS * GRID_COLS];
        let mut rooms = Vec::new();

        let mut q = VecDeque::<usize>::new();
        q.push_back(35);

        while !q.is_empty() && room_count > rooms.len() {
            let cur = q.pop_front().unwrap();

            if cur < 1 || cur > 79 { continue; }

            if grid[cur] != 0 { continue; }

            rooms.push(Room::generate_room(cur, screen));
            grid[cur] = rooms.len();

            q.push_back(cur + 10);
            q.push_back(cur - 10);
            q.push_back(cur + 1);
            q.push_back(cur - 1);
        }

        for r in rooms.iter_mut() {
            let n = r.grid_num - 10;
            let s = r.grid_num + 10;
            let w = r.grid_num - 1;
            let e = r.grid_num + 1;

            if n > 0          && grid[n] != 0 { r.doors[0] = Some(Door { is_open: false, connects_to: grid[n] }); }
            if s > grid.len() && grid[s] != 0 { r.doors[2] = Some(Door { is_open: false, connects_to: grid[s] }); }
            if w > 0          && grid[w] != 0 { r.doors[1] = Some(Door { is_open: false, connects_to: grid[w] }); }
            if e > grid.len() && grid[e] != 0 { r.doors[3] = Some(Door { is_open: false, connects_to: grid[e] }); }
        }

        Dungeon {
            grid,
            rooms,
        }
    }

    pub fn get_room(&self, grid_num: usize) -> GameResult<&Room> {
        let index = self.get_room_index(grid_num)?;
        if !(0..self.rooms.len()).contains(&index) { return Err(Errors::UnknownRoomIndex(index).into()); }
        Ok(&self.rooms[index])
    }

    pub fn get_room_mut(&mut self, grid_num: usize) -> GameResult<&mut Room> {
        let index = self.get_room_index(grid_num)?;
        if !(0..self.rooms.len()).contains(&index) { return Err(Errors::UnknownRoomIndex(index).into()); }
        Ok(&mut self.rooms[index])
    }

    fn get_room_index(&self, grid_num: usize) -> GameResult<usize> {
        if !(0..self.grid.len()).contains(&grid_num) { return Err(Errors::UnknownGridNum(grid_num).into()); }
        Ok(self.grid[grid_num] - 1)
    }

    pub fn get_start_room_grid_num() -> usize { GRID_ROWS * GRID_COLS / 2 }
}

// #[derive(Debug, Copy, Clone)]
// pub struct Door {
//     pub pos,
//     pub bbox
//     pub is_open: bool,
//     pub connects_to: usize,
// }

#[derive(Debug, Copy, Clone)]
pub struct Door {
    pub is_open: bool,
    pub connects_to: usize,
}
