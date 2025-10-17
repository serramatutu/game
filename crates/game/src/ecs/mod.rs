use crate::{Ctx, with_components};
use anyhow::Result;
use heapless::Vec;
use paste::paste;
use std::iter::Iterator;

pub mod components;
pub use components::Entity;

pub mod systems;
use systems::SystemFn;

/// The sentinel value used to represent an entity not having a component or the null entity
pub const SENTINEL: usize = 0;

/// The max number of entities in the world at a time
const MAX_ENTITIES: usize = 8192;

/// All the registered ECS systems
///
/// They execute in order from top to bottom
const SYSTEMS: [SystemFn; 2] = [
    systems::navigation::follow::update_and_render,
    #[cfg(debug_assertions)]
    systems::debug::draw::update_and_render,
];

/// Holds all the entities, components and systems of the ECS.
///
/// INVARIANTS:
/// - The zero index of each component list is a dummy that is initialized with
///   default and should not belong to any entity.
/// - The zero index entity is a null entity that is never in the world.
#[derive(Clone, Default, Debug)]
pub struct Ecs {
    components: components::Components,
    entities: Vec<Entity, MAX_ENTITIES>,
}

impl Ecs {
    fn get_component<T: Copy>(components: &[(usize, T)], idx: usize) -> Option<T> {
        match idx {
            SENTINEL => None,
            _ => Some(components[idx].1),
        }
    }

    fn get_component_mut<T: Copy>(components: &mut [(usize, T)], idx: usize) -> Option<&mut T> {
        match idx {
            SENTINEL => None,
            _ => Some(&mut components[idx].1),
        }
    }

    pub fn get_entity(&self, id: usize) -> &Entity {
        // TODO: bounds check?
        &self.entities[id]
    }

    pub fn update_and_render<'gamestatic>(
        &mut self,
        ctx: &mut Ctx<'gamestatic, 'gamestatic>,
        prev: &Ecs,
    ) -> Result<()> {
        for sys in SYSTEMS {
            sys(ctx, prev, self)?;
        }
        Ok(())
    }
}

/// Helper to create a getter for a component type in the `Ecs` struct
macro_rules! impl_accessor {
    ($attr:ident, $type:ty) => {
        paste! {
            pub fn [<$attr _for>](&self, entity_id: usize) -> Option<$type> {
                let entity = self.get_entity(entity_id);
                Self::get_component(&self.components.$attr, entity.$attr)
            }

            pub fn [<$attr _for_unchecked>](&self, entity_id: usize) -> $type {
                let entity = self.get_entity(entity_id);
                debug_assert!(entity.$attr != SENTINEL, concat!("Tried to get '",stringify!($attr),"' attribute from entity that does not contain it."));
                let (_entity_id, component) = self.components.$attr[entity.$attr];
                component
            }

            pub fn [<$attr _for_mut>](&mut self, entity_id: usize) -> Option<&mut $type> {
                let attr_idx = self.get_entity(entity_id).$attr;
                Self::get_component_mut(&mut self.components.$attr, attr_idx)
            }

            pub fn [<set_ $attr _for>](&mut self, entity_id: usize, val: $type) {
                let attr_idx = self.get_entity(entity_id).$attr;
                debug_assert!(attr_idx != SENTINEL, concat!("Tried to set '",stringify!($attr),"' in entity that does not contain it."));
                self.components.$attr[attr_idx].1 = val;
            }

            // TODO: unset with component swap

            pub fn [<$attr _iter>](&self) -> impl Iterator<Item = &(usize, $type)> {
                // skip the sentinel
                self.components.$attr.iter().skip(1)
            }
        }
    };
}

/// Implement all accesssors for all the component types
macro_rules! impl_accessors {
    ( $( ($attr:ident, $type:ty) ),+ ) => {
        $(
            impl_accessor!($attr, $type);
        )*
    }
}

impl Ecs {
    with_components!(impl_accessors);
}

/// Impleent the entity spawner with all possible components
macro_rules! impl_entity_spawner {
    ( $( ($attr:ident, $type:ty) ),+ ) => {

        /// Constructs an entity by adding components to it
        #[derive(Default)]
        pub struct EntitySpawner {
            $(
                $attr: Option<$type>,
            )*
        }

        impl EntitySpawner {
            $(
                paste! {
                    #[doc= concat!("Add the default", stringify!($attr) , " value to the spawned entity")]
                    pub fn [<with_ $attr _default>](mut self) -> Self {
                        self.$attr = Some(Default::default());
                        self
                    }

                    #[doc= concat!("Add ", stringify!($attr) , " to the spawned entity")]
                    pub fn [<with_ $attr>](mut self, value: $type) -> Self {
                        self.$attr = Some(value);
                        self
                    }
                }
            )*

            pub fn new() -> Self {
                Default::default()
            }

            /// Spawn the entity into the ECS world
            pub fn spawn(&self, ecs: &mut Ecs) -> usize {
                let entity_id = ecs.entities.len();
                // FIXME: what to do when there are too many entities that get spawned? Fail
                // silently?
                ecs.entities.push(Default::default()).unwrap_or_else(|_| panic!("Too many entities"));
                let entity = &mut ecs.entities[entity_id];


                $(
                    if let Some(value) = self.$attr {
                        let component_id = ecs.components.$attr.len();
                        ecs.components.$attr.push((entity_id, value)).expect("Too many components. This is a definitely a bug.");
                        entity.$attr = component_id;
                    }
                )*

                entity_id
            }
        }
    }
}

with_components!(impl_entity_spawner);
