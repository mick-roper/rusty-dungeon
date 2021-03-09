use rand::Rng;

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub rooms: Vec<Room>,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32, room_count: u32) -> Map {
        panic!("not implemented!");

        // todo: iterate rooms and use them to update the tiles

        // Map{width, height, tiles, rooms}
    }
}

pub struct Tile {
    pub blocked: bool,
    pub tile_coords: (i32, i32),
}

#[derive(PartialEq, Copy, Clone)]
pub struct Room {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Room {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Room {
        Room{x1: x, y1: y, x2: x+w, y2: y+h}
    }
}

fn generate_rooms(n: u32, map_width: u32, map_height: u32) -> Vec<Room> {
    let mut rooms = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let x = rng.gen_range(10..(map_width - 35));
        let y = rng.gen_range(10..(map_height - 35));
        let w = rng.gen_range(5..25);
        let h = rng.gen_range(5..25);

        let new_room = Room::new(x, y, w, h);
        rooms.push(new_room);
    }

    rooms
}