use allocator_api2::alloc::Allocator;
use engine::{
    resources::{
        Resources,
        manager::ResourceError,
        sprite_map::{SpriteMapIdMarker, Tileset},
    },
    types::Id,
};

use crate::{
    Ctx,
    ecs::{
        Ecs, EntitySpawner,
        components::{Terrain, Tile},
    },
};

pub struct ResourceIds {
    pub sprite: Id<SpriteMapIdMarker>,
    pub tileset: Id<Tileset>,
}

pub fn load_resources<'r, A: Allocator + Clone>(
    res: &'r Resources<'r, A>,
) -> Result<ResourceIds, ResourceError> {
    res.sprites
        .load("tiles/mask")?
        .and_then(|sprite_id, sprite| {
            Ok(ResourceIds {
                sprite: sprite_id,
                tileset: sprite.get_tileset_id("mask"),
            })
        })
}

fn generate() -> Terrain {
    let mut terrain = Terrain::default();
    for v in 1..51 {
        terrain.tiles.set(v, 1, Tile(true));
        terrain.tiles.set(v, 50, Tile(true));
        terrain.tiles.set(1, v, Tile(true));
        terrain.tiles.set(50, v, Tile(true));
        terrain.tiles.set(25, v, Tile(true));
        terrain.tiles.set(v, 25, Tile(true));
    }

    terrain
}

pub fn spawn<'gs, A: Allocator + Clone>(_ctx: &mut Ctx<'gs, A>, ecs: &mut Ecs<A>) -> usize {
    let terrain = generate();

    EntitySpawner::new()
        .with_pos_default()
        .with_terrain(terrain)
        .spawn(ecs)
}
