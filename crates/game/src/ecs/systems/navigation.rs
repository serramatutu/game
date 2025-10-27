//! Pathfinding, navigation etc

use crate::{Ctx, ecs::Ecs};

/// System to make an entity follow another
pub mod follow {
    use allocator_api2::alloc::Allocator;

    use super::*;

    // TODO: follow speed as component?
    const SPEED_S: f64 = 500.0;

    pub fn update_and_render<'gs, A: Allocator + Clone>(
        ctx: &mut Ctx<'gs, A>,
        prev: &Ecs<A>,
        next: &mut Ecs<A>,
    ) -> anyhow::Result<()> {
        for &(follower_id, follow) in prev.follow_iter() {
            let follower_pos = prev.pos_for_unchecked(follower_id);
            let target_pos = prev.pos_for_unchecked(follow.target_entity);

            let diff = target_pos - follower_pos;
            let distance = diff.length();

            let speed_per_frame = SPEED_S * ctx.delta_s;

            // likely to overshoot on the next frame if the distance is less than the travel
            // distance per frame, so we just snap it to the target
            let new_pos = if distance < speed_per_frame * 1.5 {
                if follow.stop_after_arriving {
                    next.unset_follow_for(follower_id);
                }
                target_pos
            } else {
                follower_pos + diff.normalize() * speed_per_frame
            };

            next.set_pos_for(follower_id, new_pos);
        }
        Ok(())
    }
}
