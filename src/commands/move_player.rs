use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
};
use bevy_fortress::prelude::*;

use crate::{
    components::{tags::PlayerTag, Map, Position},
    constants::{MAP_HEIGHT, MAP_WIDTH},
    types::TileType,
};

#[derive(Clone, Copy, Deref, DerefMut)]
pub struct MovePlayer(pub Coord);

impl Command for MovePlayer {
    fn apply(self, world: &mut World) {
        let mut system_state = SystemState::<(
            Query<&mut Position, With<PlayerTag>>,
            Query<&Map>,
            Query<&TileType>,
        )>::from_world(world);

        let (mut q_player, q_map, q_terrain) = system_state.get_mut(world);

        for mut position in q_player.iter_mut() {
            let mut new_position = **position;
            new_position.x = (position.x + self.x).min(MAP_WIDTH - 1).max(0);
            new_position.y = (position.y + self.y).min(MAP_HEIGHT - 1).max(0);

            for map in q_map.iter() {
                if let Some(entity) = map.get_entity_at(new_position) {
                    if let Ok(tile_type) = q_terrain.get(entity) {
                        if *tile_type == TileType::Wall {
                            continue;
                        } else {
                            **position = new_position;
                        }
                    }
                }
            }
        }

        system_state.apply(world);
    }
}
