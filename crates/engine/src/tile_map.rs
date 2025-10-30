use std::ops::{BitAnd, BitOr};

use derivative::Derivative;
use heapless::Vec;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NeighborMask(pub u8);

impl NeighborMask {
    pub const EMPTY: u8 = 0;

    pub const TOP_LEFT: u8 = 1;
    pub const TOP: u8 = 2;
    pub const TOP_RIGHT: u8 = 4;
    pub const LEFT: u8 = 8;
    pub const RIGHT: u8 = 16;
    pub const BOT_LEFT: u8 = 32;
    pub const BOT: u8 = 64;
    pub const BOT_RIGHT: u8 = 128;
}

impl NeighborMask {
    pub fn is(&self, other: &NeighborMask) -> bool {
        self.0 & other.0 != 0
    }
}

impl BitOr<NeighborMask> for NeighborMask {
    type Output = Self;

    fn bitor(self, rhs: NeighborMask) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd<NeighborMask> for NeighborMask {
    type Output = Self;

    fn bitand(self, rhs: NeighborMask) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

/// The reason the chunk size is const is because Rust does not allow compile-time
/// arithmetics like [u8; N * N], and so it can't take a single `const N: usize` param.
///
/// See: https://github.com/rust-lang/rust/issues/76560#issue-697556349
const SIZE: usize = 256;
const SIZE_PAD: usize = SIZE + 2;
const SIZE_PAD_SQ: usize = SIZE_PAD * SIZE_PAD;

/// A mask that can be iterated over to find neighbors
const NEIGHBORS: [(u8, i16, i16); 8] = [
    (NeighborMask::TOP_LEFT, -1, -1),
    (NeighborMask::TOP, 0, -1),
    (NeighborMask::TOP_RIGHT, 1, -1),
    (NeighborMask::LEFT, -1, 0),
    (NeighborMask::RIGHT, 1, 0),
    (NeighborMask::BOT_LEFT, -1, 1),
    (NeighborMask::BOT, 0, 1),
    (NeighborMask::BOT_RIGHT, 1, 1),
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
    /// 1  2  4
    /// 8  T  16
    /// 32 64 128
    pub fn iter_neighbors(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (NeighborMask, &Tile)> {
        let x = x as i16;
        let y = y as i16;
        NEIGHBORS.into_iter().map(move |(np, dx, dy)| {
            (
                NeighborMask(np),
                self.get((x + dx) as usize, (y + dy) as usize),
            )
        })
    }

    /// Get a mask of neighbors that match the predicate
    pub fn filter_neighbors<F>(&self, x: usize, y: usize, predicate: F) -> NeighborMask
    where
        F: Fn(&Tile) -> bool,
    {
        self.iter_neighbors(x, y)
            .filter(|(_np, tile)| predicate(tile))
            .fold(NeighborMask(0), |acc, (np, _tile)| acc | np)
    }
}
