mod zorb;

use std::path::PathBuf;
use std::ptr::NonNull;

use allocator_api2::alloc::{Allocator, Layout};
use anyhow::Result;
use engine::coords::{self, WorldPoint, WorldRect, WorldSize};
use engine::hooks::{DropParams, InitParams, UpdateAndRenderParams};

use engine::resources::sprite_map::SpriteMap;
use engine::types::Id;
use sdl3::pixels::Color;
use zorb::Zorb;

struct LoadedResources<'res> {
    zorb_face: Id<'res, SpriteMap<'res>>,
}

struct State<'res> {
    res: LoadedResources<'res>,
    zorb: Zorb,
}

#[unsafe(no_mangle)]
pub fn init<'eng>(params: &'eng mut InitParams<'eng, 'eng>) -> Result<NonNull<[u8]>> {
    let layout = Layout::new::<State>();
    let ptr = params.allocator.allocate(layout)?;

    let state = unsafe { ptr.cast::<State>().as_mut() };

    params.resources.root = PathBuf::from("resources/obj");

    params.camera.init(0.5, 3.0, WorldPoint::origin());
    params.camera.set_zoom(0.5);

    state.res.zorb_face = params.resources.load_sprite_map("zorb/face")?;

    state.zorb.pos = WorldPoint::new(400.0, 400.0);
    state.zorb.target = WorldPoint::new(400.0, 400.0);

    Ok(ptr)
}

#[unsafe(no_mangle)]
pub fn drop(params: DropParams) {
    // TODO: unload resources
    let layout = Layout::new::<State>();
    unsafe {
        params
            .allocator
            .deallocate(params.state.cast::<u8>(), layout);
    }
}

#[unsafe(no_mangle)]
pub fn update_and_render<'eng>(
    params: &'eng mut UpdateAndRenderParams<'eng, 'eng>,
) -> Result<bool> {
    let state = unsafe { params.state.cast::<State>().as_mut() };

    if params.events.quit() || params.events.key(sdl3::keyboard::Keycode::Escape).down {
        return Ok(false);
    }

    // handle input
    let left_mouse = params.events.mouse_btn(sdl3::mouse::MouseButton::Left);
    if left_mouse.down {
        state.zorb.target = params.camera.screen_to_world_point(&left_mouse.pos);
    }

    if params.events.key(sdl3::keyboard::Keycode::W).down {
        params.camera.pos.y -= 300.0 * params.delta_ms as f32 / 1000.0;
    }
    if params.events.key(sdl3::keyboard::Keycode::S).down {
        params.camera.pos.y += 300.0 * params.delta_ms as f32 / 1000.0;
    }
    if params.events.key(sdl3::keyboard::Keycode::A).down {
        params.camera.pos.x -= 300.0 * params.delta_ms as f32 / 1000.0;
    }
    if params.events.key(sdl3::keyboard::Keycode::D).down {
        params.camera.pos.x += 300.0 * params.delta_ms as f32 / 1000.0;
    }
    if params.events.key(sdl3::keyboard::Keycode::Z).down {
        params.camera.change_zoom_around(
            1.0 * params.delta_ms as f32 / 1000.0,
            params.events.mouse_pos,
        );
    }
    if params.events.key(sdl3::keyboard::Keycode::X).down {
        params
            .camera
            .change_zoom_around(-(params.delta_ms as f32) / 1000.0, params.events.mouse_pos);
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
    state.zorb.update_and_render(&state.res, params)?;

    Ok(true)
}
