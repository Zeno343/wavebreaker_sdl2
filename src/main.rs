use sdl2::{
    event::Event,
    EventPump,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{
        Canvas,
        TextureQuery,
    },
    Sdl,
    video::{
        Window,
    },
};

mod font;
use font::{
    FontCache,
    FontManager,
};

mod view;
use view::View;

fn main() -> Result<(), String> {
    let mut view = View::init().unwrap();

    let mut event_pump = view.event_pump().unwrap();

    let font_manager = FontManager::init(&view.canvas).unwrap();
    let mut input_mono = font_manager.load("assets/InputMono-Regular.ttf").unwrap();

    view.clear(Color::RGB(0, 0, 0));
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
        view.draw_glyph(&mut input_mono, 'A', Color::RGB(255, 0, 0), Rect::new(0, 0, 50, 50))?;
        view.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
