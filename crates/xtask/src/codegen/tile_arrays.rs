use engine::tile_map::NeighborMask;
use std::{fs::OpenOptions, io::BufWriter, io::Write};

/// The masks as layed out in sprite files
const MASKS: [(u8, (u8, u8)); 47] = [
    (NeighborMask::BOT, (0, 0)),
    (NeighborMask::BOT | NeighborMask::RIGHT, (1, 0)),
    (
        NeighborMask::BOT | NeighborMask::RIGHT | NeighborMask::LEFT,
        (2, 0),
    ),
    (NeighborMask::BOT | NeighborMask::LEFT, (3, 0)),
    (NeighborMask::TOP | NeighborMask::BOT, (0, 1)),
    (
        NeighborMask::TOP | NeighborMask::BOT | NeighborMask::RIGHT,
        (1, 1),
    ),
    (
        NeighborMask::TOP | NeighborMask::BOT | NeighborMask::RIGHT | NeighborMask::LEFT,
        (2, 1),
    ),
    (
        NeighborMask::TOP | NeighborMask::BOT | NeighborMask::LEFT,
        (3, 1),
    ),
    (NeighborMask::TOP, (0, 2)),
    (NeighborMask::TOP | NeighborMask::RIGHT, (1, 2)),
    (
        NeighborMask::TOP | NeighborMask::RIGHT | NeighborMask::LEFT,
        (2, 2),
    ),
    (NeighborMask::TOP | NeighborMask::LEFT, (3, 2)),
    (NeighborMask::EMPTY, (0, 3)),
    (NeighborMask::RIGHT, (1, 3)),
    (NeighborMask::RIGHT | NeighborMask::LEFT, (2, 3)),
    (NeighborMask::LEFT, (3, 3)),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::RIGHT
            | NeighborMask::LEFT
            | NeighborMask::BOT,
        (4, 0),
    ),
    (
        NeighborMask::LEFT | NeighborMask::RIGHT | NeighborMask::BOT | NeighborMask::BOT_RIGHT,
        (5, 0),
    ),
    (
        NeighborMask::LEFT | NeighborMask::RIGHT | NeighborMask::BOT | NeighborMask::BOT_LEFT,
        (6, 0),
    ),
    (
        NeighborMask::TOP_RIGHT
            | NeighborMask::TOP
            | NeighborMask::RIGHT
            | NeighborMask::LEFT
            | NeighborMask::BOT,
        (7, 0),
    ),
    (
        NeighborMask::TOP | NeighborMask::RIGHT | NeighborMask::BOT | NeighborMask::BOT_RIGHT,
        (4, 1),
    ),
    (
        NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_LEFT
            | NeighborMask::BOT
            | NeighborMask::BOT_RIGHT,
        (5, 1),
    ),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_LEFT
            | NeighborMask::BOT
            | NeighborMask::BOT_RIGHT,
        (6, 1),
    ),
    (
        NeighborMask::TOP | NeighborMask::LEFT | NeighborMask::BOT_LEFT | NeighborMask::BOT,
        (7, 1),
    ),
    (
        NeighborMask::TOP | NeighborMask::TOP_RIGHT | NeighborMask::RIGHT | NeighborMask::BOT,
        (4, 2),
    ),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT
            | NeighborMask::BOT_RIGHT,
        (5, 2),
    ),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_LEFT
            | NeighborMask::BOT,
        (6, 2),
    ),
    (
        NeighborMask::TOP_LEFT | NeighborMask::TOP | NeighborMask::LEFT | NeighborMask::BOT,
        (7, 2),
    ),
    (
        NeighborMask::TOP
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_LEFT
            | NeighborMask::BOT,
        (4, 3),
    ),
    (
        NeighborMask::TOP | NeighborMask::TOP_RIGHT | NeighborMask::LEFT | NeighborMask::RIGHT,
        (5, 3),
    ),
    (
        NeighborMask::TOP_LEFT | NeighborMask::TOP | NeighborMask::LEFT | NeighborMask::RIGHT,
        (6, 3),
    ),
    (
        NeighborMask::TOP
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT
            | NeighborMask::BOT_RIGHT,
        (7, 3),
    ),
    (
        NeighborMask::RIGHT | NeighborMask::BOT | NeighborMask::BOT_RIGHT,
        (8, 0),
    ),
    (
        NeighborMask::TOP
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_RIGHT
            | NeighborMask::BOT
            | NeighborMask::BOT_LEFT,
        (9, 0),
    ),
    (
        NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_RIGHT
            | NeighborMask::BOT
            | NeighborMask::BOT_LEFT,
        (10, 0),
    ),
    (
        NeighborMask::LEFT | NeighborMask::BOT_LEFT | NeighborMask::BOT,
        (11, 0),
    ),
    (
        NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::RIGHT
            | NeighborMask::BOT
            | NeighborMask::BOT_RIGHT,
        (8, 1),
    ),
    (
        NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_LEFT
            | NeighborMask::BOT,
        (9, 1),
    ),
    // SKIP, hole
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::LEFT
            | NeighborMask::BOT_LEFT
            | NeighborMask::BOT,
        (11, 1),
    ),
    (
        NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT
            | NeighborMask::BOT_RIGHT,
        (8, 2),
    ),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_LEFT
            | NeighborMask::BOT
            | NeighborMask::BOT_RIGHT,
        (9, 2),
    ),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT
            | NeighborMask::BOT_RIGHT,
        (10, 2),
    ),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::LEFT
            | NeighborMask::RIGHT
            | NeighborMask::BOT_LEFT
            | NeighborMask::BOT,
        (11, 2),
    ),
    (
        NeighborMask::TOP | NeighborMask::TOP_RIGHT | NeighborMask::RIGHT,
        (8, 3),
    ),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::RIGHT
            | NeighborMask::LEFT,
        (9, 3),
    ),
    (
        NeighborMask::TOP_LEFT
            | NeighborMask::TOP
            | NeighborMask::TOP_RIGHT
            | NeighborMask::RIGHT
            | NeighborMask::LEFT
            | NeighborMask::BOT,
        (10, 3),
    ),
    (
        NeighborMask::TOP_LEFT | NeighborMask::TOP | NeighborMask::LEFT,
        (11, 3),
    ),
];

