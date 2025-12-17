use derivative::Derivative;
use dyn_clone::DynClone;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::types::Id;

pub struct Entity;

/// The type used for entity IDs
pub type EntityId = Id<Entity>;

/// The sentinel value used to represent an entity not having a component or the null entity
const SENTINEL: u32 = 0;

impl EntityId {
    // The null entity
    pub fn null() -> Self {
        Self::new(SENTINEL)
    }
}

pub trait EcsAny: Any + DynClone {}

impl Debug for dyn EcsAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EcsAny")
    }
}

impl<T: 'static + Clone> EcsAny for T {}

impl Clone for Box<dyn EcsAny> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

#[derive(Clone, Debug)]
struct ComponentEntry {
    idx_within_entity: u8,
    components: Box<dyn EcsAny>,
}

impl ComponentEntry {
    fn components_as<T: 'static + Clone>(&self) -> Option<&Vec<(EntityId, T)>> {
        (self.components.as_ref() as &dyn Any).downcast_ref::<Vec<(EntityId, T)>>()
    }

    fn components_as_mut<T: 'static + Clone>(&mut self) -> Option<&mut Vec<(EntityId, T)>> {
        (self.components.as_mut() as &mut dyn Any).downcast_mut::<Vec<(EntityId, T)>>()
    }
}

pub struct ComponentInit<C: Copy + Hash + Eq>(C, Box<dyn EcsAny>);

impl<C: Copy + Hash + Eq> ComponentInit<C> {
    pub fn new<T: 'static + Clone>(key: C, default: T) -> Self {
        Self(key, Box::new(vec![(EntityId::null(), default)]))
    }
}

/// A template for an entity that can be spawned in the Ecs world.
#[derive(Clone, Default)]
pub struct EntityTemplate<C: Hash + Eq> {
    components: HashMap<C, Box<dyn EcsAny>>,
}

impl<C: Copy + Hash + Eq> EntityTemplate<C> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn set<T: EcsAny>(&mut self, key: C, value: T) {
        self.components.insert(key, Box::new(value));
    }

    pub fn with<T: EcsAny>(mut self, key: C, value: T) -> Self {
        self.set(key, value);
        self
    }
}

/// A function that spawns concrete components in entities based on a type erased dyn component.
///
/// The engine is not aware of what this type erased value is, and therefore it cannot downcast
/// the refs to the correct vec types.
pub type DynAddFn<C> = fn(&mut Ecs<C>, C, EntityId, &dyn EcsAny);

/// Holds all the entities, components and systems of the ECS.
///
/// INVARIANTS:
/// - The zero index of each component list is a dummy that is initialized with
///   default and should not belong to any entity.
/// - The zero index entity is a null entity that is never in the world.
#[derive(Derivative)]
#[derivative(Clone(clone_from = "true"))]
pub struct Ecs<C: Copy + Hash + Eq> {
    /// Maps component keys to their respective contiguous vecs of data
    components: HashMap<C, ComponentEntry>,
    /// An array that will be reinterpreted at runtime as Vec<[u32; NUM_COMPONENTS]>
    entities: Vec<u32>,
    /// A concrete function that adds a `&dyn EcsAny` component into the Ecs
    dyn_add: DynAddFn<C>,
}

impl<C: Copy + Hash + Eq + Debug> Ecs<C> {
    pub fn new(
        component_types: impl ExactSizeIterator<Item = ComponentInit<C>>,
        dyn_add: DynAddFn<C>,
    ) -> Self {
        let mut components = HashMap::with_capacity(component_types.len());
        for (i, init) in component_types.into_iter().enumerate() {
            let entry = ComponentEntry {
                idx_within_entity: i as u8,
                components: init.1,
            };

            let inserted = components.insert(init.0, entry);
            debug_assert!(inserted.is_none(), "Duplicate component key.");
        }

        debug_assert!(!components.is_empty(), "ECS with no components.");

        // TODO: reserve vec capacities
        Self {
            entities: std::iter::repeat_n(SENTINEL, components.len()).collect(),
            components,
            dyn_add,
        }
    }

    /// Spawn an entity into this ECS
    pub fn spawn(&mut self, template: &EntityTemplate<C>) -> EntityId {
        self.entities.resize_with(
            self.entities.len() + self.components.len(),
            Default::default,
        );
        debug_assert!(self.entities.len().is_multiple_of(self.components.len()));

        let entity_id = EntityId::new((self.entities.len() / self.components.len() - 1) as u32);

        for (key, value) in &template.components {
            (self.dyn_add)(self, *key, entity_id, value.as_ref());
        }

        entity_id
    }

    /// Get a reference to a component belonging to an entity
    pub fn get_ref<T: 'static + Clone>(&self, key: C, entity_id: EntityId) -> Option<&T> {
        debug_assert!(entity_id.0 != SENTINEL);

        let entry = self.components.get(&key).unwrap();
        let components_vec = entry.components_as::<T>().unwrap();

        let entity_idx = (entity_id.0 as usize) * self.components.len();
        let component_idx = self.entities[entity_idx + entry.idx_within_entity as usize];

