pub struct Map {
    pub width: u32,
    pub height: u32,
    tiles: Vec<Tile>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall, Floor
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let tiles = vec![Tile::Floor; (width*height) as usize];
        Map{width, height, tiles}
    }

    pub fn tile_at(&self, x: u32, y: u32) -> Tile {
        self.tiles[self.xy_idx(x, y) as usize]
    }

    fn xy_idx(&self, x: u32, y: u32) -> u32 {
        self.width * y + x
    }
}