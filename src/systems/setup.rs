use bevy::prelude::*;

use crate::{assets::AssetContext, states::AppState};

pub fn setup(mut asset_context: AssetContext, mut app_state: ResMut<NextState<AppState>>) {
    asset_context.load_actor("data/actors/npc.actor");
    asset_context.load_actor("data/actors/player.actor");

    app_state.set(AppState::InGame);
}
