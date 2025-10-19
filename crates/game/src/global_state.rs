use allocator_api2::alloc::{Allocator, Global as GlobalAllocator};
use derivative::Derivative;
use engine::{
    camera::Camera,
    resources::{Resources, sprite_map::SpriteMap},
    types::Id,
};
use sdl3::render::WindowCanvas;

use crate::ecs::Ecs;

/// The map of known resource IDs
pub(crate) struct ResourceIds<'res> {
    pub zorb_sprite: Id<SpriteMap<'res>>,
}

/// The alternating state between `update_and_render` calls
#[derive(Derivative)]
#[derivative(Clone(clone_from = "true"))]
pub(crate) struct State<'res> {
    // World objects
    pub ecs: Ecs<'res>,
    pub zorb: usize,
}

/// The global memory block that is used by the game
pub(crate) struct MemoryPool<'res> {
    // Object and resource management
    pub resource_ids: ResourceIds<'res>,

    pub prev: State<'res>,
    pub next: State<'res>,
}

/// A context object that can be passed around throughout the game
#[expect(dead_code)]
pub(crate) struct Ctx<'gs, A: Allocator = GlobalAllocator> {
    pub allocator: A,
    pub canvas: &'gs mut WindowCanvas,
    pub camera: &'gs mut Camera,

    pub resources: &'gs Resources<'gs>,
    pub resource_ids: &'gs ResourceIds<'gs>,

    pub now_ms: u64,
    pub delta_ms: u64,
    pub delta_s: f64,

    pub screen_w: u16,
    pub screen_h: u16,
}
