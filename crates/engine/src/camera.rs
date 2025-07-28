use crate::{
    coords::{ScreenSize, WorldPoint},
    math,
};

/// A camera positioned somewhere in the world
pub struct Camera {
    pub pos: WorldPoint,
    pub zoom: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl Camera {
    pub fn init(&mut self, min_zoom: f32, max_zoom: f32, pos: WorldPoint) {
        self.pos = pos;
        self.zoom = 1.0;
        self.min_zoom = min_zoom;
        self.max_zoom = max_zoom;
    }

    /// Get the maximum texture width for when this camera is zoomed out
    pub fn get_texture_size(&self, viewport_size: ScreenSize) -> ScreenSize {
        ScreenSize::new(
            viewport_size.width * self.min_zoom,
            viewport_size.height * self.min_zoom,
        )
    }

    /// Change the zoom by a delta, clamping it between the max and min allowed zoom
    pub fn change_zoom(&mut self, delta: f32) {
        self.zoom = math::clamp(self.zoom + delta, self.min_zoom, self.max_zoom)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: WorldPoint::origin(),
            zoom: 1.0,
            min_zoom: 1.0,
            max_zoom: 1.0,
        }
    }
}
