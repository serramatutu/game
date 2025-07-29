use crate::{
    coords::{
        ScreenBox, ScreenPoint, ScreenRect, ScreenSize, WorldBox, WorldPoint, WorldRect, WorldSize,
    },
    math,
};

/// A camera positioned somewhere in the world
#[derive(Debug)]
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

    /// Change the zoom by a delta, clamping it between the max and min allowed zoom
    pub fn change_zoom(&mut self, delta: f32) {
        self.zoom = math::clamp(self.zoom + delta, self.min_zoom, self.max_zoom);
    }

    /// Set the zoom to a value, clamping it between the max and min allowed zoom
    pub fn set_zoom(&mut self, new_zoom: f32) {
        self.zoom = math::clamp(new_zoom, self.min_zoom, self.max_zoom);
    }

    /// Convert a point in the world to a point in the screen
    pub fn world_to_screen_point(&self, world: &WorldPoint) -> ScreenPoint {
        // TODO: zoom
        ScreenPoint::new(world.x - self.pos.x, world.y - self.pos.y)
    }

    /// Convert a point in the screen to a point in the world
    pub fn screen_to_world_point(&self, screen: &ScreenPoint) -> WorldPoint {
        // TODO: zoom
        WorldPoint::new(screen.x + self.pos.x, screen.y + self.pos.y)
    }

    /// Convert a size in the world to a point in the screen
    pub fn world_to_screen_size(&self, world: &WorldSize) -> ScreenSize {
        ScreenSize::new(world.width * self.zoom, world.height * self.zoom)
    }

    /// Convert a size in the screen to a size in the world
    pub fn screen_to_world_size(&self, screen: &ScreenSize) -> WorldSize {
        WorldSize::new(screen.width / self.zoom, screen.height / self.zoom)
    }

    /// Convert a box in the world to a box in the screen
    pub fn world_to_screen_box(&self, world: &WorldBox) -> ScreenBox {
        ScreenBox::new(
            self.world_to_screen_point(&world.min),
            self.world_to_screen_point(&world.max),
        )
    }

    /// Convert a box in the screen to a box in the world
    pub fn screen_to_world_box(&self, screen: &ScreenBox) -> WorldBox {
        WorldBox::new(
            self.screen_to_world_point(&screen.min),
            self.screen_to_world_point(&screen.max),
        )
    }

    /// Convert a rect in the world to a rect in the screen
    pub fn world_to_screen_rect(&self, world: &WorldRect) -> ScreenRect {
        ScreenRect::new(
            self.world_to_screen_point(&world.origin),
            self.world_to_screen_size(&world.size),
        )
    }

    /// Convert a rect in the screen to a rect in the world
    pub fn screen_to_world_rect(&self, screen: &ScreenRect) -> WorldRect {
        WorldRect::new(
            self.screen_to_world_point(&screen.origin),
            self.screen_to_world_size(&screen.size),
        )
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
