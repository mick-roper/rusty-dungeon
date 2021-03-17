extern crate sdl2;

mod game;
mod components;
mod texture_info;
mod map;
mod player;

use sdl2::render::{Canvas, Texture};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::rect::Rect;
use specs::*;
use std::time::Duration;
use std::cmp::{min, max};

use game::State;
use map::{Map, TileType};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const FRAME_RATE: u64 = 1000 / 30;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let window = video_subsystem
        .window("rusty dungeon", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let tileset = texture_creator.load_texture(texture_info::TEXTURE_FILE_PATH)?;

    canvas.clear();
    canvas.set_draw_color(Color::RGB(20, 20, 20));
    canvas.present();

    let mut event_pump = sdl_context.event_pump().expect("couldnt get event pump");
    let mut state = State::new(50, 50);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => break 'running,
                        _ => state.handle_input(keycode),
                    }
                }
                _ => {}
            }
        }
        
        state.update();
        draw(&mut state, &mut canvas, &tileset);

        ::std::thread::sleep(Duration::from_millis(FRAME_RATE));
    }

    println!("quitting...");

    Ok(())
}

fn draw(state: &mut State, canvas: &mut Canvas<sdl2::video::Window>, tileset: &Texture<'_>) {
    canvas.clear();

    // 1: draw the map
    let map = state.ecs.fetch::<Map>();
    let players = state.ecs.read_storage::<components::Player>();
    let positions = state.ecs.read_storage::<components::Position>();
    let mut centre_x: i32 = -1;
    let mut centre_y: i32 = -1;

    for (_player, pos) in (&players, &positions).join() {
        centre_x = pos.x;
        centre_y = pos.y;
        break
    }

    draw_map(centre_x, centre_y, &map, canvas, tileset);

    let mut src = Rect::new(-1, -1, texture_info::TEXTURE_SIZE, texture_info::TEXTURE_SIZE);
    let mut dst = Rect::new(-1, -1, texture_info::TEXTURE_SIZE, texture_info::TEXTURE_SIZE);
    let positions = state.ecs.read_storage::<components::Position>();
    let drawables = state.ecs.read_storage::<components::Drawable>();
    let t_s = texture_info::TEXTURE_SIZE as i32;

    for (pos, draw) in (&positions, &drawables).join() {
        let (src_x, src_y) = draw.texture_index;
        let dst_x = pos.x * t_s;
        let dst_y = pos.y * t_s;

        src.set_x(src_x);
        src.set_y(src_y);
        dst.set_x(dst_x);
        dst.set_y(dst_y);

        canvas.copy(tileset, src, dst).expect("could not draw tileset");
    }

    canvas.present();
}

fn draw_map(map_centre_x: i32, map_centre_y: i32, map: &Map, canvas: &mut Canvas<sdl2::video::Window>, tileset: &Texture<'_>) {
    let (canvas_x, canvas_y) = canvas.window().drawable_size();
    let mut src = Rect::new(-1, -1, texture_info::TEXTURE_SIZE, texture_info::TEXTURE_SIZE);
    let mut dst = Rect::new(-1, -1, texture_info::TEXTURE_SIZE, texture_info::TEXTURE_SIZE);

    let ts_i32 = texture_info::TEXTURE_SIZE as i32;

    // todo: centre the map on the player
    let c_x = map_centre_x * ts_i32;
    let c_y = map_centre_y * (canvas_x / 2) as i32;
    let half_x = (canvas_x / 2) as i32;
    let half_y = (canvas_y / 2) as i32;
    let min_x = max(0, (c_x - half_x) / ts_i32);
    let min_y = max(0, (c_y - half_y) / ts_i32);
    let max_x = min(map.width() as i32, c_x + half_x as i32) / ts_i32;
    let max_y = min(map.height() as i32, c_y + half_y as i32) / ts_i32;

    println!("{} {} {} {}", min_x, min_y, max_x, max_y);

    for x in 0..map.width() {
        let t_x = x * texture_info::TEXTURE_SIZE;
        if t_x > canvas_x {
            break
        }

        for y in 0..map.height() {
            let t_y = y * texture_info::TEXTURE_SIZE;
            if t_y > canvas_y {
                break
            }

            let texture_pos: (i32, i32);

            if x < 0 || y < 0 || x >= map.width() || y >= map.height() {
                texture_pos = texture_info::VOID;
            } else {
                let tile = map.tile_at(x as i32, y as i32);
                texture_pos = match tile.tile_type {
                    TileType::Floor => texture_info::FLOOR_1,
                    TileType::Wall => texture_info::WALL_TOP_1,
                    TileType::Void => texture_info::VOID,
                };
            }

            src.set_x(texture_pos.0);
            src.set_y(texture_pos.1);
            dst.set_x(t_x as i32);
            dst.set_y(t_y as i32);

            canvas.copy(tileset, src, dst).expect("could not copy the texture")
        }
    }
}