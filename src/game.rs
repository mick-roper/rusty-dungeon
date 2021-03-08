use specs::*;
use sdl2::keyboard::Keycode;

use super::components::*;

pub struct State {
    pub world: World
}

impl State {
    pub fn new() -> State {
        let mut world = World::empty();

        world.register::<Position>();
        world.register::<Drawable>();
        world.register::<Animated>();

        State{world}
    }

    pub fn handle_input(&mut self, keycode: Keycode) {
        println!("pressed {}", keycode)
    }

    pub fn update(&mut self) -> Result<(), String> {
        println!("updating the game state...");
        Ok(())
    }
}