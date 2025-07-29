mod zorb;

use std::ptr::NonNull;

use allocator_api2::alloc::{Allocator, Layout};
use anyhow::Result;
use engine::coords::{self, WorldPoint, WorldRect, WorldSize};
use engine::hooks::{DropParams, InitParams, UpdateAndRenderParams};

use sdl3::pixels::Color;
use zorb::Zorb;

struct State {
    zorb: Zorb,
}

#[unsafe(no_mangle)]
pub fn init(params: &mut InitParams) -> Result<NonNull<[u8]>> {
    let layout = Layout::new::<State>();
    let ptr = params.allocator.allocate(layout)?;

    let state = unsafe { ptr.cast::<State>().as_mut() };

    params.camera.init(0.5, 3.0, WorldPoint::origin());
    params.camera.set_zoom(0.5);

    state.zorb.pos = WorldPoint::new(400.0, 400.0);
    state.zorb.target = WorldPoint::new(400.0, 400.0);

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
pub fn update_and_render(params: &mut UpdateAndRenderParams) -> Result<bool> {
    let state = unsafe { params.state.cast::<State>().as_mut() };

    // handle input
    if let Some(mouse) = params.events.mouse_up(sdl3::mouse::MouseButton::Left) {
        state.zorb.target = params.camera.screen_to_world_point(&mouse.pos);
    }
    if params.events.key_down(sdl3::keyboard::Keycode::W).is_some() {
        params.camera.pos.y -= 300.0 * params.delta_ms as f32 / 1000.0;
    }
    if params.events.key_down(sdl3::keyboard::Keycode::S).is_some() {
        params.camera.pos.y += 300.0 * params.delta_ms as f32 / 1000.0;
    }
    if params.events.key_down(sdl3::keyboard::Keycode::A).is_some() {
        params.camera.pos.x -= 300.0 * params.delta_ms as f32 / 1000.0;
    }
    if params.events.key_down(sdl3::keyboard::Keycode::D).is_some() {
        params.camera.pos.x += 300.0 * params.delta_ms as f32 / 1000.0;
    }
    if params.events.key_down(sdl3::keyboard::Keycode::Z).is_some() {
        params
            .camera
            .change_zoom(1.0 * params.delta_ms as f32 / 1000.0);
    }
    if params.events.key_down(sdl3::keyboard::Keycode::X).is_some() {
        params
            .camera
            .change_zoom(-(params.delta_ms as f32) / 1000.0);
    }

    params.canvas.set_draw_color(Color::YELLOW);
    params
        .canvas
        .fill_rect(coords::convert::screen_rect_to_sdl(
            &params.camera.world_to_screen_rect(&WorldRect::new(
                WorldPoint::origin(),
                WorldSize::new(25.0, 25.0),
            )),
        ))?;

    // updante and render
    state.zorb.update_and_render(params)?;

    Ok(true)
}
