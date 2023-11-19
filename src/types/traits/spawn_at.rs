use bevy::prelude::*;
use bevy_fortress::prelude::*;

pub trait SpawnAt {
    fn spawn_at(&self, commands: &mut Commands, position: Coord) -> Entity {
        let entity = commands.spawn_empty().id();

        self.add_to_entity(commands, entity, position);

        entity
    }

    fn add_to_entity(&self, commands: &mut Commands, entity: Entity, position: Coord);
}
