use std::ptr::NonNull;

use allocator_api2::alloc::Global as GlobalAllocator;

use sdl3::{EventPump, render::WindowCanvas};

pub struct InitParams {
    pub allocator: GlobalAllocator,
}

pub struct DropParams {
    pub allocator: GlobalAllocator,
    pub state: NonNull<[u8]>,
}

pub struct UpdateAndRenderParams<'a> {
    pub allocator: GlobalAllocator,

    pub event_pump: &'a mut EventPump,
    pub canvas: &'a mut WindowCanvas,

    pub now_ms: u64,
    pub delta_ms: u64,

    pub screen_w: u16,
    pub screen_h: u16,

    pub state: NonNull<[u8]>,
}
