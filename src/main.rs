use sdl2::{
    event::Event,
    EventPump,
    keyboard::Keycode,
    pixels::Color,
    render::Canvas,
    Sdl,
    video::Window
};

pub struct View {
    context: Sdl,
    canvas: Canvas<Window>
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

        Ok(View {
            context,
            canvas
        })
    }

    fn event_pump(&mut self) -> Result<EventPump, String> {
        self.context.event_pump()
    }

    fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.canvas.present();
    }
}

fn main() {
    let mut view = View::init().unwrap();

    let mut event_pump = view.event_pump().unwrap();

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

        view.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
