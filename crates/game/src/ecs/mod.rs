use std::any::Any;

#[cfg(debug_assertions)]
use crate::ecs::components::DebugFlags;
use crate::{
    Ctx,
    ecs::components::{Components, Follow, Pos, SpriteAnims, Terrain},
};
mod systems;

#[macro_use]
pub mod components {
    use engine::{
        animation::AnimationCursor,
        coords::WorldPoint,
        ecs::EntityId,
        resources::sprite_map::{SpriteMapAnimation, SpriteMapIdMarker},
        tile_map::TileMap,
        types::Id,
    };
    use heapless::Vec;
    use sdl3::pixels::Color;

    pub type Pos = WorldPoint;

    #[derive(Clone, Default, Debug)]
    pub struct Tile(pub bool);

    #[derive(Clone, Default, Debug)]
    pub struct Terrain {
        pub tiles: TileMap<Tile>,
    }

    pub const MAX_ANIM_PER_ENTITY: usize = 4;

    #[derive(Copy, Clone, Default, Debug)]
    pub struct SpriteAnim {
        pub sprite: Id<SpriteMapIdMarker>,
        pub anim: Id<SpriteMapAnimation>,
        pub cursor: AnimationCursor,
    }

    impl SpriteAnim {
        pub fn from_sprite(sprite: Id<SpriteMapIdMarker>, anim: Id<SpriteMapAnimation>) -> Self {
            Self {
                sprite,
                anim,
                ..Default::default()
            }
        }
    }

    pub type SpriteAnims = Vec<SpriteAnim, MAX_ANIM_PER_ENTITY>;

    #[derive(Copy, Clone, Default, Debug)]
    pub struct Follow {
        pub stop_after_arriving: bool,
        pub target_entity: EntityId,
    }

    #[derive(Copy, Clone, Default, Debug)]
    pub struct DebugFlags {
        pub box_color: Option<Color>,
    }

    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Components {
        Pos,
        Tile,
        Terrain,
        SpriteAnims,
        Follow,
        #[cfg(debug_assertions)]
        DebugFlags,
    }

    #[macro_export]
    macro_rules! with_components {
        ($inner_macro:ident) => {
            $inner_macro! {
                $crate::ecs::components::DebugFlags,
                $crate::ecs::components::Follow,
                $crate::ecs::components::Pos,
                $crate::ecs::components::SpriteAnims,
                $crate::ecs::components::Terrain
            }
        };
    }
}

fn dyn_add(ecs: &mut Ecs, key: components::Components, entity_id: EntityId, value: &dyn EcsAny) {
    macro_rules! impl_dyn_add {
        ( $( $component:ty ),+ ) => {

            $(
                if let Some(value) = (value as &dyn Any).downcast_ref::<$component>() {
                    ecs.add::<$component>(key, entity_id, value.clone());
                    return;
                }
            )*

        };
    }

    with_components!(impl_dyn_add);
}

pub use engine::ecs::EntityId;
use engine::ecs::{ComponentInit, EcsAny};

pub type Ecs = engine::ecs::Ecs<Components>;

pub fn create_ecs() -> Ecs {
    let components = [
        #[cfg(debug_assertions)]
        ComponentInit::new(Components::DebugFlags, DebugFlags::default()),
        ComponentInit::new(Components::Follow, Follow::default()),
        ComponentInit::new(Components::Pos, Pos::default()),
        ComponentInit::new(Components::SpriteAnims, SpriteAnims::default()),
        ComponentInit::new(Components::Terrain, Terrain::default()),
    ];

    Ecs::new(components.into_iter(), dyn_add)
}

pub fn update_and_render<'gs>(
    ctx: &mut Ctx<'gs>,
    prev: &Ecs,
    next: &mut Ecs,
) -> anyhow::Result<()> {
    let systems = [
        systems::navigation::follow::update_and_render,
        systems::draw::update_and_render_terrain,
        systems::draw::update_and_render_animations,
        #[cfg(debug_assertions)]
        systems::debug::draw::update_and_render,
    ];

    for sys in systems {
        sys(ctx, prev, next)?
    }

    Ok(())
}
