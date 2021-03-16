use specs::*;
use sdl2::keyboard::Keycode;

use super::*;

pub struct State {
    pub ecs: World
}

impl State {
    pub fn new(width: u32, height: u32) -> State {
        let mut ecs = World::new();
        
        let map: map::Map;
        let player_x: i32;
        let player_y: i32;

        {
            map = map::Map::new(width, height);
            player_x = map.first_room().x();
            player_y = map.first_room().y();
        }

        ecs.insert(map);
        ecs.register::<components::Position>();
        ecs.register::<components::Drawable>();
        ecs.register::<components::Animated>();
        ecs.register::<components::Player>();

        ecs.create_entity()
            .with(components::Player{})
            .with(components::Drawable{
                width: texture_info::TEXTURE_SIZE,
                height: texture_info::TEXTURE_SIZE,
                z_index: 1,
                texture_index: texture_info::PLAYER,
            })
            .with(components::Position{
                x: player_x,
                y: player_y,
            })
            .build();

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