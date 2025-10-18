use euclid::{Box2D, Point2D, Rect, Size2D, Vector2D};
use sdl3::render::FRect;

pub struct ScreenSpace;
pub struct WorldSpace;

/// A point on the screen
pub type ScreenPoint = Point2D<f64, ScreenSpace>;

/// A size on the screen
pub type ScreenSize = Size2D<f64, ScreenSpace>;

/// A vector on the screen
pub type ScreenVector = Vector2D<f64, ScreenSpace>;

/// A box on the screen
pub type ScreenBox = Box2D<f64, ScreenSpace>;

/// A rect on the screen
pub type ScreenRect = Rect<f64, ScreenSpace>;

/// A point in the world
pub type WorldPoint = Point2D<f64, WorldSpace>;

/// A size in the world
pub type WorldSize = Size2D<f64, WorldSpace>;

/// A vector in the world
pub type WorldVector = Vector2D<f64, WorldSpace>;

/// A box in the world
pub type WorldBox = Box2D<f64, WorldSpace>;

/// A rect in the world
pub type WorldRect = Rect<f64, WorldSpace>;

/// Conversion functions between coordinates in the different libraries
pub mod convert {
    use super::*;

    /// Convert a `ScreenRect` to an `FRect`
    pub fn screen_rect_to_sdl(rect: &ScreenRect) -> FRect {
        FRect {
            x: rect.origin.x as f32,
            y: rect.origin.y as f32,
            w: rect.size.width as f32,
            h: rect.size.height as f32,
        }
    }

    /// Convert a `ScreenBox` to an `FRect`
    pub fn screen_box_to_sdl(rect: &ScreenBox) -> FRect {
        FRect {
            x: rect.min.x as f32,
            y: rect.min.y as f32,
            w: (rect.max.x - rect.min.x) as f32,
            h: (rect.max.y - rect.min.y) as f32,
        }
    }
}
