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
pub(crate) struct State {
    // World objects
    pub ecs: Ecs,
    pub zorb: usize,
    pub terrain: usize,
}

/// The global memory block that is used by the game
pub(crate) struct MemoryPool {
    // Object and resource management
    pub resource_ids: ResourceIds,

    pub prev: State,
    pub next: State,
}

/// A context object that can be passed around throughout the game
#[expect(dead_code)]
pub(crate) struct Ctx<'gs> {
    pub canvas: &'gs mut WindowCanvas,
    pub camera: &'gs mut Camera,

    pub resources: &'gs mut Resources<'gs>,
    pub resource_ids: &'gs mut ResourceIds,

    pub now_ms: u64,
    pub delta_ms: u64,
    pub delta_s: f64,

    pub screen_w: u16,
    pub screen_h: u16,
}
