//! Pathfinding, navigation etc

use crate::{Ctx, ecs::Ecs};

/// System to make an entity follow another
pub mod follow {
    use super::*;

    // TODO: follow speed as component?
    const SPEED_S: f32 = 5.0;

    pub fn update_and_render<'gs>(
        _ctx: &mut Ctx<'gs, 'gs>,
        prev: &Ecs,
        next: &mut Ecs,
    ) -> anyhow::Result<()> {
        for &(follower_id, target_id) in prev.follow_target_iter() {
            let follower_pos = prev.pos_for_unchecked(follower_id);
            let target_pos = prev.pos_for_unchecked(target_id);

            let diff = target_pos - follower_pos;
            let speed = SPEED_S.min(diff.length());

            next.set_vel_for(follower_id, diff.normalize() * speed);
        }
        Ok(())
    }
}
