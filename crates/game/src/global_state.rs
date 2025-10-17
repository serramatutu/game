use allocator_api2::alloc::{Allocator, Global as GlobalAllocator};
use engine::{
    camera::Camera,
    resources::{Resources, sprite_map::SpriteMap},
    types::Id,
};
use sdl3::render::WindowCanvas;

use crate::ecs::Ecs;

/// The map of known resource IDs
#[derive(Clone)]
pub(crate) struct ResourceIds<'res> {
    pub zorb_sprite: Id<'res, SpriteMap<'res>>,
}

/// The state that gets persisted between calls of `update_and_render`
#[derive(Clone)]
pub(crate) struct State<'gamestatic> {
    // Object and resource management
    pub resource_ids: ResourceIds<'gamestatic>,

    // World objects
    pub ecs: Ecs,
    pub zorb: usize,
}

/// A context object that can be passed around throughout the game
#[expect(dead_code)]
pub(crate) struct Ctx<'gamestatic, 'caller, A: Allocator = GlobalAllocator> {
    pub allocator: A,
    pub canvas: &'gamestatic mut WindowCanvas,
    pub camera: &'gamestatic mut Camera,

    pub resources: &'caller Resources<'gamestatic>,
    pub resource_ids: &'caller ResourceIds<'gamestatic>,

    pub now_ms: u64,
    pub delta_ms: u64,

    pub screen_w: u16,
    pub screen_h: u16,
}
