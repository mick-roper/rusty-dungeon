use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use specs::prelude::*;

use super::game::State;
use super::components::*;
use super::resource_management::*;

type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;

pub struct Renderer {
    context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture_creator: TextureCreator<WindowContext>,
    // texture_manager: TextureManager<'l, WindowContext>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Result<Renderer, String> {
        let context = sdl2::init()?;
        let video_system = context.video()?;
        let window = video_system
            .window("Rusty Dungeon", width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Renderer{context, canvas, texture_creator})
    }

    pub fn event_pump(&mut self) -> Result<sdl2::EventPump, String> {
        self.context.event_pump()
    }

    pub fn draw(&mut self, state: &mut State) -> Result<(), String>{
        let positions = state.world.read_storage::<Position>();
        let drawables = state.world.read_storage::<Drawable>();
        let r1 = Rect::new(-1, -1, 0, 0);
        let mut r2 = Rect::new(-1, -1, 0, 0);

        self.canvas.clear();

        for (pos, drawable) in (&positions, &drawables).join() {
            r2.set_x(pos.x);
            r2.set_y(pos.y);
            r2.set_width(drawable.width);
            r2.set_height(drawable.height);
            let t = self.texture_creator.load("abc.png")?;
            
            self.canvas.copy(&t, r1, r2)?;
        }

        self.canvas.present();


        Ok(())
    }
}