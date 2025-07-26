use anyhow::Result;
use libloading::{Library, Symbol};
use sdl3::{event::Event, keyboard::Keycode, pixels::Color, render::WindowCanvas};
use thiserror::Error;

use std::time::Duration;

struct Game<'a> {
    init_fn: Symbol<'a, fn()>,
    update_and_render_fn: Symbol<'a, fn(canvas: &mut WindowCanvas)>,
}

impl Game<'_> {
    pub fn init(&self) {
        (self.init_fn)()
    }

    pub fn update_and_render(&self, canvas: &mut WindowCanvas) {
        (self.update_and_render_fn)(canvas);
    }
}

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Failed to load dynamic library")]
    FailedToLoadLibrary,

    #[error("Symbol not found in library")]
    SymbolNotFound,
}

impl Game<'_> {
    fn from_lib(lib: &Library) -> Result<Game, LoadError> {
        unsafe {
            let game = Game {
                init_fn: lib.get(b"init").or(Err(LoadError::SymbolNotFound))?,
                update_and_render_fn: lib
                    .get(b"update_and_render")
                    .or(Err(LoadError::SymbolNotFound))?,
            };
            Ok(game)
        }
    }
}

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

pub fn main() -> Result<()> {
    let game_lib = unsafe { Library::new("./target/debug/libgame.so")? };
    let game = Game::from_lib(&game_lib)?;

    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("dev: game", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    game.init();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    game.update_and_render(&mut canvas);

    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
