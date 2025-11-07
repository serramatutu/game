use crate::{
    coords::{
        ScreenBox, ScreenPoint, ScreenRect, ScreenSize, WorldBox, WorldPoint, WorldRect, WorldSize,
    },
    math,
};

/// A camera positioned somewhere in the world
#[derive(Debug)]
pub struct Camera {
    zoom: f64,

    pub pos: WorldPoint,
    pub zoom_factor: f64,
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub world_to_pix: f64,
}

impl Camera {
    pub fn init(
        &mut self,
        min_zoom: f64,
        max_zoom: f64,
        pos: WorldPoint,
        zoom_factor: f64,
        world_to_pix_factor: f64,
    ) {
        self.pos = pos;
        self.zoom = 1.0;
        self.min_zoom = min_zoom;
        self.max_zoom = max_zoom;
        self.zoom_factor = zoom_factor;
        self.world_to_pix = world_to_pix_factor;
    }

    /// Set the zoom around the top left corner to a value, clamping it between the max and min allowed zoom
    pub fn set_zoom(&mut self, new_zoom: f64) {
        let zoom = math::clamp(new_zoom, self.min_zoom, self.max_zoom);
        self.zoom = zoom * self.zoom_factor;
    }

    /// Change the zoom around the top left corner by a delta, clamping it between the max and min allowed zoom.
    pub fn change_zoom(&mut self, delta: f64) {
        self.set_zoom(self.zoom / self.zoom_factor + delta);
    }

    /// Change the zoom around the given point by a delta, clamping it between the max and min allowed zoom.
    pub fn change_zoom_around(&mut self, delta: f64, point: ScreenPoint) {
        let wp_before = self.screen_to_world_point(&point);
        self.change_zoom(delta);
        let wp_after = self.screen_to_world_point(&point);
        self.pos += wp_before - wp_after;
    }

    /// Convert a point in the world to a point in the screen
    pub fn world_to_screen_point(&self, world: &WorldPoint) -> ScreenPoint {
        ScreenPoint::new(
            (world.x - self.pos.x) * self.zoom * self.world_to_pix,
            (world.y - self.pos.y) * self.zoom * self.world_to_pix,
        )
    }

    /// Convert a point in the screen to a point in the world
    pub fn screen_to_world_point(&self, screen: &ScreenPoint) -> WorldPoint {
        // TODO: WORLD TO PIX
        WorldPoint::new(
            screen.x / (self.zoom * self.world_to_pix) + self.pos.x,
            screen.y / (self.zoom * self.world_to_pix) + self.pos.y,
        )
    }

    /// Convert a size in the world to a point in the screen
    pub fn world_to_screen_size(&self, world: &WorldSize) -> ScreenSize {
        ScreenSize::new(
            world.width * self.zoom * self.world_to_pix,
            world.height * self.zoom * self.world_to_pix,
        )
    }

    /// Convert a size in the screen to a size in the world
    pub fn screen_to_world_size(&self, screen: &ScreenSize) -> WorldSize {
        WorldSize::new(
            screen.width / (self.zoom * self.world_to_pix),
            screen.height / (self.zoom * self.world_to_pix),
        )
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
            zoom_factor: 1.0,
            min_zoom: 1.0,
            max_zoom: 1.0,
            world_to_pix: 1.0,
        }
    }
}
