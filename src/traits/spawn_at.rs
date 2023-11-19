use bevy::prelude::*;

pub trait SpawnAt {
    fn spawn_at(&self, commands: &mut Commands, position: IVec2) -> Entity {
        let entity = commands.spawn_empty().id();

        self.add_to_entity(commands, entity, position);

        entity
    }

    fn add_to_entity(&self, commands: &mut Commands, entity: Entity, position: IVec2);
}
