use engine::coords::WorldPoint;

use crate::{
    Ctx,
    ecs::{
        Ecs, EntitySpawner,
        components::{SpriteAnim, SpriteAnims},
    },
};

pub fn spawn<'gs>(ctx: &mut Ctx<'gs>, ecs: &mut Ecs<'gs>) -> usize {
    let anims = SpriteAnims::from_array([SpriteAnim::from_sprite(ctx.resource_ids.zorb_sprite)]);

    let mut spawner = EntitySpawner::new()
        .with_pos(WorldPoint::new(400.0, 400.0))
        .with_sprite_anims(anims);

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
