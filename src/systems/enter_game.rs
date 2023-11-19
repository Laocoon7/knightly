use bevy::prelude::*;

use crate::{
    assets::AssetContext,
    components::{tags::PlayerTag, Map},
    states::GameState,
    types::traits::SpawnAt,
};

pub fn enter_game(
    mut commands: Commands,

    asset_context: AssetContext,

    mut game_state: ResMut<NextState<GameState>>,
) {
    let Some(player_asset) = asset_context.get_actor("player") else {
        error!("Failed to find actor: player");
        return;
    };

    let Some(_npc_asset) = asset_context.get_actor("npc") else {
        error!("Failed to find actor: npc");
        return;
    };

    if let Some(map) = Map::new(&mut commands, &asset_context) {
        let start_position = map.start_position;
        commands.spawn(map);

        let player_entity = player_asset.spawn_at(&mut commands, start_position);
        commands.entity(player_entity).insert(PlayerTag);

        game_state.set(GameState::Playing);
    }
}
