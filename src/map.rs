use sdl2::rect::{Rect};
use rand::{thread_rng, Rng};

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub rooms: Vec<Rect>,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let mut tiles = vec![Tile{tile_type: TileType::Void}; (width * height) as usize];
        let rooms = generate_rooms(1, width / 10, height / 10);

        for room in rooms.iter() {
            let r_x = room.x();
            let r_x2 = r_x + room.width() as i32;
            let r_y = room.y();
            let r_y2 = r_y + room.height() as i32;

            for x in r_x..r_x2 {
                for y in r_y..r_y2 {
                    let idx = xy_idx(width, x as u32, y as u32);
                    let new_tile_type: TileType;
                    if x == r_x || x == r_x2-1 || y == r_y || y == r_y2-1 {
                        new_tile_type = TileType::Wall;
                    } else {
                        new_tile_type = TileType::Floor;
                    }

                    tiles[idx].tile_type = new_tile_type;
                }
            }
        }

        Map{width, height, tiles, rooms}
    }

    pub fn tile_at(&self, x: u32, y: u32) -> &Tile {
        let idx = xy_idx(self.width, x, y);
        &self.tiles[idx]
    }
}

fn xy_idx(width: u32, x: u32, y: u32) -> usize {
    (width * y + x) as usize
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