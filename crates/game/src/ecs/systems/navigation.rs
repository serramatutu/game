//! Pathfinding, navigation etc

use crate::{Ctx, ecs::Ecs};

/// System to make an entity follow another
pub mod follow {
    use engine::coords::WorldVector;

    use super::*;

    // TODO: follow speed as component?
    const SPEED_S: f32 = 500.0;

    const NEAR_ENOUGH_M: f32 = 20.0;

    pub fn update_and_render<'gs>(
        _ctx: &mut Ctx<'gs, 'gs>,
        prev: &Ecs,
        next: &mut Ecs,
    ) -> anyhow::Result<()> {
        for &(follower_id, follow) in prev.follow_iter() {
            let follower_pos = prev.pos_for_unchecked(follower_id);
            let target_pos = prev.pos_for_unchecked(follow.target_entity);

            let diff = target_pos - follower_pos;
            let len = diff.length();

            // normalize without dividing by zero
            if (len - NEAR_ENOUGH_M).abs() > NEAR_ENOUGH_M {
                let vel = diff.normalize() * SPEED_S;
                next.set_vel_for(follower_id, vel);
            } else if follow.stop_after_arriving {
                next.unset_follow_for(follower_id);
                next.set_vel_for(follower_id, WorldVector::zero());
            }
        }
        Ok(())
    }
}
