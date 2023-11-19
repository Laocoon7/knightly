use bevy::{asset::AssetPath, prelude::*};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ActorRecord {
    pub name: String,
    pub atlas_path: AssetPath<'static>,
    pub index: usize,
    pub color: Color,
}
