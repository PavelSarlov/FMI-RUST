use ggez::{
    graphics::{self, DrawParam, Rect},
    GameResult,
    Context
};
use crate::{
    assets::*,
    entities::{Enemy},
    utils::Errors,
};
use std::{
    collections::{VecDeque},
    fmt::{Formatter, Display},
    f32::consts::PI,
};
use rand::{thread_rng, Rng};

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
    pub enemies: Vec<Enemy>,
    pub grid_num: usize,
    pub doors: [usize; 4],
}

impl Room {
    pub fn update(&mut self) {
        todo!()
    }
    
    pub fn draw(&self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32)) -> GameResult {
        let (sw, sh) = world_coords;
        let draw_params = DrawParam::default()
            .dest([0., 0.])
            .scale(Room::get_room_scale(sw, sh, assets.room_base.dimensions()));
        
        graphics::draw(ctx, &assets.room_base, draw_params)?;

        for (i, v) in self.doors.iter().enumerate() {
            match i {
                0 => { graphics::draw(ctx, &assets.door_north, draw_params)?; },
                1 => { graphics::draw(ctx, &assets.door_east, draw_params)?; },
                2 => { graphics::draw(ctx, &assets.door_south, draw_params)?; },
                _ => { graphics::draw(ctx, &assets.door_west, draw_params)?; },
            }
        }

        Ok(())
    }

    fn get_room_scale(sw: f32, sh: f32, image: Rect) -> [f32; 2] {
        let x = sw / image.w;
        let y = sh / image.h;
        [x, y]
    }

    fn generate_room(grid_num: usize) -> Room {
        let enemies = Vec::new();
        let doors = [0; 4];

        Room {
            enemies,
            grid_num,
            doors,
        }
    }
}

#[derive(Debug)]
pub struct Dungeon {
    grid: [[usize; 8]; 9],
    rooms: Vec<Room>,
}

impl Dungeon {
    pub fn generate_dungeon() -> Self {
        let level = 1;
        let room_count = thread_rng().gen_range(0..2) + 5 + level * 2;
        let mut grid = [[0; 8]; 9];
        let mut rooms = Vec::new();

        let mut q = VecDeque::<usize>::new();
        q.push_back(35);

        while !q.is_empty() && room_count > rooms.len() {
            let cur = q.pop_front().unwrap();

            if cur < 1 || cur > 79 { continue; }

            let (r, c) = (cur / 10, cur % 10);

            if grid[r][c] != 0 { continue; }

            grid[r][c] = rooms.len() + 1;
            rooms.push(Room::generate_room(cur));

            q.push_back(cur + 10);
            q.push_back(cur - 10);
            q.push_back(cur + 1);
            q.push_back(cur - 1);
        }

        for i in 0..9 {
            for j in 0..8 {
                if grid[i][j] > 0 {
                    let mut room = &mut rooms[grid[i][j] - 1];
                    if i > 0 { room.doors[0] = grid[i-1][j]; }
                    if i < 8 { room.doors[2] = grid[i+1][j]; }
                    if j > 0 { room.doors[1] = grid[i][j+1]; }
                    if j < 7 { room.doors[3] = grid[i][j-1]; }
                }
            }
        }

        Dungeon {
            grid,
            rooms,
        }
    }

    pub fn get_room(&self, index: usize) -> Result<&Room, Errors> {
        if !(0..self.rooms.len()).contains(&index) { return Err(Errors::UnknownRoom(index)); }
        Ok(&self.rooms[index])
    }

    pub fn get_room_mut(&mut self, index: usize) -> Result<&mut Room, Errors> {
        if !(0..self.rooms.len()).contains(&index) { return Err(Errors::UnknownRoom(index)); }
        Ok(&mut self.rooms[index])
    }

    pub fn get_start_room() -> usize { 0 }

    pub fn draw_room(&self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32), index: usize) -> GameResult {
        self.rooms[index].draw(ctx, assets, world_coords)?;

        Ok(())
    }
}
