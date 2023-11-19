use bevy::prelude::*;

use crate::commands::MovePlayer;

pub fn player_input(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Numpad1) {
        commands.add(MovePlayer(IVec2::new(-1, -1)));
    } else if keyboard_input.just_pressed(KeyCode::Numpad2) {
        commands.add(MovePlayer(IVec2::new(0, -1)));
    } else if keyboard_input.just_pressed(KeyCode::Numpad3) {
        commands.add(MovePlayer(IVec2::new(1, -1)));
    } else if keyboard_input.just_pressed(KeyCode::Numpad4) {
        commands.add(MovePlayer(IVec2::new(-1, 0)));
    } else if keyboard_input.just_pressed(KeyCode::Numpad6) {
        commands.add(MovePlayer(IVec2::new(1, 0)));
    } else if keyboard_input.just_pressed(KeyCode::Numpad7) {
        commands.add(MovePlayer(IVec2::new(-1, 1)));
    } else if keyboard_input.just_pressed(KeyCode::Numpad8) {
        commands.add(MovePlayer(IVec2::new(0, 1)));
    } else if keyboard_input.just_pressed(KeyCode::Numpad9) {
        commands.add(MovePlayer(IVec2::new(1, 1)));
    }
}
