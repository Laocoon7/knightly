use bevy::prelude::*;

use super::{AppState, GameState};

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
        app.add_state::<GameState>();
    }
}
