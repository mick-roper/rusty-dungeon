use specs::*;
use sdl2::keyboard::Keycode;

use super::*;

pub struct State {
    pub world: World
}

impl State {
    pub fn new(width: u32, height: u32) -> State {
        let mut world = World::empty();

        // world.insert(map::Map::new(width, height));

        // world.insert(Map::new(width, height));
        world.register::<components::Position>();
        world.register::<components::Drawable>();
        world.register::<components::Animated>();

        State{world}
    }

    pub fn handle_input(&mut self, keycode: Keycode) {
        println!("pressed {}", keycode)
    }

    pub fn update(&mut self) -> Result<(), String> {
        // println!("updating the game state...");
        Ok(())
    }
}