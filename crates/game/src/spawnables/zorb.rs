use engine::coords::WorldPoint;

use crate::ecs::{Ecs, EntitySpawner};

pub fn spawn(ecs: &mut Ecs) -> usize {
    EntitySpawner::new()
        .with_pos(WorldPoint::new(400.0, 400.0))
        .with_vel_default()
        .spawn(ecs)
}
