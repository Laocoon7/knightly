use assets::AssetsPlugin;
use bevy::{prelude::*, window::WindowResolution};
use components::ComponentsPlugin;
use states::StatesPlugin;
use systems::SystemsPlugin;

pub mod assets;

pub mod commands;
pub mod components;
pub mod constants;
pub mod states;
pub mod systems;
pub mod types;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    title: "Knightly".to_string(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
        AssetsPlugin,
        ComponentsPlugin,
        StatesPlugin,
        SystemsPlugin,
    ));

    app.run();
}
