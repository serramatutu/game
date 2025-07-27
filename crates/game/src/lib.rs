mod zorb;

use std::ptr::NonNull;

use allocator_api2::alloc::{Allocator, Layout};
use anyhow::Result;
use engine::hooks::{DropParams, InitParams, UpdateAndRenderParams};
use math::WorldPoint;

use zorb::Zorb;

struct State {
    zorb: Zorb,
}

#[unsafe(no_mangle)]
pub fn init(params: InitParams) -> Result<NonNull<[u8]>> {
    let layout = Layout::new::<State>();
    let ptr = params.allocator.allocate(layout)?;

    let state = unsafe { ptr.cast::<State>().as_mut() };
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
pub fn update_and_render(mut params: UpdateAndRenderParams) -> Result<bool> {
    let state = unsafe { params.state.cast::<State>().as_mut() };

    if let Some(mouse) = params.events.mouse_up(sdl3::mouse::MouseButton::Left) {
        // TODO: screen to world
        state.zorb.target = WorldPoint::new(mouse.pos.x, mouse.pos.y);
    }

    state.zorb.update_and_render(&mut params)?;

    Ok(true)
}
