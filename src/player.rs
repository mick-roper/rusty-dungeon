use specs::*;
use super::game::State;
use super::components::*;
use super::map::Map;

pub fn try_move_player(delta: (i32, i32), state: &mut State) {
    let players = state.ecs.read_storage::<Player>();
    let mut positions = state.ecs.write_storage::<Position>();
    let map = state.ecs.fetch::<Map>();

    for (_player, pos) in (&players, &mut positions).join() {
        let new_x = pos.x + delta.0;
        let new_y = pos.y + delta.1;
        if new_x > 0 && new_x < map.width() as i32 && new_y > 0 && new_y < map.height() as i32 && !map.tile_at(new_x, new_y).is_blocked {
            pos.x = new_x;
            pos.y = new_y;
        }
    }
}