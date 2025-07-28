//! Global hooks which are used as the interface between the engine and the
//! engine and the game shared lib.

use std::ptr::NonNull;

use allocator_api2::alloc::Global as GlobalAllocator;

use sdl3::render::WindowCanvas;

use crate::{camera::Camera, events::Events};

pub struct InitParams<'a> {
    pub allocator: GlobalAllocator,
    pub camera: &'a mut Camera,
}

pub struct DropParams {
    pub allocator: GlobalAllocator,
    pub state: NonNull<[u8]>,
}

pub struct UpdateAndRenderParams<'a> {
    pub allocator: GlobalAllocator,

    pub events: &'a mut Events,
    pub canvas: &'a mut WindowCanvas,
    pub camera: &'a mut Camera,

    pub now_ms: u64,
    pub delta_ms: u64,

    pub screen_w: u16,
    pub screen_h: u16,

    pub state: NonNull<[u8]>,
}
