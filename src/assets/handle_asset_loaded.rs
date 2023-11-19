use bevy::prelude::*;

use super::{actors::ActorAsset, terrain::TerrainAsset, LoadedAssets};

pub fn handle_asset_loaded(
    mut loaded_assets: ResMut<LoadedAssets>,

    mut actor_events: EventReader<AssetEvent<ActorAsset>>,
    actor_assets: Res<Assets<ActorAsset>>,

    mut terrain_events: EventReader<AssetEvent<TerrainAsset>>,
    terrain_assets: Res<Assets<TerrainAsset>>,
) {
    for event in actor_events.read() {
        match event {
            AssetEvent::Removed { id } => loaded_assets.remove_actor_by_id(id),
            AssetEvent::LoadedWithDependencies { id } => {
                if let Some(asset) = actor_assets.get(*id) {
                    loaded_assets.actor_loaded(&asset.name, id);
                }
            },
            _ => (),
        }
    }

    for event in terrain_events.read() {
        match event {
            AssetEvent::Removed { id } => loaded_assets.remove_terrain_by_id(id),
            AssetEvent::LoadedWithDependencies { id } => {
                if let Some(asset) = terrain_assets.get(*id) {
                    loaded_assets.terrain_loaded(&asset.name, id);
                }
            },
            _ => (),
        }
    }
}
