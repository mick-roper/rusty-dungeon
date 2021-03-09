extern crate sdl2;

mod game;
mod map;
mod components;
mod tile_coords;

use sdl2::render::{Canvas, Texture};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::rect::Rect;
use specs::*;
use std::time::Duration;

use game::State;
use map::{Map, Tile};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const TILE_SIZE: u32 = 16;

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
    let tileset = texture_creator.load_texture("resources/tileset.png")?;

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
    let map = state.world.fetch::<Map>();
    draw_map(&map, canvas, tileset)?;

    let positions = state.world.read_storage::<components::Position>();
    let drawables = state.world.read_storage::<components::Drawable>();

    for (_pos, _draw) in (&positions, &drawables).join() {
        // todo: implement this
    }

    canvas.present();

    Ok(())
}

fn draw_map(map: &Map, canvas: &mut Canvas<sdl2::video::Window>, tileset: &Texture<'_>) -> Result<(), String> {
    let mut src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
    let mut dst = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);

    let mut x = 0;
    let mut y = 0;
    let mut tile_xy: (i32, i32);

    // draw the floor
    while x < map.width {
        dst.set_x(x as i32);
        while y < map.height {
            match (x, y) {
                (0, 0) => {
                    tile_xy = tile_coords::WALL_CORNER_TOP_LEFT;
                },
                (0, 752) => {
                    tile_xy = tile_coords::WALL_CORNER_BTM_LEFT;
                },
                (1008, 0) => {
                    tile_xy = tile_coords::WALL_CORNER_TOP_RIGHT;
                },
                (1008, 752) => {
                    tile_xy = tile_coords::WALL_CORNER_BTM_RIGHT;
                },
                (0, ..) => {
                    tile_xy = tile_coords::WALL_LEFT_1;
                },
                (1008, ..) => {
                    tile_xy = tile_coords::WALL_RIGHT_1;
                },
                (.., 0) => {
                    tile_xy = tile_coords::WALL_TOP_1;
                },
                (.., 752) => {
                    tile_xy = tile_coords::WALL_BTM_1;
                },
                _ => { // floor
                    tile_xy = tile_coords::FLOOR_1;
                }
            }
            let (tx, ty) = tile_xy;
            src.set_x(tx);
            src.set_y(ty);
            dst.set_y(y as i32);
            canvas.copy(tileset, src, dst)?;
            y += TILE_SIZE;
        }

        x += TILE_SIZE;
        y = 0;
    }

    Ok(())
}