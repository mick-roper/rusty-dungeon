use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Drawable {
    pub width: u32,
    pub height: u32,
    pub z_index: u32,
    pub texture_index: (i32, i32),
}

#[derive(Component, Clone, Copy)]
pub struct Animated {
    pub animation_index: i32,
}

#[derive(Component, Copy, Clone)]
pub struct Player{}