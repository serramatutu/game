mod consts;
mod ecs;
mod global_state;
mod spawnables;

use std::path::PathBuf;
use std::ptr::NonNull;

use allocator_api2::alloc::{Allocator, Layout};
use anyhow::Result;
use ecs::EntitySpawner;
use ecs::components::Follow;
use engine::coords::WorldPoint;
use engine::hooks::{DropParams, InitParams, UpdateAndRenderParams};

use global_state::Ctx;

use crate::global_state::State;

#[unsafe(no_mangle)]
pub fn init<'gamestatic>(
    params: &'gamestatic mut InitParams<'gamestatic, 'gamestatic>,
) -> Result<NonNull<[u8]>> {
    let layout = Layout::new::<State>();
    let ptr = params.allocator.allocate(layout)?;

    let state = unsafe { ptr.cast::<State>().as_mut() };

    params.resources.root = PathBuf::from("resources/obj");

    params.camera.init(0.5, 3.0, WorldPoint::origin());
    params.camera.set_zoom(0.5);

    state.resource_ids.zorb_sprite = params.resources.load_sprite_map("zorb")?;

    // NOTE: have to explicitly call default constructors as memory is initialized
    // with zeros
    state.ecs = Default::default();
    state.zorb = spawnables::zorb::spawn(&mut state.ecs);

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
pub fn update_and_render<'gamestatic>(
    params: &'gamestatic mut UpdateAndRenderParams<'gamestatic, 'gamestatic>,
) -> Result<bool> {
    let new_state = unsafe { params.state.cast::<State>().as_mut() };
    let prev_state = new_state.clone();

    if params.events.quit() || params.events.key(sdl3::keyboard::Keycode::Escape).down {
        return Ok(false);
    }

    // handle input
    let left_mouse = params.events.mouse_btn(sdl3::mouse::MouseButton::Left);
    if left_mouse.down {
        // TODO: move this to an input handling system
        let follow_pos = params.camera.screen_to_world_point(&left_mouse.pos);
        let follow_entity = EntitySpawner::new()
            .with_pos(follow_pos)
            .spawn(&mut new_state.ecs);

        new_state.ecs.overwrite_follow_for(
            prev_state.zorb,
            Follow {
                stop_after_arriving: true,
                target_entity: follow_entity,
            },
        );

        // TODO: notify entity to delete itself
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

    let mut ctx = Ctx {
        allocator: params.allocator,
        camera: params.camera,
        canvas: params.canvas,
        delta_ms: params.delta_ms,
        now_ms: params.now_ms,
        resources: params.resources,
        resource_ids: &new_state.resource_ids,
        screen_w: params.screen_w,
        screen_h: params.screen_h,
    };
    new_state.ecs.update_and_render(&mut ctx, &prev_state.ecs)?;

    Ok(true)
}
