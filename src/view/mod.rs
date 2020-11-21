use sdl2::{
    EventPump,
    pixels::Color,
    rect::Rect,
    render::{
        Canvas,
    },
    Sdl,
    video::{
        Window,
    },
};

use crate::font::{
    FontCache,
};

pub struct View {
    context: Sdl,
    pub canvas: Canvas<Window>,
}

impl View {
    pub fn init(name: &str, width: u32, height: u32) -> Result<View, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
     
        let window = video_subsystem.window(name, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
     
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(View {
            context,
            canvas,
        })
    }

    pub fn event_pump(&mut self) -> Result<EventPump, String> {
        self.context.event_pump()
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn draw_glyph(&mut self, font: &mut FontCache, glyph: char, color: Color, target: Rect) -> Result<(), String> {
        let texture = font.glyph(glyph);
        texture.set_color_mod(color.r, color.g, color.b);

        self.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

}
