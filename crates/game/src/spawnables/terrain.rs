use engine::{
    ecs::EntityTemplate,
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
        Ecs, EntityId,
        components::{Components, Pos, Terrain, Tile},
    },
};

pub struct ResourceIds {
    pub sprite: Id<SpriteMapIdMarker>,
    pub tileset: Id<Tileset>,
}

pub fn load_resources<'r>(res: &'r Resources<'r>) -> Result<ResourceIds, ResourceError> {
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

pub fn spawn<'gs>(_ctx: &mut Ctx<'gs>, ecs: &mut Ecs) -> EntityId {
    let terrain = generate();

    ecs.spawn(
        &EntityTemplate::new()
            .with(Components::Pos, Pos::default())
            .with(Components::Terrain, terrain),
    )
}
