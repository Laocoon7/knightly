use bevy::prelude::*;

use super::{
    tags::{LeftMoverTag, PlayerTag},
    Position,
};

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        // Register Types for Bevy_Reflect
        app.register_type::<Position>();

        // Register Tags for Bevy_Reflect
        app.register_type::<PlayerTag>();
        app.register_type::<LeftMoverTag>();
    }
}
