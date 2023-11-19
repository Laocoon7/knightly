use bevy::prelude::*;

pub trait Spawn {
    fn spawn(&self, commands: &mut Commands) -> Entity {
        let entity = commands.spawn_empty().id();

        self.add_to_entity(commands, entity);

        entity
    }

    fn add_to_entity(&self, commands: &mut Commands, entity: Entity);
}
