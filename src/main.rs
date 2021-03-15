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

fn main() -> Result<(), String> {
    let d = Duration::new(0, 1_000_000_000u32 / 30);
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

    let mut event_pump = sdl_context.event_pump()?;
    let mut state = State::new(WIDTH, HEIGHT);

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
        draw(&mut state, &mut canvas, &tileset)?;

        ::std::thread::sleep(d);
    }

    Ok(())
}

fn draw(state: &mut State, canvas: &mut Canvas<sdl2::video::Window>, tileset: &Texture<'_>) -> Result<(), String> {
    canvas.clear();

    canvas.set_draw_color(Color::RGB(20, 20, 20));

    // 1: draw the map
    let map = state.ecs.fetch::<Map>();
    draw_map(150, 250, &map, canvas, tileset)?;

    let positions = state.ecs.read_storage::<components::Position>();
    let drawables = state.ecs.read_storage::<components::Drawable>();

    for (_pos, _draw) in (&positions, &drawables).join() {
        // todo: implement this
    }

    canvas.present();

    Ok(())
}

fn draw_map(centre_x: i32, centre_y: i32, map: &Map, canvas: &mut Canvas<sdl2::video::Window>, tileset: &Texture<'_>) -> Result<(), String> {
    let mut src = Rect::new(0, 0, texture_info::TEXTURE_SIZE, texture_info::TEXTURE_SIZE);
    let mut dst = Rect::new(0, 0, texture_info::TEXTURE_SIZE, texture_info::TEXTURE_SIZE);

    let (w, h) = canvas.window().drawable_size();
    let half_w = w as i32 / 2;
    let half_h = h as i32 / 2;
    let x1 = centre_x - half_w;
    let x2 = centre_x + half_w;
    let y1 = centre_y - half_w;
    let y2 = centre_y + half_h;

    let t_size = texture_info::TEXTURE_SIZE as i32;

    for x in x1..x2 {
        dst.set_x(x);
        for y in y1..y2 {
            dst.set_y(y);

            let tile = map.tile_at(x / t_size, y / t_size);
            let mut texture_point = texture_info::VOID;

            if tile.tile_type == TileType::Wall {
                texture_point = texture_info::WALL_TOP_1;
            } else if tile.tile_type == TileType::Floor {
                texture_point = texture_info::FLOOR_1;
            }

            src.set_x(texture_point.0);
            src.set_y(texture_point.1);

            canvas.copy(tileset, src, dst)?;
        }
    }

    Ok(())
}