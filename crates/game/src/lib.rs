mod consts;
mod ecs;
mod global_state;
mod spawnables;

use std::path::PathBuf;
use std::ptr::NonNull;

use allocator_api2::alloc::{Allocator, Layout};
use anyhow::Result;
use ecs::components::Follow;
use ecs::{EntitySpawner, SENTINEL};
use engine::coords::WorldPoint;
use engine::hooks::{DropParams, InitParams, UpdateAndRenderParams};
use engine::types::Reset;

use global_state::{Ctx, MemoryPool};

#[unsafe(no_mangle)]
pub fn init<'gs>(params: &'gs mut InitParams<'gs, 'gs>) -> Result<NonNull<[u8]>> {
    let layout = Layout::new::<MemoryPool>();
    let ptr = params.allocator.allocate(layout)?;

    let pool = unsafe { ptr.cast::<MemoryPool>().as_mut() };

    params.resources.root = PathBuf::from("resources/obj");

    params.camera.init(0.5, 3.0, WorldPoint::origin());
    params.camera.set_zoom(0.5);

    // NOTE: have to explicitly call default constructors as memory is initialized
    // with zeros
    pool.prev.ecs.reset();
    pool.next.ecs.reset();
    pool.resource_ids.zorb_sprite = params.resources.load_sprite_map("zorb")?;

    Ok(ptr)
}

#[unsafe(no_mangle)]
pub fn drop(params: DropParams) {
    // TODO: unload resources
    let layout = Layout::new::<MemoryPool>();
    unsafe {
        params
            .allocator
            .deallocate(params.memory.cast::<u8>(), layout);
    }
}

#[unsafe(no_mangle)]
pub fn update_and_render<'gs>(params: &'gs mut UpdateAndRenderParams<'gs, 'gs>) -> Result<bool> {
    let pool = unsafe { params.memory.cast::<MemoryPool>().as_mut() };

    let mut ctx = Ctx {
        allocator: params.allocator,
        camera: params.camera,
        canvas: params.canvas,
        delta_ms: params.delta_ms,
        delta_s: params.delta_ms as f64 / 1000.0,
        now_ms: params.now_ms,
        resources: params.resources,
        resource_ids: &pool.resource_ids,
        screen_w: params.screen_w,
        screen_h: params.screen_h,
    };

    if params.events.quit() || params.events.key(sdl3::keyboard::Keycode::Escape).down {
        return Ok(false);
    }

    let right_mouse = params.events.mouse_btn(sdl3::mouse::MouseButton::Right);
    if right_mouse.down && pool.prev.zorb == SENTINEL {
        pool.next.zorb = spawnables::zorb::spawn(&mut ctx, &mut pool.next.ecs);
    }

    let left_mouse = params.events.mouse_btn(sdl3::mouse::MouseButton::Left);
    if left_mouse.down {
        // TODO: move this to an input handling system
        let follow_pos = ctx.camera.screen_to_world_point(&left_mouse.pos);
        let follow_entity = EntitySpawner::new()
            .with_pos(follow_pos)
            .spawn(&mut pool.next.ecs);

        pool.next.ecs.overwrite_follow_for(
            pool.prev.zorb,
            Follow {
                stop_after_arriving: true,
                target_entity: follow_entity,
            },
        );

        // TODO: notify entity to delete itself
    }

    if params.events.key(sdl3::keyboard::Keycode::W).down {
        ctx.camera.pos.y -= 300.0 * ctx.delta_ms as f64 / 1000.0;
    }
    if params.events.key(sdl3::keyboard::Keycode::S).down {
        ctx.camera.pos.y += 300.0 * ctx.delta_ms as f64 / 1000.0;
    }
    if params.events.key(sdl3::keyboard::Keycode::A).down {
        ctx.camera.pos.x -= 300.0 * ctx.delta_ms as f64 / 1000.0;
    }
    if params.events.key(sdl3::keyboard::Keycode::D).down {
        ctx.camera.pos.x += 300.0 * ctx.delta_ms as f64 / 1000.0;
    }
    if params.events.key(sdl3::keyboard::Keycode::Z).down {
        ctx.camera
            .change_zoom_around(1.0 * ctx.delta_ms as f64 / 1000.0, params.events.mouse_pos);
    }
    if params.events.key(sdl3::keyboard::Keycode::X).down {
        ctx.camera
            .change_zoom_around(-(ctx.delta_ms as f64) / 1000.0, params.events.mouse_pos);
    }

    pool.next.ecs.update_and_render(&mut ctx, &pool.prev.ecs)?;
    pool.prev.clone_from(&pool.next);

    Ok(true)
}
