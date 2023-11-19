use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};

use crate::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

pub fn build_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(TILE_SIZE),
            ..Default::default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        transform: Transform::from_xyz(MAP_WIDTH as f32 / 2.0, MAP_HEIGHT as f32 / 2.0, 999.0),
        ..Default::default()
    },));
}
