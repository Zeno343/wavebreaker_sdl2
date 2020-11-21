use std::{
    char::from_u32,
    collections::HashMap,
};

use sdl2::{
    pixels::Color,
    render::{
        Canvas,
        Texture,
        TextureCreator,
    },
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
    pub fn glyph(&mut self, glyph: char) -> &mut Texture<'a> {
        self.glyph_map.get_mut(&glyph).unwrap()
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
                .blended(Color::RGBA(255, 255, 255, 255)).map_err(|e| e.to_string())?;
            let texture = self.texture_creator.create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            
            glyph_map.insert(char_code, texture);
        }

        //
        for c in 0x2580 ..= 0x259F {
            let char_code = from_u32(c).unwrap();
            let surface = font.render(&char_code.to_string())
                .blended(Color::RGBA(255, 255, 255, 255)).map_err(|e| e.to_string())?;
            let texture = self.texture_creator.create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            
            glyph_map.insert(char_code, texture);
        }

        Ok(FontCache {
            glyph_map
        }) 
    }
}
