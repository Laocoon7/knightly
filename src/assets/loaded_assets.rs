use bevy::{prelude::*, utils::HashMap};

use crate::assets::actors::ActorAsset;

#[derive(Resource, Default)]
pub struct LoadedAssets {
    pub actor_handles_loading: Vec<Handle<ActorAsset>>,
    pub actor_handles: HashMap<String, Handle<ActorAsset>>,
    pub actor_ids: HashMap<String, AssetId<ActorAsset>>,
}

impl LoadedAssets {
    pub fn add_actor(&mut self, handle: Handle<ActorAsset>) { self.actor_handles_loading.push(handle); }

    pub fn actor_loaded(&mut self, name: &str, id: &AssetId<ActorAsset>) {
        if let Some(index) = self.actor_handles_loading.iter().position(|x| x.id() == *id) {
            let handle = self.actor_handles_loading.remove(index);
            self.actor_handles.insert(name.to_string(), handle);
            self.actor_ids.insert(name.to_string(), id.clone());
        }
    }

    pub fn get_actor_id(&self, name: &str) -> Option<AssetId<ActorAsset>> {
        self.actor_ids.get(name).cloned()
    }

    pub fn get_actor_handle(&self, name: &str) -> Option<Handle<ActorAsset>> {
        self.actor_handles.get(name).cloned()
    }

    pub fn remove_actor_by_id(&mut self, id: &AssetId<ActorAsset>) {
        self.actor_ids.retain(|_name, current_id| *current_id != *id);
    }
}
