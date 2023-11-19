use bevy::prelude::*;

use crate::{
    components::{tags::LeftMoverTag, Position},
    constants::MAP_WIDTH,
};

pub fn left_walker(mut q_lefty: Query<&mut Position, With<LeftMoverTag>>) {
    for mut position in q_lefty.iter_mut() {
        position.x -= 1;
        if position.x < 0 {
            position.x = MAP_WIDTH - 1;
        }
    }
}
