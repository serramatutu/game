//! Drawing, animation and rendering systems

use allocator_api2::alloc::Allocator;
use engine::coords::{WorldPoint, WorldRect, WorldSize, convert::screen_rect_to_sdl};

use crate::{Ctx, consts::WORLD_TO_PIXEL, ecs::Ecs};

pub fn update_and_render_terrain<'gs, A: Allocator + Clone>(
    ctx: &mut Ctx<'gs, A>,
    prev: &Ecs<A>,
    _next: &mut Ecs<A>,
) -> anyhow::Result<()> {
    let Some(res) = &ctx.resource_ids.terrain else {
        return Ok(());
    };

    let sprite_map = ctx.resources.sprites.get(res.sprite);
    let tileset = sprite_map.get_tileset(res.tileset);

    let block_width_world = WORLD_TO_PIXEL / tileset.grid_size as f64;

    for (_, terrain) in prev.terrain_iter() {
        // OPTIMIZE: use a pre-computed sprite that gets saved between frames
        for x in 1..terrain.tiles.size() {
            for y in 1..terrain.tiles.size() {
                let tile = terrain.tiles.get(x, y);
                if !tile.0 {
                    continue;
                }

                let solid_neighbors = terrain.tiles.filter_neighbors(x, y, |t| t.0);

                let tex_rect = tileset.tex_rect_for(solid_neighbors);

                let world_pos = WorldPoint::new(
                    (x * block_width_world as usize) as f64,
                    (y * block_width_world as usize) as f64,
                );
                let world_size = WorldSize::new(block_width_world, block_width_world);
                let world_rect = WorldRect::new(world_pos, world_size);
                let screen_box = ctx.camera.world_to_screen_rect(&world_rect);

                ctx.canvas.copy(
                    tileset.tex,
                    Some(tex_rect),
                    Some(screen_rect_to_sdl(&screen_box)),
                )?;
            }
        }
    }

    Ok(())
}

pub fn update_and_render_animations<'gs, A: Allocator + Clone>(
    ctx: &mut Ctx<'gs, A>,
    prev: &Ecs<A>,
    next: &mut Ecs<A>,
) -> anyhow::Result<()> {
    for (entity_id, prev_anims) in prev.sprite_anims_iter() {
        let entity_id = *entity_id;

        let pos = prev.pos_for_unchecked(entity_id);
        let next_anims = next.sprite_anims_for_mut_unchecked(entity_id);

        for (prev_anim, next_anim) in prev_anims.iter().zip(next_anims) {
            let sprite = ctx.resources.sprites.get(prev_anim.sprite);
            // FIXME: u64 animation IDs
            let anim = sprite.get_animation(prev_anim.anim);
            let layer_cels = anim.update_cursor_loop(&mut next_anim.cursor, ctx.now_ms);

            for cel_i in layer_cels.iter() {
                let cel = &sprite.cels[*cel_i as usize];

                // TODO: proper pixel to world conversion somewhere
                let world_pos =
                    WorldPoint::new(pos.x + cel.src_rect.x as f64, pos.y + cel.src_rect.y as f64);
                let world_size = WorldSize::new(cel.src_rect.w as f64, cel.src_rect.h as f64);
                let world_rect = WorldRect::new(world_pos, world_size);
                let screen_box = ctx.camera.world_to_screen_rect(&world_rect);

                ctx.canvas.copy(
                    &sprite.tex,
                    Some(cel.tex_rect),
                    Some(screen_rect_to_sdl(&screen_box)),
                )?;
            }
        }
    }
    Ok(())
}