        match component_idx {
            SENTINEL => None,
            _ => {
                let (got_entity_id, component) = &components_vec[component_idx as usize];
                debug_assert!(entity_id == *got_entity_id);
                Some(component)
            }
        }
    }

    /// Get a mutable reference of a component belonging to an entity
    pub fn get_mut<T: 'static + Clone>(&mut self, key: C, entity_id: EntityId) -> Option<&mut T> {
        debug_assert!(entity_id.0 != SENTINEL);

        let num_components = self.components.len();

        let entry = self.components.get_mut(&key).unwrap();
        let idx_within_entity = entry.idx_within_entity;

        let components_vec = entry.components_as_mut::<T>().unwrap();

        let entity_idx = (entity_id.0 as usize) * num_components;
        let component_idx = self.entities[entity_idx + idx_within_entity as usize];

        match component_idx {
            SENTINEL => None,
            _ => {
                let (got_entity_id, component) = &mut components_vec[component_idx as usize];
                debug_assert!(entity_id == *got_entity_id);
                Some(component)
            }
        }
    }

    /// Get a copy of a component belonging to an entity
    pub fn get<T: 'static + Copy>(&self, key: C, entity_id: EntityId) -> Option<T> {
        self.get_ref(key, entity_id).copied()
    }

    fn set_inner<T: 'static + Clone>(
        &mut self,
        key: C,
        entity_id: EntityId,
        value: T,
        add_if_not_exists: bool,
    ) {
        debug_assert!(entity_id.0 != SENTINEL);

        let num_components = self.components.len();
        let entry = self.components.get_mut(&key).unwrap();
        let component_in_entity =
            entity_id.0 as usize * num_components + entry.idx_within_entity as usize;
        let component_idx = self.entities[component_in_entity] as usize;

        let component_vec = entry.components_as_mut::<T>().unwrap();

        if component_idx as u32 == SENTINEL {
            if add_if_not_exists {
                component_vec.push((entity_id, value));
                self.entities[component_in_entity] = (component_vec.len() - 1) as u32;
            } else {
                debug_assert!(
                    false,
                    "Tried to set component in an entity that does not contain it."
                )
            }
        } else {
            component_vec[component_idx].1 = value;
        }
    }

    /// Set a component in the entity, adding it if it doesn't already have it
    pub fn add<T: 'static + Clone>(&mut self, key: C, entity_id: EntityId, value: T) {
        self.set_inner(key, entity_id, value, true);
    }

    /// Set a component in the entity, assuming it already has that component
    pub fn set<T: 'static + Clone>(&mut self, key: C, entity_id: EntityId, value: T) {
        self.set_inner(key, entity_id, value, false);
    }

    /// Unset a component for the given entity
    pub fn unset<T: 'static + Clone>(&mut self, key: C, entity_id: EntityId) -> Option<T> {
        debug_assert!(entity_id.0 != SENTINEL);

        let num_components = self.components.len();
        let entry = self.components.get_mut(&key).unwrap();
        let component_in_entity =
            entity_id.0 as usize * num_components + entry.idx_within_entity as usize;
        let component_idx = self.entities[component_in_entity] as usize;

        if component_idx == 0 {
            return None;
        }

        let component_vec = entry.components_as_mut::<T>().unwrap();
        let (removed_entity_id, removed) = component_vec.swap_remove(component_idx);
        debug_assert!(removed_entity_id == entity_id);
        self.entities[component_in_entity] = SENTINEL;

        // rewire indexes in the other entity that got swapped if it was not the last
        if component_idx < component_vec.len() {
            let (swapped_entity_id, _) = component_vec[component_idx];
            self.entities[swapped_entity_id.0 as usize + entry.idx_within_entity as usize] =
                component_idx as u32;
        }

        Some(removed)
    }

    /// Iterator over all entities with components of a given key
    pub fn iter_refs<T: 'static + Clone>(&self, key: C) -> impl Iterator<Item = (EntityId, &T)> {
        self.components
            .get(&key)
            .unwrap()
            .components_as::<T>()
            .unwrap()
            .iter()
            .filter_map(|(id, component)| {
                let id = *id;
                if id.0 == SENTINEL {
                    None
                } else {
                    Some((id, component))
                }
            })
    }

    /// Mutable iterator over all entities with components of a given key
    pub fn iter_mut<T: 'static + Clone>(
        &mut self,
        key: C,
    ) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.components
            .get_mut(&key)
            .unwrap()
            .components_as_mut::<T>()
            .unwrap()
            .iter_mut()
            .filter_map(|(id, component)| {
                let id = *id;
                if id.0 == SENTINEL {
                    None
                } else {
                    Some((id, component))
                }
            })
    }

    /// Iterator over copies of all entities with components of a given key
    pub fn iter<T: 'static + Copy>(&self, key: C) -> impl Iterator<Item = (EntityId, T)> {
        self.iter_refs(key)
            .map(|(entity_id, component)| (entity_id, *component))
    }
}
