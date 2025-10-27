use std::{
    cell::Cell,
    path::{Path, PathBuf},
};

use allocator_api2::{alloc::Allocator, vec::Vec};
use hashbrown::{DefaultHashBuilder, HashMap, HashSet};
use sdl3::image::LoadTexture;
use sdl3::render::{FRect, ScaleMode, Texture, TextureCreator};
use serde::{Deserialize, Serialize};

use crate::{
    animation::{Animation, AnimationCursor, Keyframe},
    serde::{is_empty, ordered_map},
    types::Id,
};

use super::manager::{Resource, ResourceError, ResourceLoader, ResourceManager};

/// Add this tag to frames that should be skipped
const TAG_NO_EXPORT: &str = "no-export";
const TAG_TILESET: &str = "tileset";

/// A rectangle as exported by Aseprite
#[derive(Serialize, Deserialize, Debug)]
struct AsepriteRect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl From<&AsepriteRect> for FRect {
    fn from(value: &AsepriteRect) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
            w: value.w as f32,
            h: value.h as f32,
        }
    }
}

impl From<AsepriteRect> for FRect {
    fn from(value: AsepriteRect) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
            w: value.w as f32,
            h: value.h as f32,
        }
    }
}

impl From<&FRect> for AsepriteRect {
    fn from(value: &FRect) -> Self {
        Self {
            x: value.x as u16,
            y: value.y as u16,
            w: value.w as u16,
            h: value.h as u16,
        }
    }
}

/// For use with serde's [with] attribute in `FRect`
mod rect_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub(crate) fn serialize<S>(value: &FRect, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ase: AsepriteRect = value.into();
        ase.serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<FRect, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ase = AsepriteRect::deserialize(deserializer)?;
        Ok(ase.into())
    }
}

/// A single cel in an Aseprite export
#[derive(Deserialize, Debug)]
struct AsepriteCel {
    #[serde(rename = "filename")]
    name: String,
    duration: u16,

    /// The position of this cel in the packed sprite map
    #[serde(rename = "frame")]
    sprite_tex_rect: AsepriteRect,

    /// The position of this cel in the original source
    #[serde(rename = "spriteSourceSize")]
    source_rect: AsepriteRect,

    #[serde(skip)]
    tags: HashSet<String>,
}

/// The animation direction
#[derive(Deserialize, Debug)]
enum AsepriteAnimDirection {
    #[serde(rename = "pingpong")]
    PingPong,
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "backward")]
    Backward,
}

/// An aseprite animation tag
#[derive(Deserialize, Debug)]
struct AsepriteAnim {
    name: String,
    direction: AsepriteAnimDirection,
}

/// A tagged cel
#[derive(Deserialize, Debug)]
struct AsepriteLayerTagCel {
    frame: u8,
    data: String,
}

/// An aseprite layer tag
#[derive(Deserialize, Debug)]
struct AsepriteLayerTag {
    name: String,
    #[serde(default)]
    cels: Vec<AsepriteLayerTagCel>,
    #[serde(default)]
    data: String,
}

impl AsepriteLayerTag {
    fn tags(&self) -> HashMap<&str, &str> {
        self.data
            .split(",")
            .map(|v| match v.split_once("=") {
                Some(v) => v,
                None => (v, ""),
            })
            .collect()
    }
}

/// The metadata of an Aseprite export
#[derive(Deserialize, Debug)]
struct AsepriteMeta {
    #[serde(rename = "frameTags")]
    animations: Vec<AsepriteAnim>,
    layers: Vec<AsepriteLayerTag>,
}

/// A raw sprite map JSON file that contains the metadata about a spritesheet.
///
/// This is in the same format as exported by Aseprite.
#[derive(Deserialize, Debug)]
struct AsepriteExport {
    meta: AsepriteMeta,
    #[serde(rename = "frames")]
    cels: Vec<AsepriteCel>,
}

