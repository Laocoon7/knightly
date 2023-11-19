use bevy::prelude::*;
use bevy_fortress::prelude::*;

#[derive(Component, Reflect, Default, Clone, Copy, Deref, DerefMut)]
pub struct Position(pub Coord);
