//! Pathfinding, navigation etc

use crate::{Ctx, ecs::Ecs};

pub fn update_and_render<'gs>(
    ctx: &mut Ctx<'gs, 'gs>,
    prev: &Ecs,
    next: &mut Ecs,
) -> anyhow::Result<()> {
    for &(entity_id, vel) in prev.vel_iter() {
        let delta_s = (ctx.delta_ms as f32) / 1000.0;
        let new_pos = prev.pos_for_unchecked(entity_id) + vel * delta_s;
        next.set_pos_for(entity_id, new_pos);
    }
    Ok(())
}
