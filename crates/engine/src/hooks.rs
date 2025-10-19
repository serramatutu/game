//! Global hooks which are used as the interface between the engine and the
//! engine and the game shared lib.

use std::ptr::NonNull;

use allocator_api2::alloc::Global as GlobalAllocator;

use sdl3::render::WindowCanvas;

use crate::{camera::Camera, events::Events, resources::Resources};

pub struct InitParams<'eng, 'res>
where
    'res: 'eng,
{
    pub allocator: GlobalAllocator,
    pub camera: &'eng mut Camera,
    pub resources: &'eng mut Resources<'res>,
}

pub struct DropParams {
    pub allocator: GlobalAllocator,
    // TODO: resources?
    pub memory: NonNull<[u8]>,
}

pub struct UpdateAndRenderParams<'eng, 'res>
where
    'res: 'eng,
{
    pub allocator: GlobalAllocator,

    pub events: &'eng mut Events,
    pub canvas: &'eng mut WindowCanvas,
    pub camera: &'eng mut Camera,
    pub resources: &'eng mut Resources<'res>,

    pub now_ms: u64,
    pub delta_ms: u64,

    pub screen_w: u16,
    pub screen_h: u16,

    pub memory: NonNull<[u8]>,
}
