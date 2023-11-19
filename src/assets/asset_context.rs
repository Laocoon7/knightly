use bevy::{ecs::system::SystemParam, prelude::*};

use super::{actors::ActorAsset, LoadedAssets};

#[derive(SystemParam)]
pub struct AssetContext<'w, 's> {
    _commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,

    loaded_assets: ResMut<'w, LoadedAssets>,

    actor_assets: Res<'w, Assets<ActorAsset>>,
}

impl<'w, 's> AssetContext<'w, 's> {
    pub fn load_actor(&mut self, path: &str) {
        let handle = self.asset_server.load(path.to_string());
        self.loaded_assets.add_actor(handle);
    }

    pub fn get_actor(&self, name: &str) -> Option<&ActorAsset> {
        if let Some(id) = self.loaded_assets.get_actor_id(name) {
            self.actor_assets.get(id)
        } else {
            None
        }
    }
}
