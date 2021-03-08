use sdl2::rect::Rect;
use std::collections::HashMap;

pub const FLOOR_TILE_1: u32 = 0;
pub const FLOOR_TILE_2: u32 = 1;
pub const FLOOR_TILE_3: u32 = 2;
pub const FLOOR_TILE_4: u32 = 3;
pub const FLOOR_TILE_5: u32 = 4;
pub const FLOOR_TILE_6: u32 = 5;
pub const WALL_1: u32 = 100;

pub struct TextureMap {
    map: HashMap<u32, Rect>
}

impl TextureMap {
    pub fn new() -> TextureMap {
        let mut map = HashMap::new();

        // floor tiles
        map.insert(FLOOR_TILE_1, Rect::new(16, 64, 16, 16));
        map.insert(FLOOR_TILE_2, Rect::new(32, 64, 16, 16));
        map.insert(FLOOR_TILE_3, Rect::new(48, 64, 16, 16));
        map.insert(FLOOR_TILE_4, Rect::new(16, 80, 16, 16));
        map.insert(FLOOR_TILE_5, Rect::new(32, 80, 16, 16));
        map.insert(FLOOR_TILE_6, Rect::new(48, 80, 16, 16));

        // wall tiles
        map.insert(WALL_1, Rect::new(36, 124, 24, 20));

        TextureMap{map}
    }

    pub fn get_rect(&self, idx: &u32) -> Option<&Rect> {
        self.map.get(idx)
    }
}