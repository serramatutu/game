use allocator_api2::alloc::Allocator;
use derivative::Derivative;
use engine::{camera::Camera, resources::Resources};
use sdl3::render::WindowCanvas;

use crate::{ecs::Ecs, spawnables};

/// The map of known resource IDs
pub(crate) struct ResourceIds {
    pub zorb: Option<spawnables::zorb::ResourceIds>,
    pub terrain: Option<spawnables::terrain::ResourceIds>,
}

/// The alternating state between `update_and_render` calls
#[derive(Derivative)]
#[derivative(Clone(clone_from = "true"))]
pub(crate) struct State<A: Allocator + Clone> {
    // World objects
    pub ecs: Ecs<A>,
    pub zorb: usize,
    pub terrain: usize,
}

/// The global memory block that is used by the game
pub(crate) struct MemoryPool<A: Allocator + Clone> {
    // Object and resource management
    pub resource_ids: ResourceIds,

    pub prev: State<A>,
    pub next: State<A>,
}

/// A context object that can be passed around throughout the game
#[expect(dead_code)]
pub(crate) struct Ctx<'gs, A: Allocator + Clone> {
    pub allocator: A,
    pub canvas: &'gs mut WindowCanvas,
    pub camera: &'gs mut Camera,

    pub resources: &'gs mut Resources<'gs, A>,
    pub resource_ids: &'gs mut ResourceIds,

    pub now_ms: u64,
    pub delta_ms: u64,
    pub delta_s: f64,

    pub screen_w: u16,
    pub screen_h: u16,
}
