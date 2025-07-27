use anyhow::Result;
use engine::{
    events::Events,
    hooks::{DropParams, InitParams, UpdateAndRenderParams},
};
use libloading::{Library, Symbol};
use sdl3::pixels::Color;
use thiserror::Error;

use std::{fs, ptr::NonNull, time::Duration};

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
    #[error("Failed to unload dynamic library")]
    FailedToUnloadLibrary,
    #[error("Symbol not found in library")]
    SymbolNotFound,
    #[error("Could not find or parse latest.txt")]
    UndefinedLatest,
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

    fn get_latest_library_path() -> Result<String, LoadError> {
        let latest_str =
            fs::read_to_string("./target/debug/latest.txt").or(Err(LoadError::UndefinedLatest))?;
        let latest = latest_str
            .parse::<u64>()
            .or(Err(LoadError::UndefinedLatest))?;
        Ok(format!("./target/debug/libgame.so.{latest}"))
    }
}

const WINDOW_WIDTH: u16 = 1920;
const WINDOW_HEIGHT: u16 = 1080;

pub fn main() -> Result<()> {
    let mut path = Game::get_latest_library_path()?;
    let mut game_lib =
        unsafe { Library::new(path.clone()).or(Err(LoadError::FailedToLoadLibrary))? };
    let mut game = Some(Game::from_lib(&game_lib)?);

    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("dev: game", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let event_pump = sdl_context.event_pump()?;
    let mut events = Events::new(event_pump);

    let game_state = game.as_ref().unwrap().init(InitParams {
        allocator: GlobalAllocator,
    })?;

    let mut exit = false;
    let mut prev_now_ms: u64 = 0;

    while !exit {
        let new_path = Game::get_latest_library_path()?;
        if new_path != path {
            path = new_path;
            sdl3::log::log_info(sdl3::log::Category::Application, "Reloading game");

            game_lib.close().or(Err(LoadError::FailedToUnloadLibrary))?;

            game_lib =
                unsafe { Library::new(path.clone()).or(Err(LoadError::FailedToLoadLibrary))? };
            game = Some(Game::from_lib(&game_lib)?);
        }

        events.clear();
        events.scan();

        if events.key_down(sdl3::keyboard::Keycode::Escape).is_some() {
            break;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let now_ms = sdl3::timer::ticks();
        let delta_ms = now_ms - prev_now_ms;
        prev_now_ms = now_ms;

        let params = UpdateAndRenderParams {
            allocator: GlobalAllocator,
            canvas: &mut canvas,
            events: &mut events,
            now_ms,
            delta_ms,
            screen_w: WINDOW_WIDTH,
            screen_h: WINDOW_HEIGHT,
            state: game_state,
        };

        exit = !game.as_ref().unwrap().update_and_render(params)?;

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // TODO: drop gracefully even when in case of error (errdefer)
    // NOTE: from here onwards game_state is dangling
    if let Some(game) = &game {
        game.drop(DropParams {
            allocator: GlobalAllocator,
            state: game_state,
        });
    }

    Ok(())
}
