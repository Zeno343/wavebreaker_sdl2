use std::{
    cell::RefCell,
    char::from_u32,
    collections::HashMap,
    path::Path,
};

use sdl2::{
    event::Event,
    EventPump,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{
        Canvas,
        Texture,
        TextureCreator,
        TextureQuery,
    },
    Sdl,
    ttf,
    ttf::Sdl2TtfContext,
    video::{
        Window,
        WindowContext,
    },
};

pub struct FontCache<'a> {
    glyph_map: HashMap<char, Texture<'a>>,
}

impl<'a> FontCache<'a> {
    pub fn glyph(&self, glyph: char) -> &'a Texture {
        self.glyph_map.get(&glyph).unwrap()
    }
}

pub struct FontManager {
    ttf_context: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
}

impl FontManager {
    pub fn init(canvas: &Canvas<Window>) -> Result<FontManager, String> {
        let ttf_context = ttf::init().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator(); 
        
        Ok(FontManager {
            ttf_context,
            texture_creator,
        })
    }

    pub fn load(&self, path: &str) -> Result<FontCache, String> {
        let mut font = self.ttf_context.load_font(path, 128)?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        let mut glyph_map = HashMap::new();
        
        //Latin alphabet
        for c in 0x0020 ..= 0x007F {
            let char_code = from_u32(c).unwrap();
            let surface = font.render(&char_code.to_string())
                .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())?;
            let texture = self.texture_creator.create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            
            glyph_map.insert(char_code, texture);
        }

        //
        for c in 0x2580 ..= 0x259F {
            let char_code = from_u32(c).unwrap();
            let surface = font.render(&char_code.to_string())
                .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())?;
            let texture = self.texture_creator.create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            
            glyph_map.insert(char_code, texture);
        }
        Ok(FontCache {
            glyph_map
        }) 
    }
}

pub struct View {
    context: Sdl,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

impl View {
    fn init() -> Result<View, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
     
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
     
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        Ok(View {
            context,
            canvas,
            texture_creator,
        })
    }

    fn event_pump(&mut self) -> Result<EventPump, String> {
        self.context.event_pump()
    }

    fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_glyph(&mut self, font: &FontCache, glyph: char) {
        let texture = font.glyph(glyph);
        let TextureQuery { width, height, ..} = texture.query();
        let target = Rect::new(0, 0, width, height);

        self.canvas.copy(&texture, None, Some(target));
    }

    fn present(&mut self) {
        self.canvas.present();
    }

}

fn main() {
    let mut view = View::init().unwrap();

    let mut event_pump = view.event_pump().unwrap();

    let font_manager = FontManager::init(&view.canvas).unwrap();
    let input_mono = font_manager.load("InputMono-Regular.ttf").unwrap();

    view.clear(Color::RGB(0, 0, 0));
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        view.clear(Color::RGB(0, 0, 0));
        view.draw_glyph(&input_mono, 'A');
        view.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
