use bevy::prelude::*;
use bevy_fortress::prelude::*;

use crate::{components::Position, constants::LAYER_TERRAIN, types::traits::SpawnAt};

#[derive(Asset, TypePath, Clone)]
pub struct TerrainAsset {
    pub name: String,
    pub texture_atlas: Handle<TextureAtlas>,
    pub index: usize,
    pub color: Color,
}

impl SpawnAt for TerrainAsset {
    fn add_to_entity(&self, commands: &mut Commands, entity: Entity, position: Coord) {
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
                transform: Transform::from_xyz(position.x as f32, position.y as f32, LAYER_TERRAIN),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            Position(position),
        ));
    }
}
