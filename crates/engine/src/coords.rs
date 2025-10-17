use euclid::{Box2D, Point2D, Rect, Size2D, Vector2D};
use sdl3::render::FRect;

pub struct ScreenSpace;
pub struct WorldSpace;

/// A point on the screen
pub type ScreenPoint = Point2D<f32, ScreenSpace>;

/// A size on the screen
pub type ScreenSize = Size2D<f32, ScreenSpace>;

/// A vector on the screen
pub type ScreenVector = Vector2D<f32, ScreenSpace>;

/// A box on the screen
pub type ScreenBox = Box2D<f32, ScreenSpace>;

/// A rect on the screen
pub type ScreenRect = Rect<f32, ScreenSpace>;

/// A point in the world
pub type WorldPoint = Point2D<f32, WorldSpace>;

/// A size in the world
pub type WorldSize = Size2D<f32, WorldSpace>;

/// A vector in the world
pub type WorldVector = Vector2D<f32, WorldSpace>;

/// A box in the world
pub type WorldBox = Box2D<f32, WorldSpace>;

/// A rect in the world
pub type WorldRect = Rect<f32, WorldSpace>;

/// Conversion functions between coordinates in the different libraries
pub mod convert {
    use super::*;

    /// Convert a `ScreenRect` to an `FRect`
    pub fn screen_rect_to_sdl(rect: &ScreenRect) -> FRect {
        FRect {
            x: rect.origin.x,
            y: rect.origin.y,
            w: rect.size.width,
            h: rect.size.height,
        }
    }

    /// Convert a `ScreenBox` to an `FRect`
    pub fn screen_box_to_sdl(rect: &ScreenBox) -> FRect {
        FRect {
            x: rect.min.x,
            y: rect.min.y,
            w: rect.max.x - rect.min.x,
            h: rect.max.y - rect.min.y,
        }
    }
}
