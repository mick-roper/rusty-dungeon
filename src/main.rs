extern crate sdl2;

mod game;
mod components;
mod texture_info;
mod map;

use sdl2::render::{Canvas, Texture};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::rect::Rect;
use specs::*;
use std::time::Duration;

use game::State;
use map::{Map, TileType};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const FRAME_RATE: u64 = 1000 / 30;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let window = video_subsystem
        .window("rusty dungeon", WIDTH, HEIGHT)
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

    let mut event_pump = sdl_context.event_pump().expect("counldnt get event pump");
    let mut state = State::new(WIDTH * 5, HEIGHT * 5);

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
        
        state.update()?;
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
    draw_map(150, 250, &map, canvas, tileset);

    // let positions = state.ecs.read_storage::<components::Position>();
    // let drawables = state.ecs.read_storage::<components::Drawable>();

    // for (_pos, _draw) in (&positions, &drawables).join() {
    //     // todo: implement this
    // }

    canvas.present();
}

fn draw_map(centre_x: i32, centre_y: i32, map: &Map, canvas: &mut Canvas<sdl2::video::Window>, tileset: &Texture<'_>) {
    let (canvas_x, canvas_y) = canvas.window().drawable_size();
    let mut src = Rect::new(-1, -1, texture_info::TEXTURE_SIZE, texture_info::TEXTURE_SIZE);
    let mut dst = Rect::new(-1, -1, texture_info::TEXTURE_SIZE, texture_info::TEXTURE_SIZE);

    for x in 0..map.width {
        let t_x = x * texture_info::TEXTURE_SIZE;
        if t_x > canvas_x {
            break
        }

        for y in 0..map.height {
            let t_y = y * texture_info::TEXTURE_SIZE;

            if t_y > canvas_y {
                break
            }

            let tile = map.tile_at(x, y);
            let texture_pos = match tile.tile_type {
                TileType::Floor => texture_info::FLOOR_1,
                TileType::Wall => texture_info::WALL_TOP_1,
                TileType::Void => texture_info::VOID,
            };

            src.set_x(texture_pos.0);
            src.set_y(texture_pos.1);
            dst.set_x(t_x as i32);
            dst.set_y(t_y as i32);

            canvas.copy(tileset, src, dst).expect("could not copy the texture")
        }
    }
}