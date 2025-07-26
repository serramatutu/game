use std::ptr::NonNull;

use allocator_api2::alloc::{Allocator, Layout};
use anyhow::Result;
use sdl3::{event::Event, keyboard::Keycode, pixels::Color, render::FRect};
use shared::{DropParams, InitParams, UpdateAndRenderParams};

struct State {
    pos: u16,
}

#[unsafe(no_mangle)]
pub fn init(params: InitParams) -> Result<NonNull<[u8]>> {
    let layout = Layout::new::<State>();
    let ptr = params.allocator.allocate(layout)?;

    let state = unsafe { ptr.cast::<State>().as_mut() };
    state.pos = 0;

    Ok(ptr)
}

#[unsafe(no_mangle)]
pub fn drop(params: DropParams) {
    let layout = Layout::new::<State>();
    unsafe {
        params
            .allocator
            .deallocate(params.state.cast::<u8>(), layout);
    }
}

#[unsafe(no_mangle)]
pub fn update_and_render(params: UpdateAndRenderParams) -> Result<bool> {
    let state = unsafe { params.state.cast::<State>().as_mut() };

    for event in params.event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Ok(false),
            _ => {}
        }
    }

    state.pos += (500 * params.delta_ms / 1000) as u16;

    while state.pos > params.screen_w {
        state.pos -= params.screen_w;
    }

    params.canvas.set_draw_color(Color::RGB(255, 0, 0));
    params.canvas.draw_rect(FRect {
        x: state.pos as f32,
        y: 10.0,
        w: 50.0,
        h: 50.0,
    })?;
    Ok(true)
}
