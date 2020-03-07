use rltk::{Rltk, BaseMap, Algorithm2D};
use rltk::{Console, RGB, RandomNumberGenerator, Point};
use specs::prelude::*;
use crate::Rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn new_map_rooms_and_corridoors() -> Self {
        let width = 80;
        let height = 50;
        let size = width * height;
        let mut map = Self {
            tiles: vec![TileType::Wall; size],
            rooms: vec![],
            width: width as i32,
            height: height as i32,
            revealed_tiles: vec![false; size],
            visible_tiles: vec![false; size],
        };

        map.generate_rooms();
        
        map
    }

    /// Makes a map with solid boundaries and 400 randomly placed walls.
    /// No guarantees that it won't look awful.
    #[allow(unused)]
    pub fn new_map_test() -> Self {
        let width = 80;
        let height = 50;
        let size = width * height;
        let mut map = Self {
            tiles: vec![TileType::Floor; size],
            rooms: vec![Rect::new(0, 0, width as i32, height as i32)],
            width: width as i32,
            height: height as i32,
            revealed_tiles: vec![false; size],
            visible_tiles: vec![false; size],
        };

        // Make the boundaries walls
        for x in 0..80 {
            let top = map.xy_idx(x, 0);
            let bottom = map.xy_idx(x, 49);

            map.tiles[top] = TileType::Wall;
            map.tiles[bottom] = TileType::Wall;
        }

        for y in 0..50 {
            let left = map.xy_idx(0, y);
            let right = map.xy_idx(79, y);

            map.tiles[left] = TileType::Wall;
            map.tiles[right] = TileType::Wall;
        }

        // Generate a bunch of random walls
        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = map.xy_idx(x, y);
            
            if idx != map.xy_idx(40, 25) {
                map.tiles[idx] = TileType::Wall;
            }
        }

        map
    }

    pub fn generate_rooms(&mut self) {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0 .. MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, self.width - w - 1) - 1;
            let y = rng.roll_dice(1, self.height - h - 1) - 1;

            let new_room = Rect::new(x, y, w, h);
            let room_fits = self.rooms
                .iter()
                .all(|other_room| !new_room.intersects(other_room));
            
            if !room_fits {
                continue;
            }

            self.apply_room_to_map(&new_room);

            if !self.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();

                if rng.range(0, 2) == 1 {
                    self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    self.apply_vertical_tunnel(prev_y, new_y, new_x);
                } else {
                    self.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    self.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }

            self.rooms.push(new_room);
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in (room.y1 + 1) ..= (room.y2) {
            for x in (room.x1 + 1) ..= (room.x2) {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in x1.min(x2) ..= x1.max(x2) {
            let idx = self.xy_idx(x, y);

            if idx > 0 && idx < self.tiles.len() {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in y1.min(y2) ..= y1.max(y2) {
            let idx = self.xy_idx(x, y);

            if idx > 0 && idx < self.tiles.len() {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, _idx: usize) -> Vec<(usize, f32)> {
        Vec::new()
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        1.0
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let (glyph, mut fg) = match tile {
                TileType::Floor => ('.', RGB::from_f32(0., 0.5, 0.5)),
                TileType::Wall => ('#', RGB::from_f32(0.0, 1.0, 0.0)),
            };

            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale();
            }

            ctx.set(
                x, y,
                fg,
                RGB::from_f32(0.0, 0.0, 0.0),
                rltk::to_cp437(glyph),
            );
        }

        x += 1;

        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
