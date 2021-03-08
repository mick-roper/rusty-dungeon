use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use specs::prelude::*;

use super::game::State;
use super::components::*;

const TEXTURE_ID: String = String::from("resources/0x72_DungeonTilesetII_v1.3.png");

pub struct Renderer {
    context: sdl2::Sdl,
    canvas: Canvas<sdl2::video::Window>,
    texture_map: HashMap<String, Texture<'static>>,
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

        let mut canvas: Canvas<sdl2::video::Window> = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();
        let texture_map = HashMap::new();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Renderer{context, canvas, texture_map})
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
            let t = &self.texture_map[&TEXTURE_ID];
            
            self.canvas.copy(t, r1, r2)?;
        }

        self.canvas.present();


        Ok(())
    }
}