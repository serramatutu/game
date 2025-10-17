//! Debugging utilities

use crate::{Ctx, ecs::Ecs};

/// System to draw debug squares around entities
pub mod draw {
    use engine::coords::{self, WorldRect, WorldSize};
    use sdl3::pixels::Color;

    use super::*;

    pub fn update_and_render<'gs>(
        ctx: &mut Ctx<'gs, 'gs>,
        prev: &Ecs,
        _next: &mut Ecs,
    ) -> anyhow::Result<()> {
        for &(_entity_id, pos) in prev.pos_iter() {
            ctx.canvas.set_draw_color(Color::RED);
            ctx.canvas.draw_rect(coords::convert::screen_rect_to_sdl(
                &ctx.camera
                    .world_to_screen_rect(&WorldRect::new(pos, WorldSize::new(25.0, 25.0))),
            ))?;
        }
        Ok(())
    }
}