/// An animation of frames within a sprite map
#[derive(Deserialize, Serialize, Debug)]
pub struct SpriteMapAnimation {
    // /// Maps layer names to the layer indexes
    // #[serde(serialize_with = "ordered_map")]
    // pub layers: HashMap<String, u8>,
    /// Each keyframe is a vec where the index is the layer index and the value is the cel index in the spritemap
    pub keyframes: Animation<Vec<u16>>,
}

/// Split a cel's name into (anim, frame_index, layer_name)
fn split_cel_name(name: &str) -> (&str, u8, &str) {
    let split: Vec<_> = name.splitn(3, "#").collect();
    assert!(
        split.len() == 3,
        "Frame ID should be in the format 'anim#frame_i#layer_name'"
    );

    let frame_i = split[1]
        .parse::<u8>()
        .expect("Frame index should be number");

    (split[0], frame_i, split[2])
}

impl SpriteMapAnimation {
    /// Update an animation cursor and return the cel indexes of the current frame
    pub fn update_cursor(&self, cursor: &mut AnimationCursor, now_ms: u64) -> Option<&[u16]> {
        cursor
            .update(now_ms, &self.keyframes)
            .map(|vec| vec.as_ref())
    }

    /// Update the cursor and loop if cursor is ended
    pub fn update_cursor_loop(&self, cursor: &mut AnimationCursor, now_ms: u64) -> &[u16] {
        match self.update_cursor(cursor, now_ms) {
            None => cursor.start(now_ms, &self.keyframes).as_ref(),
            Some(v) => v,
        }
    }

    /// Get a `SpriteMapAnimation` from the Aseprite export
    fn from_aseprite<A: Allocator + Clone>(
        allocator: A,
        fts: &AsepriteAnim,
        all_cels: &[AsepriteCel],
    ) -> Self {
        let mut seen_layers = HashSet::new_in(allocator.clone());
        let mut unique_layers = Vec::new_in(allocator.clone());

        // A map of: frame index -> (frame duration, layer cel indices)
        let mut frames: HashMap<u8, (u16, Vec<u16>), _, A> = HashMap::new_in(allocator.clone());
        for (sprite_map_i, cel) in all_cels.iter().enumerate() {
            let (anim_name, frame_i, layer_name) = split_cel_name(&cel.name);

            if anim_name != fts.name || cel.tags.contains(TAG_NO_EXPORT) {
                continue;
            }

            if !seen_layers.contains(layer_name) {
                unique_layers.push(layer_name);
                seen_layers.insert(layer_name);
            }

            let (frame_duration, frame_layers) = frames.entry(frame_i).or_default();

            *frame_duration = cel.duration;
            frame_layers.push(sprite_map_i as u16);
        }

        // Create a tuple of the keyframes from the hashmap, sort the tuples by index, then turn
        // them into a vec
        let mut keyframe_tuples: Vec<_> = frames
            .into_iter()
            .map(|(frame_i, (duration, layers))| (frame_i, Keyframe::new(duration, layers)))
            .collect();
        keyframe_tuples.sort_by(|a, b| a.0.cmp(&b.0));
        let mut keyframes: Vec<_> = keyframe_tuples
            .into_iter()
            .map(|(_i, layers)| layers)
            .collect();

        let full_frames = match fts.direction {
            AsepriteAnimDirection::Forward => keyframes,
            AsepriteAnimDirection::Backward => {
                keyframes.reverse();
                keyframes
            }
            AsepriteAnimDirection::PingPong => {
                let rev = Vec::from(&keyframes[1..keyframes.len() - 1]);
                keyframes.reverse();
                keyframes.extend(rev);
                keyframes
            }
        };

        SpriteMapAnimation {
            keyframes: Animation::new(full_frames),
        }
    }
}

