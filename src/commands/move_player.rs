use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
};

use crate::{
    components::{tags::PlayerTag, Position},
    constants::{MAP_HEIGHT, MAP_WIDTH},
};

#[derive(Clone, Copy, Deref, DerefMut)]
pub struct MovePlayer(pub IVec2);

impl Command for MovePlayer {
    fn apply(self, world: &mut World) {
        let mut system_state = SystemState::<Query<&mut Position, With<PlayerTag>>>::from_world(world);

        let mut q_player = system_state.get_mut(world);

        for mut position in q_player.iter_mut() {
            position.x = (position.x + self.x).min(MAP_WIDTH - 1).max(0);
            position.y = (position.y + self.y).min(MAP_HEIGHT - 1).max(0);
        }

        system_state.apply(world);
    }
}
