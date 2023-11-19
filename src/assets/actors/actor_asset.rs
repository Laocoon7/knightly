use bevy::prelude::*;

use crate::{components::Position, constants::LAYER_ACTOR, traits::SpawnAt};

#[derive(Asset, TypePath, Clone)]
pub struct ActorAsset {
    pub name: String,
    pub texture_atlas: Handle<TextureAtlas>,
    pub index: usize,
    pub color: Color,
}

impl SpawnAt for ActorAsset {
    fn add_to_entity(&self, commands: &mut Commands, entity: Entity, position: IVec2) {
        commands.entity(entity).insert((
            Name::new(self.name.clone()),
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: self.color,
                    index: self.index,
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: self.texture_atlas.clone(),
                transform: Transform::from_xyz(position.x as f32, position.y as f32, LAYER_ACTOR),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            Position(position),
        ));
    }
}