/// A set of tiles, where each cel has implicit information about how to connect to
/// its neighbors depending on the position on the grid.
///
/// It stores the tile size and the cel where all the subtiles are stored. This cel
/// must be exactly 12x4 tiles in size, and the tiles must be arranged according to
/// the mask.
#[derive(Serialize, Deserialize)]
pub struct Tileset {
    grid_size: u8,
    cel: u16,
}

impl Resource<'_> for Tileset {
    type Id = Self;
}

/// A resolved tile set that can be used for rendering
pub struct ResolvedTileset<'texowner, 'tex> {
    pub tex: &'texowner Texture<'tex>,
    pub rect: FRect,
    pub grid_size: u8,
}

/// A single rectangle that represents the boundaries of a single image
/// within the spritemap (aka a Cel)
#[derive(Serialize, Deserialize)]
pub struct SpriteMapCel {
    /// The rect where this sprite is positioned in the global texture
    #[serde(with = "rect_serde")]
    pub tex_rect: FRect,
    /// The rect where this sprite is positioned relative to its source material
    #[serde(with = "rect_serde")]
    pub src_rect: FRect,
}

impl SpriteMapCel {
    pub fn new(tex_rect: FRect, src_rect: FRect) -> Self {
        Self { tex_rect, src_rect }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "sprite_map", tag = "type")]
struct SerializedSpriteMap {
    tex_path: String,
    cels: Vec<SpriteMapCel>,
    #[serde(
        default,
        skip_serializing_if = "is_empty",
        serialize_with = "ordered_map"
    )]
    animations: HashMap<String, SpriteMapAnimation>,
    #[serde(
        default,
        skip_serializing_if = "is_empty",
        serialize_with = "ordered_map"
    )]
    tilesets: HashMap<String, Tileset>,
}

// Holds many sprites in one single image. Each frame can be indexed from this map.
pub struct SpriteMap<'tex, A: Allocator> {
    id: Id<SpriteMapIdMarker>,
    pub tex: Texture<'tex>,
    pub cels: Vec<SpriteMapCel, A>,

    animation_names: HashMap<String, Id<SpriteMapAnimation>, DefaultHashBuilder, A>,
    animations: Vec<SpriteMapAnimation, A>,

    tileset_names: HashMap<String, Id<Tileset>, DefaultHashBuilder, A>,
    tilesets: Vec<Tileset, A>,
}

/// Just to use Ids without messing with lifetimes
pub struct SpriteMapIdMarker;

impl<'tex, A: Allocator + Clone> Resource<'tex> for SpriteMap<'tex, A> {
    type Id = SpriteMapIdMarker;
}

