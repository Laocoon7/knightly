use bevy::{prelude::*, utils::HashMap};

use super::{actors::ActorAsset, terrain::TerrainAsset};

#[derive(Resource, Default)]
pub struct LoadedAssets {
    pub actor_handles_loading: Vec<Handle<ActorAsset>>,
    pub actor_handles: HashMap<String, Handle<ActorAsset>>,
    pub actor_ids: HashMap<String, AssetId<ActorAsset>>,

    pub terrain_handles_loading: Vec<Handle<TerrainAsset>>,
    pub terrain_handles: HashMap<String, Handle<TerrainAsset>>,
    pub terrain_ids: HashMap<String, AssetId<TerrainAsset>>,
}

impl LoadedAssets {
    pub fn add_actor(&mut self, handle: Handle<ActorAsset>) { self.actor_handles_loading.push(handle); }

    pub fn add_terrain(&mut self, handle: Handle<TerrainAsset>) { self.terrain_handles_loading.push(handle); }

    pub fn actor_loaded(&mut self, name: &str, id: &AssetId<ActorAsset>) {
        if let Some(index) = self.actor_handles_loading.iter().position(|x| x.id() == *id) {
            let handle = self.actor_handles_loading.remove(index);
            self.actor_handles.insert(name.to_string(), handle);
            self.actor_ids.insert(name.to_string(), id.clone());
        }
    }

    pub fn terrain_loaded(&mut self, name: &str, id: &AssetId<TerrainAsset>) {
        if let Some(index) = self.terrain_handles_loading.iter().position(|x| x.id() == *id) {
            let handle = self.terrain_handles_loading.remove(index);
            self.terrain_handles.insert(name.to_string(), handle);
            self.terrain_ids.insert(name.to_string(), id.clone());
        }
    }

    pub fn get_actor_id(&self, name: &str) -> Option<AssetId<ActorAsset>> {
        self.actor_ids.get(name).cloned()
    }

    pub fn get_terrain_id(&self, name: &str) -> Option<AssetId<TerrainAsset>> {
        self.terrain_ids.get(name).cloned()
    }

    pub fn get_actor_handle(&self, name: &str) -> Option<Handle<ActorAsset>> {
        self.actor_handles.get(name).cloned()
    }

    pub fn get_terrain_handle(&self, name: &str) -> Option<Handle<TerrainAsset>> {
        self.terrain_handles.get(name).cloned()
    }

    pub fn remove_actor_by_id(&mut self, id: &AssetId<ActorAsset>) {
        self.actor_ids.retain(|_name, current_id| *current_id != *id);
    }

    pub fn remove_terrain_by_id(&mut self, id: &AssetId<TerrainAsset>) {
        self.terrain_ids.retain(|_name, current_id| *current_id != *id);
    }
}
