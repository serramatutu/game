use crate::{Ctx, with_components};
use allocator_api2::alloc::Allocator;
use anyhow::Result;
use derivative::Derivative;
use engine::types::Reset;
use heapless::Vec;
use paste::paste;
use std::{iter::Iterator, marker::PhantomData};

pub mod components;
pub use components::Entity;

pub mod systems;
use systems::SystemFn;

/// The sentinel value used to represent an entity not having a component or the null entity
pub const SENTINEL: usize = 0;

/// The max number of entities in the world at a time
const MAX_ENTITIES: usize = 8192;

/// Holds all the entities, components and systems of the ECS.
///
/// INVARIANTS:
/// - The zero index of each component list is a dummy that is initialized with
///   default and should not belong to any entity.
/// - The zero index entity is a null entity that is never in the world.
#[derive(Derivative, Debug)]
#[derivative(Clone(clone_from = "true"))]
pub struct Ecs<A: Allocator + Clone> {
    _pd: PhantomData<A>,
    components: components::Components,
    entities: Vec<Entity, MAX_ENTITIES>,
}

#[cfg(debug_assertions)]
const NUM_SYSTEMS: usize = 4;
#[cfg(not(debug_assertions))]
const NUM_SYSTEMS: usize = 3;

impl<A: Allocator + Clone> Ecs<A> {
    /// All the registered ECS systems
    ///
    /// They execute in order from top to bottom
    const SYSTEMS: [SystemFn<A>; NUM_SYSTEMS] = [
        systems::navigation::follow::update_and_render,
        systems::draw::update_and_render_terrain,
        systems::draw::update_and_render_animations,
        #[cfg(debug_assertions)]
        systems::debug::draw::update_and_render,
    ];

    fn get_component<T: Copy>(components: &[(usize, T)], idx: usize) -> Option<T> {
        match idx {
            SENTINEL => None,
            _ => Some(components[idx].1),
        }
    }

    fn get_component_ref<T>(components: &[(usize, T)], idx: usize) -> Option<&T> {
        match idx {
            SENTINEL => None,
            _ => Some(&components[idx].1),
        }
    }

    fn get_component_mut<T>(components: &mut [(usize, T)], idx: usize) -> Option<&mut T> {
        match idx {
            SENTINEL => None,
            _ => Some(&mut components[idx].1),
        }
    }

