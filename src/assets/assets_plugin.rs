use bevy::prelude::*;

use super::{
    actors::{ActorAsset, ActorLoader},
    atlases::AtlasLoader,
    handle_asset_loaded::handle_asset_loaded,
    terrain::{TerrainAsset, TerrainLoader},
    LoadedAssets,
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(AtlasLoader);
        app.register_asset_loader(ActorLoader).init_asset::<ActorAsset>();
        app.register_asset_loader(TerrainLoader).init_asset::<TerrainAsset>();

        app.init_resource::<LoadedAssets>();
        app.add_systems(First, handle_asset_loaded);
    }
}
