use engine::{
    coords::WorldPoint,
    resources::{
        Resources,
        manager::ResourceError,
        sprite_map::{SpriteMap, SpriteMapAnimation},
    },
    types::Id,
};

use crate::{
    Ctx,
    ecs::{
        Ecs, EntitySpawner,
        components::{SpriteAnim, SpriteAnims},
    },
};

pub struct ResourceIds<'res> {
    pub sprite: Id<SpriteMap<'res>>,
    pub anim_body_idle: Id<SpriteMapAnimation>,
    pub anim_body_walk: Id<SpriteMapAnimation>,
    pub anim_face_cute: Id<SpriteMapAnimation>,
}

pub fn load_resources<'res>(
    res: &'res mut Resources<'res>,
) -> Result<ResourceIds<'res>, ResourceError> {
    let (sprite_id, sprite) = res.sprites.load_get("zorb")?;

    let res = ResourceIds {
        sprite: sprite_id,
        anim_body_idle: sprite.get_animation_id("body:idle"),
        anim_body_walk: sprite.get_animation_id("body:walk"),
        anim_face_cute: sprite.get_animation_id("face:cute"),
    };

    Ok(res)
}

pub fn spawn<'gs>(ctx: &mut Ctx<'gs>, ecs: &mut Ecs<'gs>) -> usize {
    let res = ctx.resource_ids.zorb.as_ref().unwrap();
    let anims = SpriteAnims::from_array([
        SpriteAnim::from_sprite(res.sprite, res.anim_body_idle),
        SpriteAnim::from_sprite(res.sprite, res.anim_face_cute),
    ]);

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
