use bevy::prelude::*;

use super::{tags::PlayerTag, Position};
use crate::types::TileType;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        // Register Types for Bevy_Reflect
        app.register_type::<Position>();

        // Register Tags for Bevy_Reflect
        app.register_type::<PlayerTag>();

        app.register_type::<TileType>();
    }
}