impl<'tex, A: Allocator + Clone> SpriteMap<'tex, A> {
    #[expect(clippy::type_complexity)]
    fn sort_names_and_ids<T>(
        allocator: &A,
        id: Id<SpriteMapIdMarker>,
        values: HashMap<String, T>,
    ) -> (HashMap<String, Id<T>, DefaultHashBuilder, A>, Vec<T, A>) {
        let mut sorted: Vec<_, A> = Vec::with_capacity_in(values.len(), allocator.clone());
        sorted.extend(values);
        sorted.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));

        let mut names: HashMap<_, _, _, A> = HashMap::new_in(allocator.clone());
        let mut val_vec: Vec<_, A> = Vec::with_capacity_in(sorted.len(), allocator.clone());
        for (i, (name, val)) in sorted.into_iter().enumerate() {
            let anim_id = Id::<T>::new_split(id.full() as u16, i as u16);

            names.insert(name, anim_id);
            val_vec.push(val);
        }

        (names, val_vec)
    }

    fn new_in(
        allocator: A,
        id: Id<SpriteMapIdMarker>,
        tex: Texture<'tex>,
        metadata: SerializedSpriteMap,
    ) -> Self {
        let (animation_names, animations) =
            Self::sort_names_and_ids(&allocator, id, metadata.animations);
        let (tileset_names, tilesets) = Self::sort_names_and_ids(&allocator, id, metadata.tilesets);

        let mut cels = Vec::with_capacity_in(metadata.cels.len(), allocator);
        cels.extend(metadata.cels);

        Self {
            id,
            tex,
            cels,
            animations,
            animation_names,
            tilesets,
            tileset_names,
        }
    }

    /// Get an animation's ID by its name or panic
    pub fn get_animation_id(&self, anim_name: &str) -> Id<SpriteMapAnimation> {
        *self
            .animation_names
            .get(anim_name)
            .unwrap_or_else(|| panic!("Invalid animation '{anim_name}'"))
    }

    /// Get an animation by ID or panic
    pub fn get_animation(&self, id: Id<SpriteMapAnimation>) -> &SpriteMapAnimation {
        debug_assert!(id.hi() == self.id.full() as u16);
        &self.animations[id.lo() as usize]
    }

    /// Get a tileset's ID by its name or panic
    pub fn get_tileset_id(&self, tileset_name: &str) -> Id<Tileset> {
        *self
            .tileset_names
            .get(tileset_name)
            .unwrap_or_else(|| panic!("Invalid tileset '{tileset_name}'"))
    }

    /// Get a tileset by ID or panic
    pub fn get_tileset<'this>(&'this self, id: Id<Tileset>) -> ResolvedTileset<'this, 'tex> {
        debug_assert!(id.hi() == self.id.full() as u16);
        let tileset = &self.tilesets[id.lo() as usize];

        let rect = self.cels[tileset.cel as usize].tex_rect;
        ResolvedTileset {
            tex: &self.tex,
            rect,
            grid_size: tileset.grid_size,
        }
    }
}

/// Convert a `.ase.json` file that gets exported from Aseprite into a `.res.json` file
/// that can be loaded by the game engine.
///
#[expect(clippy::disallowed_methods)]
/// This allocates memory.
pub fn ase_to_res<A: Allocator + Clone>(
    allocator: A,
    root_path: &Path,
    res_path: &Path,
) -> Result<(), String> {
    let full_path = root_path.join(res_path);
    let tex_path = res_path.with_extension("png");
    let tex_full_path = full_path.with_extension("png");
    let ase_path = full_path.with_extension("ase.json");
    let res_path = full_path.with_extension("res.json");

    if !tex_full_path.is_file() {
        return Err(format!(
            "No PNG found for sprite map ({}).",
            tex_full_path.to_str().unwrap_or("not a path")
        ));
    }
    if !ase_path.is_file() {
        return Err(format!(
            "No Aseprite JSON found for sprite map ({}).",
            ase_path.to_str().unwrap_or("not a path")
        ));
    }

    let ase_str = std::fs::read_to_string(ase_path)
        .or(Err("Failed to load Aseprite JSON file".to_string()))?;
    let mut metadata: AsepriteExport =
        serde_json::from_str(&ase_str).or(Err("Failed to deserialize Aseprite JSON file"))?;

    // Aseprite's frame tags are not ideal so we have to attach metadata to each cel manually
    {
        let mut frame_indexes_by_layer: HashMap<&str, u8> = HashMap::new();
        for cel in &mut metadata.cels {
            let (_, _, layer_name) = split_cel_name(&cel.name);

            let index_in_layer = frame_indexes_by_layer
                .entry(layer_name)
                .and_modify(|i| *i += 1)
                .or_insert(0);

            let Some(layer_tag) = metadata.meta.layers.iter().find(|i| i.name == layer_name) else {
                continue;
            };

            for cel_tag in &layer_tag.cels {
                if cel_tag.frame == *index_in_layer {
                    cel.tags = HashSet::from_iter(cel_tag.data.split(" ").map(|s| s.to_string()));
                }
            }
        }
    }

    // Find tileset layers
    let tilesets = {
        let tileset_layers: Vec<_> = metadata
            .meta
            .layers
            .iter()
            .filter(|l| l.tags().contains_key(TAG_TILESET))
            .collect();

        tileset_layers
            .into_iter()
            .map(|layer| {
                let cels: Vec<_> = metadata
                    .cels
                    .iter()
                    .enumerate()
                    .filter(|(_, cel)| {
                        let (_, _, cel_layer) = split_cel_name(&cel.name);
                        cel_layer == layer.name
                    })
                    .collect();

                assert_eq!(cels.len(), 1);

                let (cel_i, cel) = cels[0];

                let grid_size = layer
                    .tags()
                    .get("tile-size")
                    .map(|s| s.parse::<u8>().unwrap())
                    .unwrap();

                assert_eq!(cel.source_rect.w, 12 * grid_size as u16);
                assert_eq!(cel.source_rect.h, 4 * grid_size as u16);

                let tileset = Tileset {
                    cel: cel_i as u16,
                    grid_size,
                };

                (layer.name.clone(), tileset)
            })
            .collect()
    };

    let sm = SerializedSpriteMap {
        tex_path: tex_path.to_str().unwrap().to_owned(),
        cels: metadata
            .cels
            .iter()
            .map(|layer| {
                SpriteMapCel::new(
                    FRect::from(&layer.sprite_tex_rect),
                    FRect::from(&layer.source_rect),
                )
            })
            .collect(),
        animations: metadata
            .meta
            .animations
            .iter()
            .map(|anim| {
                (
                    anim.name.to_owned(),
                    SpriteMapAnimation::from_aseprite(allocator.clone(), anim, &metadata.cels),
                )
            })
            .collect(),
        tilesets,
    };

    let res_str = serde_json::to_string_pretty(&sm).map_err(|err| err.to_string())?;
    std::fs::write(res_path, res_str).map_err(|err| err.to_string())?;

    Ok(())
}

