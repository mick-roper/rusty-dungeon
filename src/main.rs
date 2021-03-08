extern crate sdl2;

mod texture_map;
mod game;
mod components;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use std::time::Duration;

use game::State;
use texture_map::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

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
    let texture = texture_creator.load_texture("resources/tileset.png")?;
    let texture_map = TextureMap::new();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(10, 10, 10));
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut state = State::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                Event::KeyDown {keycode: Some(keycode), ..} => state.handle_input(keycode),
                _ => {}
            }
        }
        
        state.update()?;
        
        canvas.clear();

        // draw floors...
        const STEP: u32 = 16;
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        while x < WIDTH {
            while y < 600 {
                let src = texture_map.get_rect(&FLOOR_TILE_1).unwrap();
                canvas.copy(&texture, *src, Rect::new(x as i32, y as i32, STEP, STEP))?;
                y += STEP;
            }
            y = 0;
            x += STEP;
        } 

        canvas.present();

        ::std::thread::sleep(d);
    }

    Ok(())
}

fn draw_floors() {

}
