use bevy::prelude::*;

use crate::components::Position;

pub fn update_positions(mut q_positions: Query<(&mut Transform, &Position), Changed<Position>>) {
    for (mut transform, position) in q_positions.iter_mut() {
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
    }
}
