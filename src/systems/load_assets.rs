use bevy::prelude::*;

use crate::{assets::AssetContext, states::AppState};

pub fn load_assets(mut asset_context: AssetContext, mut app_state: ResMut<NextState<AppState>>) {
    asset_context.load_actor("data/actors/npc.actor");
    asset_context.load_actor("data/actors/player.actor");

    asset_context.load_terrain("data/terrain/floor.terrain");
    asset_context.load_terrain("data/terrain/wall.terrain");

    app_state.set(AppState::InGame);
}
