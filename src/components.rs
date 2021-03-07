use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Copy, Clone)]
pub struct Drawable {
    pub width: u32,
    pub height: u32,
    pub texture_id: u32,
    pub z_index: u32,
}