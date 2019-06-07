use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use std::time::Duration;

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn run_app(&self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let width = 1280;
        let height = 720;

        let window = video_subsystem
            .window("draw-rs", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let mut font = ttf_context.load_font("example.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {}
                    _ => {}
                }
            }
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

            canvas.set_draw_color(Color::RGB(240, 240, 240));
            canvas.clear();

            // let surface = font
            //     .render(&format!("Score: {}", score))
            //     .blended(Color::RGBA(0, 0, 0, 255))
            //     .map_err(|e| e.to_string())?;
            // let texture = texture_creator
            //     .create_texture_from_surface(&surface)
            //     .map_err(|e| e.to_string())?;
            // let TextureQuery { width, height, .. } = texture.query();
            // canvas.copy(&texture, None, Some(Rect::new(150, 400, 100, 40)))?;

            canvas.present();
        }

        Ok(())
    }
}
