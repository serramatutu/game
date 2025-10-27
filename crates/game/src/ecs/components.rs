use derivative::Derivative;
use engine::{
    animation::AnimationCursor,
    coords::WorldPoint,
    resources::sprite_map::{SpriteMapAnimation, SpriteMapIdMarker},
    tile_map::TileMap,
    types::{Id, Reset},
};
use heapless::Vec;
use sdl3::pixels::Color;

use crate::ecs::MAX_ENTITIES;

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
    pub target_entity: usize,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct DebugFlags {
    pub box_color: Option<Color>,
}

/// Creates a list of (attr, type) for all possible components and calls the given macro.
///
/// This can be used in other macros to generate code.
#[macro_export]
macro_rules! with_components {
    ($inner_macro:ident) => {
        $inner_macro! {
            (pos, $crate::ecs::components::Pos, true, MAX_ENTITIES),
            (follow, $crate::ecs::components::Follow, true, 64),
            // FIXME: remove debug flags in prod build
            (debug, $crate::ecs::components::DebugFlags, true, MAX_ENTITIES),
            (sprite_anims, $crate::ecs::components::SpriteAnims, false, MAX_ENTITIES),
            (terrain, $crate::ecs::components::Terrain, false, 2)
        }
    };
}

/// Implement the entity struct based on our list of components
macro_rules! impl_entity {
    ( $( ($attr:ident, $type:ty, $cheap_copy:tt, $max:tt) ),+ ) => {

        /// An entity with optionally attached components
        ///
        /// Component index 0 means the entity does not have that component
        /// attached to it.
        #[derive(Copy, Clone, Default, Debug)]
        pub struct Entity {
            $(
                pub $attr: usize,
            )*
        }

    };
}

/// Implement the components struct based on our list of components
macro_rules! impl_components {
    ( $( ($attr:ident, $type:ty, $cheap_copy:tt, $max:tt) ),+ ) => {

        /// Exhaustive list of all possible components of an entity
        ///
        /// The 0th value of every vec is a sentinel value that should not
        /// be used
        #[derive(Derivative, Debug)]
        #[derivative(Clone(clone_from="true"))]
        pub struct Components {
            $(
                pub $attr: Vec<(usize, $type), $max>,
            )*
        }

        impl<'res> Components {
            pub fn new() -> Self {
                Self {
                    $(
                        $attr: sentinel_vec(),
                    )*
                }
            }
        }

        impl Reset for Components {
            fn reset(&mut self) {
                $(
                    self.$attr.resize(1, Default::default()).unwrap();
                )*
            }
        }
    };
}

/// Create a vec with one sentinel value at the beginning
fn sentinel_vec<T: Default, const N: usize>() -> Vec<T, N> {
    let mut v = Vec::new();
    v.push(T::default())
        .unwrap_or_else(|_| panic!("Programming error."));
    v
}

with_components!(impl_entity);

with_components!(impl_components);

impl Default for Components {
    fn default() -> Self {
        Components::new()
    }
}
