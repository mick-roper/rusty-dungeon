use sdl2::rect::{Rect, Point};

use super::texture_info::{VOID, TEXTURE_SIZE};

pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let mut tiles = vec![Tile{position: Point::new(-1, -1), texture_src: Rect::new(VOID.0, VOID.1, TEXTURE_SIZE, TEXTURE_SIZE)}; (width * height) as usize];

        let t = TEXTURE_SIZE as i32;
        let mut x = 0;
        let mut y = 0;

        while x < width {
            while y < height {
                let idx = xy_idx(width, x, y);
                tiles[idx].position.x = x;
                tiles[idx].position.y = y;

                y += t;
            }

            x += t;
            y = 0;
        }

        Map{tiles}
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub position: Point,
    pub texture_src: Rect,
}

fn xy_idx(width: i32, x: i32, y: i32) -> usize {
    (width * y + x) as usize
}