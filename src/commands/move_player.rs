use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
};
use bevy_fortress::prelude::*;

use crate::{
    components::{tags::PlayerTag, Position},
    constants::{MAP_HEIGHT, MAP_WIDTH},
    types::TileType,
};

#[derive(Clone, Copy, Deref, DerefMut)]
pub struct MovePlayer(pub Coord);

impl Command for MovePlayer {
    fn apply(self, world: &mut World) {
        let mut system_state = SystemState::<(
            Query<&mut Position, With<PlayerTag>>,
            Query<(&Position, &TileType), Without<PlayerTag>>,
        )>::from_world(world);

        let (mut q_player, q_tiles) = system_state.get_mut(world);

        for mut position in q_player.iter_mut() {
            let mut new_position = **position;
            new_position.x = (position.x + self.x).min(MAP_WIDTH - 1).max(0);
            new_position.y = (position.y + self.y).min(MAP_HEIGHT - 1).max(0);

            if let Some((_found_position, found_tile_type)) =
                q_tiles.iter().find(|(tile_pos, _tile_type)| ***tile_pos == new_position)
            {
                if *found_tile_type != TileType::Wall {
                    **position = new_position;
                }
            }
        }

        system_state.apply(world);
    }
}
