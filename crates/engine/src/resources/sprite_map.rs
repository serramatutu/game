use std::collections::{HashMap, HashSet};
use std::path::Path;

use sdl3::image::LoadTexture;
use sdl3::render::{FRect, ScaleMode, Texture, TextureCreator};
use serde::Deserialize;

use crate::animation::{Animation, AnimationCursor, Keyframe};

use super::manager::{ResourceError, ResourceLoader, ResourceManager};

/// Add this tag to frames that should be skipped
const TAG_NO_EXPORT: &str = "no-export";

/// A rectangle as exported by Aseprite
#[derive(Deserialize, Debug)]
struct AsepriteRect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl AsepriteRect {
    /// Turn this into an SDL `FRect` for rendering
    fn to_sdl(&self) -> FRect {
        FRect {
            x: self.x as f32,
            y: self.y as f32,
            w: self.w as f32,
            h: self.h as f32,
        }
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
#[derive(Debug)]
pub struct SpriteMapAnimation {
    /// Maps layer names to the layer indexes
    pub layers: HashMap<String, u8>,
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
    fn from_aseprite(fts: &AsepriteAnim, all_cels: &[AsepriteCel]) -> Self {
        let mut seen_layers = HashSet::new();
        let mut unique_layers = Vec::new();

        // A map of: frame index -> (frame duration, layer cel indices)
        let mut frames: HashMap<u8, (u16, Vec<u16>)> = HashMap::new();
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

        let layer_ids = unique_layers
            .into_iter()
            .enumerate()
            .map(|(i, name)| (name.to_string(), i as u8))
            .collect();

        SpriteMapAnimation {
            layers: layer_ids,
            keyframes: Animation::new(full_frames),
        }
    }
}

pub struct SpriteMapCel {
    /// The rect where this sprite is positioned in the global texture
    pub tex_rect: FRect,
    /// The rect where this sprite is positioned relative to its source material
    pub src_rect: FRect,
}

impl SpriteMapCel {
    pub fn new(tex_rect: FRect, src_rect: FRect) -> Self {
        Self { tex_rect, src_rect }
    }
}

// Holds many sprites in one single image. Each frame can be indexed from this map.
pub struct SpriteMap<'sdlcanvas> {
    pub tex: Texture<'sdlcanvas>,
    pub cels: Vec<SpriteMapCel>,
    pub animations: HashMap<String, SpriteMapAnimation>,
}

impl<'sdlcanvas> SpriteMap<'sdlcanvas> {
    /// Get an animation by name or panic
    pub fn get_animation(&self, name: &str) -> &SpriteMapAnimation {
        self.animations
            .get(name)
            .unwrap_or_else(|| panic!("Invalid animation '{name}'"))
    }
}

/// Loads a `SpriteMap` from a PNG and a JSON file
pub struct SpriteMapLoader<T> {
    sdl_loader: TextureCreator<T>,
}

impl<T> SpriteMapLoader<T> {
    pub fn new(sdl_loader: TextureCreator<T>) -> SpriteMapLoader<T> {
        Self { sdl_loader }
    }
}

impl<'this, T> ResourceLoader<'this, SpriteMap<'this>> for SpriteMapLoader<T> {
    fn load(
        &'this self,
        full_path: &Path,
    ) -> Result<SpriteMap<'this>, super::manager::ResourceError> {
        let tex_path = full_path.with_extension("png");
        let meta_path = full_path.with_extension("json");

        assert!(
            tex_path.is_file(),
            "No PNG found for sprite map ({}).",
            tex_path.to_str().unwrap_or("not a path")
        );
        assert!(
            meta_path.is_file(),
            "No metadata found for sprite map ({}).",
            meta_path.to_str().unwrap_or("not a path")
        );

        let meta_str = std::fs::read_to_string(meta_path).or(Err(ResourceError::LoadFailed))?;
        let mut metadata: AsepriteExport =
            serde_json::from_str(&meta_str).or(Err(ResourceError::LoadFailed))?;

        // Aseprite's frame tags are not ideal so we have to attach metadata to each cel manually
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

        let mut tex = self
            .sdl_loader
            .load_texture(tex_path)
            .or(Err(ResourceError::LoadFailed))?;
        tex.set_scale_mode(ScaleMode::Nearest);

        let sm = SpriteMap {
            tex,
            cels: metadata
                .cels
                .iter()
                .map(|layer| {
                    SpriteMapCel::new(layer.sprite_tex_rect.to_sdl(), layer.source_rect.to_sdl())
                })
                .collect(),
            animations: metadata
                .meta
                .animations
                .iter()
                .map(|ft| {
                    (
                        ft.name.to_owned(),
                        SpriteMapAnimation::from_aseprite(ft, &metadata.cels),
                    )
                })
                .collect(),
        };

        Ok(sm)
    }
}

/// A resource manager for `SpriteMap`
pub type SpriteMapManager<'sdlcanvas, T> =
    ResourceManager<'sdlcanvas, SpriteMap<'sdlcanvas>, SpriteMapLoader<T>>;
