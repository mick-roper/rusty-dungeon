use sdl2::rect::{Rect, Point};

use super::texture_info::{VOID, TEXTURE_SIZE};

pub struct Map {
    width: u32,
    tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let mut tiles = vec![Tile{tile_type: TileType::Void}; (width * height) as usize];
        let rooms = generate_rooms(1, width, height);

        for room in rooms.iter() {
            let r_x = room.x();
            let r_x2 = r_x + room.width() as i32;
            let r_y = room.y();
            let r_y2 = r_y + room.height() as i32;

            for x in r_x..r_x2 {
                for y in r_y..r_y2 {
                    let idx = xy_idx(width, x, y);
                    let new_tile_type: TileType;
                    if x == r_x || x == r_x2 || y == r_y || y == r_y2 {
                        new_tile_type = TileType::Wall;
                    } else {
                        new_tile_type = TileType::Floor;
                    }

                    tiles[idx].tile_type = new_tile_type;
                }
            }
        }

        Map{width, tiles, rooms}
    }

    pub fn tile_at(&self, x: i32, y: i32) -> &Tile {
        let idx = xy_idx(self.width, x, y);
        &self.tiles[idx]
    }
}

fn xy_idx(width: u32, x: i32, y: i32) -> usize {
    (width as i32 * y + x) as usize
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Void,
    Wall,
    Floor
}

fn generate_rooms(room_count: usize, width: u32, height: u32) -> Vec<Rect> {
    let mut rooms = Vec::new();

    for _ in 0..room_count {
        let r = Rect::new(200, 275, 15, 15);
        rooms.push(r);
    }

    rooms
}