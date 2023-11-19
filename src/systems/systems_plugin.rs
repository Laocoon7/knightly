use bevy::prelude::*;

use super::{
    build_camera::build_camera, enter_game::enter_game, left_walker::left_walker, load_assets::load_assets,
    player_input::player_input, set_window_icon::set_window_icon, update_positions::update_positions,
};
use crate::states::{AppState, GameState};

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Change the window icon and start the camera
        app.add_systems(Startup, (set_window_icon, build_camera, load_assets));

        app.add_systems(
            Update,
            enter_game.run_if(in_state(AppState::InGame)).run_if(in_state(GameState::Setup)),
        );

        app.add_systems(
            PreUpdate,
            (player_input, update_positions)
                .chain()
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Playing)),
        );

        app.add_systems(
            PostUpdate,
            left_walker.run_if(in_state(AppState::InGame)).run_if(in_state(GameState::Playing)),
        );
    }
}