    pub fn update_and_render<'gs>(&mut self, ctx: &mut Ctx<'gs, A>, prev: &Ecs<A>) -> Result<()> {
        for sys in Self::SYSTEMS {
            sys(ctx, prev, self)?;
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> Reset for Ecs<A> {
    fn reset(&mut self) {
        self.components.reset();
        self.entities.resize(1, Default::default()).unwrap();
    }
}

macro_rules! impl_accessor_copy {
    // cheap_copy
    ($attr:ident, $type:ty, true, $max:tt) => {
        paste! {
            #[allow(dead_code)]
            pub fn [<$attr _for>](&self, entity_id: usize) -> Option<$type> {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = self.entities[entity_id].$attr;
                Self::get_component(&self.components.$attr, attr_idx)
            }

            #[allow(dead_code)]
            pub fn [<$attr _for_unchecked>](&self, entity_id: usize) -> $type {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = self.entities[entity_id].$attr;
                debug_assert!(attr_idx != SENTINEL, concat!("Tried to get '",stringify!($attr),"' attribute from entity that does not contain it."));
                self.components.$attr[attr_idx].1
            }
        }
    };

    // expensive copy
    ($attr:ident, $type:ty, false, $max:tt) => {
        paste! {
            #[allow(dead_code)]
            pub fn [<$attr _for>](&self, entity_id: usize) -> Option<&$type> {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = self.entities[entity_id].$attr;
                Self::get_component_ref(&self.components.$attr, attr_idx)
            }

            #[allow(dead_code)]
            pub fn [<$attr _for_unchecked>](&self, entity_id: usize) -> &$type {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = self.entities[entity_id].$attr;
                debug_assert!(attr_idx != SENTINEL, concat!("Tried to get '",stringify!($attr),"' attribute from entity that does not contain it."));
                &self.components.$attr[attr_idx].1
            }
        }
    };
}

/// Helper to create a getter for a component type in the `Ecs` struct
macro_rules! impl_accessor {
    ($attr:ident, $type:ty, $cheap_copy:tt, $max:tt) => {
        paste! {
            fn [<push_ $attr _unchecked>]<const N: usize>(components: &mut Vec<(usize, $type), N>, entity_id: usize, entity: &mut Entity, value: $type) {
                debug_assert!(entity_id != SENTINEL);

                let component_id = components.len();
                components.push((entity_id, value)).expect("Too many components.");
                entity.$attr = component_id;
            }

            #[allow(dead_code)]
            pub fn [<$attr _for_mut>](&mut self, entity_id: usize) -> Option<&mut $type> {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = self.entities[entity_id].$attr;
                Self::get_component_mut(&mut self.components.$attr, attr_idx)
            }

            #[allow(dead_code)]
            pub fn [<$attr _for_mut_unchecked>](&mut self, entity_id: usize) -> &mut $type {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = self.entities[entity_id].$attr;
                debug_assert!(attr_idx != SENTINEL, concat!("Tried to get mut '",stringify!($attr),"' in entity that does not contain it."));
                &mut self.components.$attr[attr_idx].1
            }

            #[allow(dead_code)]
            pub fn [<set_ $attr _for>](&mut self, entity_id: usize, val: $type) {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = self.entities[entity_id].$attr;
                debug_assert!(attr_idx != SENTINEL, concat!("Tried to set '",stringify!($attr),"' in entity that does not contain it."));
                self.components.$attr[attr_idx].1 = val;
            }

            #[allow(dead_code)]
            pub fn [<unset_ $attr _for>](&mut self, entity_id: usize) -> $type {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = {
                    let entity = &mut self.entities[entity_id];
                    let attr_idx = entity.$attr;
                    debug_assert!(attr_idx != SENTINEL, concat!("Tried to unset '",stringify!($attr),"' but it's already unset."));

                    entity.$attr = SENTINEL;
                    attr_idx
                };

                let (removed_entity_id, removed) = self.components.$attr.swap_remove(attr_idx);
                debug_assert!(removed_entity_id == entity_id);

                // rewire indexes in the other entity that got swapped if it was not the last
                if attr_idx < self.components.$attr.len() {
                    let (swapped_entity_id, _) = self.components.$attr[attr_idx];
                    self.entities[swapped_entity_id].$attr = attr_idx;
                }

                removed
            }

            #[allow(dead_code)]
            pub fn [<overwrite_ $attr _for>](&mut self, entity_id: usize, val: $type) {
                debug_assert!(entity_id != SENTINEL);

                let attr_idx = self.entities[entity_id].$attr;
                match attr_idx {
                    SENTINEL => {
                        Self::[<push_ $attr _unchecked>](&mut self.components.$attr, entity_id, &mut self.entities[entity_id], val);
                    }
                    _ => {
                        self.components.$attr[attr_idx].1 = val;
                    }
                }
            }

            #[allow(dead_code)]
            pub fn [<$attr _iter>](&self) -> impl Iterator<Item = &(usize, $type)> {
                // skip the sentinel
                self.components.$attr.iter().skip(1)
            }
        }
    };
}

/// Implement all accesssors for all the component types
macro_rules! impl_accessors {
    ( $( ($attr:ident, $type:ty, $cheap_copy:tt, $max:tt) ),+ ) => {
        $(
            impl_accessor_copy!($attr, $type, $cheap_copy, $max);
            impl_accessor!($attr, $type, $cheap_copy, $max);
        )*
    }
}

impl<A: Allocator + Clone> Ecs<A> {
    with_components!(impl_accessors);
}

/// Impleent the entity spawner with all possible components
macro_rules! impl_entity_spawner {
    ( $( ($attr:ident, $type:ty, $cheap_copy:tt, $max:tt) ),+ ) => {

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
                    #[allow(dead_code)]
                    pub fn [<with_ $attr _default>](mut self) -> Self {
                        self.$attr = Some(Default::default());
                        self
                    }

                    #[doc= concat!("Add ", stringify!($attr) , " to the spawned entity")]
                    #[allow(dead_code)]
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
            pub fn spawn<A: Allocator + Clone>(self, ecs: &mut Ecs<A>) -> usize {
                let entity_id = ecs.entities.len();
                // FIXME: what to do when there are too many entities that get spawned? Fail
                // silently?
                ecs.entities.push(Default::default()).unwrap_or_else(|_| panic!("Too many entities"));
                let entity = &mut ecs.entities[entity_id];

                $(
                    if let Some(value) = self.$attr {
                        paste! {
                            Ecs::<A>::[<push_ $attr _unchecked>](&mut ecs.components.$attr, entity_id, entity, value);
                        }
                    }
                )*

                entity_id
            }
        }
    }
}

with_components!(impl_entity_spawner);
