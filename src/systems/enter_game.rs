use bevy::prelude::*;

use crate::{
    assets::AssetContext,
    components::tags::{LeftMoverTag, PlayerTag},
    states::GameState,
    traits::SpawnAt,
};

const START: IVec2 = IVec2 { x: 40, y: 25 };

pub fn enter_game(
    mut commands: Commands,

    asset_context: AssetContext,

    mut game_state: ResMut<NextState<GameState>>,
) {
    let Some(player_asset) = asset_context.get_actor("player") else {
        error!("Failed to find actor: player");
        return;
    };

    let Some(npc_asset) = asset_context.get_actor("npc") else {
        error!("Failed to find actor: npc");
        return;
    };

    let player_entity = player_asset.spawn_at(&mut commands, START);
    commands.entity(player_entity).insert(PlayerTag);

    for i in 0..10 {
        let npc_entity = npc_asset.spawn_at(&mut commands, IVec2::new(i * 7, 20));
        commands.entity(npc_entity).insert(LeftMoverTag);
    }

    game_state.set(GameState::Playing);
}
