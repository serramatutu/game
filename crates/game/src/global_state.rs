use engine::{
    camera::Camera,
    resources::{Resources, sprite_map::SpriteMap},
    types::Id,
};
use sdl3::render::WindowCanvas;

use crate::Zorb;

/// The map of known resource IDs
#[derive(Clone)]
pub(crate) struct ResourceIds<'res> {
    pub zorb_sprite: Id<'res, SpriteMap<'res>>,
}

/// The state that gets persisted between calls of `update_and_render`
#[derive(Clone)]
pub(crate) struct State<'gamestatic> {
    pub zorb: Zorb,
    pub resource_ids: ResourceIds<'gamestatic>,
}

/// A context object that can be passed around throughout the game
#[expect(dead_code)]
pub(crate) struct Ctx<'gamestatic, 'caller> {
    pub canvas: &'gamestatic mut WindowCanvas,
    pub camera: &'gamestatic mut Camera,

    pub resources: &'caller Resources<'gamestatic>,
    pub resource_ids: &'caller ResourceIds<'gamestatic>,

    pub now_ms: u64,
    pub delta_ms: u64,

    pub screen_w: u16,
    pub screen_h: u16,

    pub prev_state: State<'gamestatic>,
}
