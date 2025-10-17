use engine::coords::WorldPoint;

use crate::ecs::{Ecs, EntitySpawner};

pub fn spawn(ecs: &mut Ecs) -> usize {
    let mut spawner = EntitySpawner::new()
        .with_pos(WorldPoint::new(400.0, 400.0))
        .with_vel_default();

    #[cfg(debug_assertions)]
    {
        use crate::ecs::components::DebugFlags;
        use sdl3::pixels::Color;

        spawner = spawner.with_debug(DebugFlags {
            box_color: Some(Color::RED),
        })
    }

    spawner.spawn(ecs)
}
