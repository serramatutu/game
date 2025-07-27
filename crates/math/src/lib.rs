use euclid::Point2D;

pub struct ScreenSpace;
pub struct WorldSpace;

/// A point on the screen
pub type ScreenPoint = Point2D<f32, ScreenSpace>;

/// A point in the world
pub type WorldPoint = Point2D<f32, WorldSpace>;
