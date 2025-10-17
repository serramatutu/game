use engine::coords::WorldPoint;
use heapless::Vec;

use crate::ecs::MAX_ENTITIES;

pub type Pos = WorldPoint;
pub type Vel = WorldPoint;
pub type FollowTarget = usize;

/// Creates a list of (attr, type) for all possible components and calls the given macro.
///
/// This can be used in other macros to generate code.
#[macro_export]
macro_rules! with_components {
    ($inner_macro:ident) => {
        $inner_macro! {
            (pos, $crate::ecs::components::Pos),
            (vel, $crate::ecs::components::Vel),
            (follow_target, $crate::ecs::components::FollowTarget)
        }
    };
}

/// Implement the entity struct based on our list of components
macro_rules! impl_entity {
    ( $( ($attr:ident, $type:ty) ),+ ) => {

        /// An entity with optionally attached components
        ///
        /// Component index 0 means the entity does not have that component
        /// attached to it.
        #[derive(Clone, Default)]
        pub struct Entity {
            $(
                pub $attr: usize,
            )*
        }

    };
}

/// Implement the components struct based on our list of components
macro_rules! impl_components {
    ( $( ($attr:ident, $type:ty) ),+ ) => {

        /// Exhaustive list of all possible components of an entity
        ///
        /// The 0th value of every vec is a sentinel value that should not
        /// be used
        #[derive(Clone)]
        pub struct Components {
            $(
                pub $attr: Vec<(usize, $type), MAX_ENTITIES>,
            )*
        }

        impl Components {
            pub fn new() -> Self {
                Self {
                    $(
                        $attr: sentinel_vec(),
                    )*
                }
            }
        }

    };
}

/// Create a vec with one sentinel value at the beginning
fn sentinel_vec<T: Default>() -> Vec<T, MAX_ENTITIES> {
    let mut v = Vec::new();
    v.push(T::default())
        .unwrap_or_else(|_| panic!("Programming error."));
    v
}

with_components!(impl_entity);
with_components!(impl_components);
