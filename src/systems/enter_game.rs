use bevy::prelude::*;
use bevy_fortress::prelude::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::{
    assets::AssetContext,
    components::tags::PlayerTag,
    constants::{MAP_HEIGHT, MAP_WIDTH},
    states::GameState,
    types::{traits::SpawnAt, TileType},
};

const START: Coord = Coord { x: 40, y: 25 };

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

    let Some(floor_asset) = asset_context.get_terrain("floor") else {
        error!("Failed to find terrain: floor");
        return;
    };

    let Some(wall_asset) = asset_context.get_terrain("wall") else {
        error!("Failed to find terrain: wall");
        return;
    };

    let player_entity = player_asset.spawn_at(&mut commands, START);
    commands.entity(player_entity).insert(PlayerTag);

    let map = new_map();
    for coord in map.coord_iter() {
        match map.get_checked(coord) {
            TileType::Floor => {
                let terrain_entity = floor_asset.spawn_at(&mut commands, coord);
                commands.entity(terrain_entity).insert(TileType::Floor);
            },
            TileType::Wall => {
                let terrain_entity = wall_asset.spawn_at(&mut commands, coord);
                commands.entity(terrain_entity).insert(TileType::Wall);
            },
        };
    }

    game_state.set(GameState::Playing);
}

fn new_map() -> Grid<TileType> {
    let mut map = Grid::new_default(Size::new(MAP_WIDTH as u32, MAP_HEIGHT as u32));

    for coord in map.edge_coord_iter() {
        *map.get_checked_mut(coord) = TileType::Wall;
    }

    let mut rng = Pcg64::from_entropy();

    for _ in 0..400 {
        let x = rng.gen_range(1..(MAP_WIDTH - 2));
        let y = rng.gen_range(1..(MAP_HEIGHT - 2));

        *map.get_checked_mut(Coord::new(x, y)) = TileType::Wall;
    }

    *map.get_checked_mut(START) = TileType::Floor;

    map
}
