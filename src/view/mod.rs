use sdl2::{
    EventPump,
    render::{
        Canvas,
    },
    Sdl,
    video::{
        Window,
    },
};

pub use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
};

use crate::font::{
    FontCache,
};

pub struct View {
    context: Sdl,
    width: u32,
    height: u32,
    canvas: Canvas<Window>,
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
            width,
            height,
            canvas,
        })
    }

    pub fn event_pump(&mut self) -> Result<EventPump, String> {
        self.context.event_pump()
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn draw_glyph(&mut self, font: &mut FontCache, glyph: char, color: Color, background: Color, target: Rect) -> Result<(), String> {
        let texture = font.glyph(glyph);
        texture.set_color_mod(color.r, color.g, color.b);

        self.canvas.set_draw_color(background);
        self.canvas.fill_rect(target)?;
        self.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }

    pub fn draw_text(&mut self, 
        font: &mut FontCache, 
        text: &str, 
        color: Color, 
        background: Color,
        (mut x, mut y): (i32, i32), 
        height: u32
    ) -> Result<(), String> { 
        for glyph in text.chars() {
            let (glyph_width, glyph_height) = font.size(glyph);
            let scale = glyph_height / height;
            let scaled_width = glyph_width / scale;

            let target = Rect::new(x, y, scaled_width, height);
            self.draw_glyph(font, glyph, color, background, target)?;

            x += scaled_width as i32;
        }

        Ok(())
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }
}
