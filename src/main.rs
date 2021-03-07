extern crate sdl2;

mod game;
mod renderer;
mod components;
mod resource_management;

use sdl2::event::Event;
use std::time::Duration;

use game::State;
use renderer::Renderer;

fn main() -> Result<(), String> {
    let mut state = State::new();
    let mut renderer = Renderer::new(800, 600)?;

    let mut event_pump = renderer.event_pump()?;

    let d = Duration::new(0, 1_000_000_000u32 / 30);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                Event::KeyDown {keycode: Some(keycode), ..} => state.handle_input(keycode),
                _ => {}
            }
        }
        
        state.update();
        renderer.draw(&mut state);
        ::std::thread::sleep(d);
    }

    Ok(())
}
