use bevy::prelude::*;

#[derive(Component, Reflect, Default, Clone, Copy, Deref, DerefMut)]
pub struct Position(pub IVec2);
