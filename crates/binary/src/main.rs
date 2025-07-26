use anyhow::Result;
use libloading::{Library, Symbol};
use sdl3::pixels::Color;
use shared::{DropParams, InitParams, UpdateAndRenderParams};
use thiserror::Error;

use std::{ptr::NonNull, time::Duration};

use allocator_api2::alloc::Global as GlobalAllocator;

struct Game<'a> {
    #[expect(clippy::type_complexity)]
    init_fn: Symbol<'a, fn(params: InitParams) -> Result<NonNull<[u8]>>>,
    drop_fn: Symbol<'a, fn(params: DropParams)>,
    update_and_render_fn: Symbol<'a, fn(params: UpdateAndRenderParams) -> Result<bool>>,
}

impl Game<'_> {
    pub fn init(&self, params: InitParams) -> Result<NonNull<[u8]>> {
        (self.init_fn)(params)
    }

    pub fn drop(&self, params: DropParams) {
        (self.drop_fn)(params)
    }

    pub fn update_and_render(&self, params: UpdateAndRenderParams) -> Result<bool> {
        (self.update_and_render_fn)(params)
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
                drop_fn: lib.get(b"drop").or(Err(LoadError::SymbolNotFound))?,
                update_and_render_fn: lib
                    .get(b"update_and_render")
                    .or(Err(LoadError::SymbolNotFound))?,
            };
            Ok(game)
        }
    }
}

const WINDOW_WIDTH: u16 = 1920;
const WINDOW_HEIGHT: u16 = 1080;

pub fn main() -> Result<()> {
    let game_lib = unsafe { Library::new("./target/debug/libgame.so")? };
    let game = Game::from_lib(&game_lib)?;

    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("dev: game", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    let game_state = game.init(InitParams {
        allocator: GlobalAllocator,
    })?;

    let mut exit = false;
    let mut prev_now_ms: u64 = 0;

    while !exit {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let now_ms = sdl3::timer::ticks();
        let delta_ms = now_ms - prev_now_ms;
        prev_now_ms = now_ms;

        let params = UpdateAndRenderParams {
            allocator: GlobalAllocator,
            canvas: &mut canvas,
            event_pump: &mut event_pump,
            now_ms,
            delta_ms,
            screen_w: WINDOW_WIDTH,
            screen_h: WINDOW_HEIGHT,
            state: game_state,
        };

        exit = !game.update_and_render(params)?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // TODO: drop gracefully even when in case of error (errdefer)
    // NOTE: from here onwards game_state is dangling
    game.drop(DropParams {
        allocator: GlobalAllocator,
        state: game_state,
    });

    Ok(())
}
