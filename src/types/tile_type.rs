use bevy::prelude::*;

#[derive(Component, Reflect, Default, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
}
