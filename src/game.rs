use specs::*;
use sdl2::keyboard::Keycode;

use super::*;

pub struct State<'a, 'b> {
    pub ecs: World,
    dispatcher: specs::Dispatcher<'a, 'b>,
}

impl State<'_, '_> {
    pub fn new(width: u32, height: u32) -> State<'static, 'static> {
        let mut ecs = World::new();
        
        let map: map::Map;
        let player_x: i32;
        let player_y: i32;

        {
            map = map::Map::new(width, height);
            let room = map.first_room();
            player_x = room.center().x();
            player_y = room.center().y();
        }

        println!("player start pos: {} {}", player_x, player_y);

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

        let dispatcher: Dispatcher = DispatcherBuilder::new().build();

        State{ecs, dispatcher}
    }

    pub fn handle_input(&mut self, keycode: Keycode) {
        let delta = match keycode {
            Keycode::Left => (-1, 0),
            Keycode::Right => (1, 0),
            Keycode::Up => (0, -1),
            Keycode::Down => (0, 1),
            _ => return
        };

        player::try_move_player(delta, self);
    }

    pub fn update(&mut self) {
        self.dispatcher.dispatch(&self.ecs);
        self.ecs.maintain();
    }
}