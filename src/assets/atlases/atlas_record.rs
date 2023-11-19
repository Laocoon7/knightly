use bevy::{asset::AssetPath, prelude::*};
use serde::Deserialize;

/// Defines how a `.atlas` file should be deserialized
#[derive(Deserialize)]
pub struct AtlasRecord {
    pub name: String,
    pub texture_path: AssetPath<'static>,

    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub padding: Option<Vec2>,
    pub offset: Option<Vec2>,
}
