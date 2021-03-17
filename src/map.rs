use sdl2::rect::{Rect};
use rand::{thread_rng, Rng};
use std::cmp::min;

pub struct Map {
    width: u32,
    height: u32,
    pub rooms: Vec<Rect>,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let mut tiles = vec![Tile{tile_type: TileType::Void, is_blocked: true}; (width * height) as usize];
        let rooms = generate_rooms(10, width, height);

        for room in rooms.iter() {
            let r_x = room.x();
            let r_y = room.y();
            let r_x2 = min(r_x + room.width() as i32, width as i32);
            let r_y2 = min(r_y + room.height() as i32, height as i32);

            for x in r_x..r_x2 {
                for y in r_y..r_y2 {
                    let idx = xy_idx(width, x, y);

                    if idx > tiles.len() {
                        panic!("{}*{}+{}={} is greater than {}", width, y, x, idx, tiles.len());
                    }

                    let new_tile_type: TileType;
                    let is_blocked: bool;
                    if (x == r_x || x == r_x2-1 || y == r_y || y == r_y2-1) && tiles[idx].tile_type != TileType::Floor {
                        new_tile_type = TileType::Wall;
                        is_blocked = true;
                    } else {
                        new_tile_type = TileType::Floor;
                        is_blocked = false;
                    }

                    tiles[idx].tile_type = new_tile_type;
                    tiles[idx].is_blocked = is_blocked;
                }
            }
        }

        Map{width, height, tiles, rooms}
    }

    pub fn tile_at(&self, x: i32, y: i32) -> &Tile {
        let idx = xy_idx(self.width, x, y);
        &self.tiles[idx]
    }

    pub fn first_room(&self) -> &Rect {
        &self.rooms[0]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

fn xy_idx(width: u32, x: i32, y: i32) -> usize {
    (width as i32 * y + x) as usize
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub is_blocked: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Void,
    Wall,
    Floor
}

fn generate_rooms(room_count: usize, width: u32, height: u32) -> Vec<Rect> {
    let mut rooms = Vec::new();
    let mut rng = thread_rng();

    let min_x = width / 10;
    let min_y = height / 10;

    for _ in 0..room_count {
        let x: i32 = rng.gen_range(min_x as i32..(width as i32-min_x as i32));
        let y: i32 = rng.gen_range(min_y as i32..(height as i32-min_y as i32));
        let w: u32 = rng.gen_range(10..20);
        let h: u32 = rng.gen_range(10..20);

        let r = Rect::new(x, y, w, h);
        rooms.push(r);
    }

    rooms
}