/// Loads a `SpriteMap` from a PNG and a JSON file
pub struct SpriteMapLoader<T, A: Allocator + Clone> {
    pub(super) root_path: PathBuf,

    allocator: A,
    sdl_loader: TextureCreator<T>,
    next_id: Cell<Id<SpriteMapIdMarker>>,
}

impl<T, A: Allocator + Clone> SpriteMapLoader<T, A> {
    pub fn new(
        allocator: A,
        sdl_loader: TextureCreator<T>,
        root_path: impl Into<PathBuf>,
    ) -> SpriteMapLoader<T, A> {
        Self {
            allocator,
            sdl_loader,
            root_path: root_path.into(),
            next_id: Cell::new(Id::<SpriteMapIdMarker>::new(0)),
        }
    }
}

impl<'l, 'tex, T, A: Allocator + Clone> ResourceLoader<'l, 'tex, SpriteMap<'tex, A>>
    for SpriteMapLoader<T, A>
where
    'l: 'tex,
{
    fn load(&'l self, path: &'_ str) -> Result<SpriteMap<'tex, A>, super::manager::ResourceError> {
        let full_path = self.root_path.join(path);
        let res_path = full_path.with_extension("res.json");

        let res_str = std::fs::read_to_string(res_path).or(Err(ResourceError::LoadFailed))?;
        let res: SerializedSpriteMap =
            serde_json::from_str(&res_str).or(Err(ResourceError::LoadFailed))?;

        let tex_full_path = self.root_path.join(&res.tex_path);

        let mut tex = self
            .sdl_loader
            .load_texture(tex_full_path)
            .or(Err(ResourceError::LoadFailed))?;
        tex.set_scale_mode(ScaleMode::Nearest);

        let sm = SpriteMap::new_in(self.allocator.clone(), self.next_id.get(), tex, res);

        let next = self.next_id.get().next();
        self.next_id.set(next);

        Ok(sm)
    }
}

/// A resource manager for `SpriteMap`
pub type SpriteMapManager<'tex, T, A> =
    ResourceManager<'tex, 'tex, SpriteMap<'tex, A>, SpriteMapLoader<T, A>, A>;
