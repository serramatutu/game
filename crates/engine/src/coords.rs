use euclid::{Box2D, Point2D, Size2D};

pub struct ScreenSpace;
pub struct WorldSpace;

/// A point on the screen
pub type ScreenPoint = Point2D<f32, ScreenSpace>;

/// A size on the screen
pub type ScreenSize = Size2D<f32, ScreenSpace>;

/// A box on the screen
pub type ScreenBox = Box2D<f32, ScreenSpace>;

/// A point in the world
pub type WorldPoint = Point2D<f32, WorldSpace>;

/// A size in the world
pub type WorldSize = Size2D<f32, WorldSpace>;

/// A box in the world
pub type WorldBox = Box2D<f32, WorldSpace>;
