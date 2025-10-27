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

pub fn load_resources<'r, A: Allocator + Clone>(res: &'r Resources<'r, A>) -> Result<ResourceIds, ResourceError> {
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
    for x in 0..10 {
        for y in 0..20 {
            terrain.tiles.set(x, y, Tile(true));
        }
    }

    terrain
}

pub fn spawn<'gs, A: Allocator + Clone>(ctx: &mut Ctx<'gs, A>, ecs: &mut Ecs<A>) -> usize {
    let res = ctx.resource_ids.terrain.as_ref().unwrap();

    let terrain = generate();

    EntitySpawner::new()
        .with_pos_default()
        .with_terrain(terrain)
        .spawn(ecs)
}
