use specs::*;
use sdl2::keyboard::Keycode;

use super::*;

pub struct State {
    pub ecs: World
}

impl State {
    pub fn new(width: u32, height: u32) -> State {
        let mut ecs = World::empty();

        ecs.insert(map::Map::new(width, height));
        ecs.register::<components::Position>();
        ecs.register::<components::Drawable>();
        ecs.register::<components::Animated>();

        State{ecs}
    }

    pub fn handle_input(&mut self, keycode: Keycode) {
        println!("pressed {}", keycode)
    }

    pub fn update(&mut self) -> Result<(), String> {
        // println!("updating the game state...");
        Ok(())
    }
}