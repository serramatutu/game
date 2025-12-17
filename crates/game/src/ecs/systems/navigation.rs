//! Pathfinding, navigation etc

use crate::{
    Ctx,
    ecs::Ecs,
    ecs::components::{Components, Follow, Pos},
};

/// System to make an entity follow another
pub mod follow {
    use super::*;

    // TODO: follow speed as component?
    const SPEED_S: f64 = 500.0;

    pub fn update_and_render<'gs>(
        ctx: &mut Ctx<'gs>,
        prev: &Ecs,
        next: &mut Ecs,
    ) -> anyhow::Result<()> {
        for (follower_id, follow) in prev.iter::<Follow>(Components::Follow) {
            let follower_pos = prev.get::<Pos>(Components::Pos, follower_id).unwrap();
            let target_pos = prev
                .get::<Pos>(Components::Pos, follow.target_entity)
                .unwrap();

            let diff = target_pos - follower_pos;
            let distance = diff.length();

            let speed_per_frame = SPEED_S * ctx.delta_s;

            // likely to overshoot on the next frame if the distance is less than the travel
            // distance per frame, so we just snap it to the target
            let new_pos = if distance < speed_per_frame * 1.5 {
                if follow.stop_after_arriving {
                    next.unset::<Follow>(Components::Follow, follower_id);
                }
                target_pos
            } else {
                follower_pos + diff.normalize() * speed_per_frame
            };

            next.set::<Pos>(Components::Pos, follower_id, new_pos);
        }
        Ok(())
    }
}
