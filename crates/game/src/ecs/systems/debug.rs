//! Debugging utilities

use crate::{Ctx, ecs::Ecs};

/// System to draw debug squares around entities
pub mod draw {
    use engine::coords::{self, WorldRect, WorldSize};

    use super::*;

    pub fn update_and_render<'gs>(
        ctx: &mut Ctx<'gs, 'gs>,
        prev: &Ecs,
        _next: &mut Ecs,
    ) -> anyhow::Result<()> {
        for &(entity_id, pos) in prev.pos_iter() {
            let Some(dbg_flags) = prev.debug_for(entity_id) else {
                continue;
            };

            let Some(box_color) = dbg_flags.box_color else {
                continue;
            };

            ctx.canvas.set_draw_color(box_color);
            ctx.canvas.draw_rect(coords::convert::screen_rect_to_sdl(
                &ctx.camera
                    .world_to_screen_rect(&WorldRect::new(pos, WorldSize::new(25.0, 25.0))),
            ))?;
        }
        Ok(())
    }
}
