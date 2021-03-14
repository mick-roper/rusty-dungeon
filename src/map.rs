use sdl2::rect::{Rect, Point};

use super::texture_info::{VOID, TEXTURE_SIZE};

pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let n_width = width / TEXTURE_SIZE;
        let n_height = height / TEXTURE_SIZE;
        let mut tiles = vec![Tile{position: Point::new(-1, -1), texture_src: Rect::new(VOID.0, VOID.1, TEXTURE_SIZE, TEXTURE_SIZE)}; (n_width * n_height) as usize];

        let mut x = 0;
        let mut y = 0;

        while x < (width * TEXTURE_SIZE) {
            while y < (height * TEXTURE_SIZE) {
                let idx = xy_idx(width, x, y);
                tiles[idx].position.x = x as i32;
                tiles[idx].position.y = y as i32;

                y += TEXTURE_SIZE;
            }

            x += TEXTURE_SIZE;
            y = 0;
        }

        let rooms = generate_rooms(1, width, height);

        for r in rooms.iter() {

        }

        Map{tiles}
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}

fn xy_idx(width: u32, x: u32, y: u32) -> usize {
    (width * y + x) as usize
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub position: Point,
    pub texture_src: Rect,
}

fn generate_rooms(room_count: usize, width: u32, height: u32) -> Vec<Rect> {
    let mut rooms = Vec::new();

    for _ in 0..room_count {
        let r = Rect::new(200, 275, 15, 15);
        rooms.push(r);
    }

    rooms
}