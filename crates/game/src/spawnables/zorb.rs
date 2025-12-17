use engine::{
    coords::WorldPoint,
    ecs::EntityTemplate,
    resources::{
        Resources,
        manager::ResourceError,
        sprite_map::{SpriteMapAnimation, SpriteMapIdMarker},
    },
    types::Id,
};

use crate::{
    Ctx,
    ecs::{
        Ecs, EntityId,
        components::{Components, SpriteAnim, SpriteAnims},
    },
};

pub struct ResourceIds {
    pub sprite: Id<SpriteMapIdMarker>,
    pub anim_body_idle: Id<SpriteMapAnimation>,
    pub anim_body_walk: Id<SpriteMapAnimation>,
    pub anim_face_cute: Id<SpriteMapAnimation>,
}

pub fn load_resources<'r>(res: &'r Resources<'r>) -> Result<ResourceIds, ResourceError> {
    res.sprites.load("zorb")?.and_then(|sprite_id, sprite| {
        Ok(ResourceIds {
            sprite: sprite_id,
            anim_body_idle: sprite.get_animation_id("body:idle"),
            anim_body_walk: sprite.get_animation_id("body:walk"),
            anim_face_cute: sprite.get_animation_id("face:cute"),
        })
    })
}

pub fn spawn<'gs>(ctx: &mut Ctx<'gs>, ecs: &mut Ecs) -> EntityId {
    let res = ctx.resource_ids.zorb.as_ref().unwrap();
    let anims = SpriteAnims::from_array([
        SpriteAnim::from_sprite(res.sprite, res.anim_body_idle),
        SpriteAnim::from_sprite(res.sprite, res.anim_face_cute),
    ]);

    let mut template = EntityTemplate::new()
        .with(Components::Pos, WorldPoint::new(400.0, 400.0))
        .with(Components::SpriteAnims, anims);

    #[cfg(debug_assertions)]
    {
        use crate::ecs::components::DebugFlags;
        use sdl3::pixels::Color;

        template.set(
            Components::DebugFlags,
            DebugFlags {
                box_color: Some(Color::RED),
            },
        )
    }

    ecs.spawn(&template)
}