/// Get the offset of a given mask. Can be any mask, including those not in MASKS.
fn offset_of(neighbor_mask: u8) -> (u8, u8) {
    let t = neighbor_mask & NeighborMask::TOP;
    let b = neighbor_mask & NeighborMask::BOT;
    let l = neighbor_mask & NeighborMask::LEFT;
    let r = neighbor_mask & NeighborMask::RIGHT;

    let tr_mask = if t != 0 && r != 0 {
        NeighborMask::TOP_RIGHT
    } else {
        NeighborMask::EMPTY
    };
    let tl_mask = if t != 0 && l != 0 {
        NeighborMask::TOP_LEFT
    } else {
        NeighborMask::EMPTY
    };
    let br_mask = if b != 0 && r != 0 {
        NeighborMask::BOT_RIGHT
    } else {
        NeighborMask::EMPTY
    };
    let bl_mask = if b != 0 && l != 0 {
        NeighborMask::BOT_LEFT
    } else {
        NeighborMask::EMPTY
    };

    let tr = neighbor_mask & tr_mask;
    let tl = neighbor_mask & tl_mask;
    let br = neighbor_mask & br_mask;
    let bl = neighbor_mask & bl_mask;

    let filter = t | b | l | r | tr | tl | br | bl;

    let filtered = neighbor_mask & filter;

    MASKS
        .iter()
        .find_map(|(mask, rect)| {
            if *mask == filtered {
                return Some(*rect);
            }
            None
        })
        .expect("mask should exist after filtering")
}

/// Generate the full offsets array from the neighbor masks in the map
fn gen_tile_offsets_array() -> Vec<(u8, (u8, u8))> {
    (0..=255u8).map(|m| (m, offset_of(m))).collect()
}

pub fn gen_code() {
    let mut file = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("./crates/engine/src/tile_map/mask.rs")
            .unwrap(),
    );

    let header = "// DO NOT EDIT: This code was autogenerated.

pub(crate) const MASKS: [(u8, u8); 256] = [";

    writeln!(file, "{header}").unwrap();

    gen_tile_offsets_array().iter().for_each(|&(_, (x, y))| {
        writeln!(file, "    ({x}, {y}),").unwrap();
    });

    writeln!(file, "];").unwrap();
}
