extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_system = sdl_context.video()?;

    let window = video_system
        .window("Rusty Dungeon", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .map_err(|e| e.to_string())?;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..256 {
            for x in 0..256 {
                let offset = y * pitch + x * 3;
                buffer[offset] = x as u8;
                buffer[offset + 1] = y as u8;
                buffer[offset + 2] = 0;
            }
        }
    })?;

    
    canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
    canvas.copy_ex(
        &texture,
        None,
        Some(Rect::new(450, 100, 256, 256)),
        30.0,
        None,
        false,
        false,
    )?;
    

    let mut event_pump = sdl_context.event_pump()?;

    let d = Duration::new(0, 1_000_000_000u32 / 30);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        
        update(d);
        draw(&mut canvas);
        ::std::thread::sleep(d);
    }

    Ok(())
}

fn update(elapsed: Duration) {

}

fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.clear();

    canvas.present();
}
