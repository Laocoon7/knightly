use bevy_fortress::prelude::*;

pub const TILE_SIZE: f32 = 16.0;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const MAP_SIZE: Size = Size::new_u16(MAP_WIDTH as u16, MAP_HEIGHT as u16);

pub const LAYER_TERRAIN: f32 = 1.0;
pub const LAYER_ACTOR: f32 = 2.0;
