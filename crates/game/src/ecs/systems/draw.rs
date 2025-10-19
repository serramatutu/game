//! Drawing, animation and rendering systems

use engine::coords::{WorldPoint, WorldRect, WorldSize, convert::screen_rect_to_sdl};

use crate::{Ctx, consts::PIXEL_TO_WORLD, ecs::Ecs};

pub fn update_and_render<'gs>(
    ctx: &mut Ctx<'gs>,
    prev: &Ecs,
    next: &mut Ecs,
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
                let world_pos = WorldPoint::new(
                    pos.x + cel.src_rect.x as f64 * PIXEL_TO_WORLD,
                    pos.y + cel.src_rect.y as f64 * PIXEL_TO_WORLD,
                );
                let world_size = WorldSize::new(
                    cel.src_rect.w as f64 * PIXEL_TO_WORLD,
                    cel.src_rect.h as f64 * PIXEL_TO_WORLD,
                );
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
