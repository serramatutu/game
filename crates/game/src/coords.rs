use engine::coords::{TilePoint, WorldPoint};

/// Conversion factor of world meters to pixels
pub const WORLD_TO_PIXEL: f64 = 3.0;

/// Conversion factor of world meters to tile coordinates
pub const WORLD_TO_TILE: f64 = 1.0;

/// Convert a world position into a tile position
pub fn world_to_tile(world: WorldPoint) -> TilePoint {
    TilePoint::new(
        (world.x * WORLD_TO_TILE) as u16,
        (world.y * WORLD_TO_TILE) as u16,
    )
}
