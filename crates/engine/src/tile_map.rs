use derivative::Derivative;
use heapless::Vec;

pub enum NeighborPos {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BotLeft,
    Bot,
    BotRight,
}

/// The reason the chunk size is const is because Rust does not allow compile-time
/// arithmetics like [u8; N * N], and so it can't take a single `const N: usize` param.
///
/// See: https://github.com/rust-lang/rust/issues/76560#issue-697556349
const SIZE: usize = 256;
const SIZE_PAD: usize = SIZE + 2;
const SIZE_PAD_SQ: usize = SIZE_PAD * SIZE_PAD;

/// A mask that can be iterated over to find neighbors
const NEIGHBORS: [(NeighborPos, i16, i16); 8] = [
    (NeighborPos::TopLeft, -1, -1),
    (NeighborPos::Top, 0, -1),
    (NeighborPos::TopRight, 1, -1),
    (NeighborPos::Left, -1, 0),
    (NeighborPos::Right, 1, 0),
    (NeighborPos::BotLeft, -1, 1),
    (NeighborPos::Bot, 0, 1),
    (NeighborPos::BotRight, 1, 1),
];

/// Stores the tiles in a world and allows querying for them.
///
/// It stores a padding around the map to avoid bounds checking of
/// neighbors.
#[derive(Derivative)]
#[derivative(Clone(clone_from = "true"))]
#[derive(Debug)]
pub struct TileMap<Tile> {
    map: Vec<Tile, SIZE_PAD_SQ>,
}

impl<Tile: Default + Clone> Default for TileMap<Tile> {
    fn default() -> Self {
        let mut map = Vec::new();
        map.resize_default(SIZE_PAD_SQ).unwrap();

        Self { map }
    }
}

impl<Tile> TileMap<Tile> {
    /// Get the size of the tilemap along one axis
    pub fn size(&self) -> usize {
        SIZE
    }

    /// Get a tile ref
    pub fn get(&self, x: usize, y: usize) -> &Tile {
        &self.map[(x + 1) * SIZE + (y + 1)]
    }

    /// Get a tile mutable ref
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.map[(x + 1) * SIZE + (y + 1)]
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        *self.get_mut(x, y) = tile;
    }

    /// Iter over a tile's neighbors like so, where T is the tile:
    ///
    /// 0 1 2
    /// 3 T 4
    /// 5 6 7
    pub fn iter_neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (NeighborPos, &Tile)> {
        let x = x as i16;
        let y = y as i16;
        NEIGHBORS
            .into_iter()
            .map(move |(np, dx, dy)| (np, self.get((x + dx) as usize, (y + dy) as usize)))
    }
}
