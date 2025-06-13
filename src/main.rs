use bevy::prelude::*;
use dot_wars_core::GameState;  // Core crate'den GameState'i import et

mod plugins;

use plugins::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Nokta Savaşları - Büyük Strateji Savaş Simülatörü".into(),
                resolution: (1400.0, 900.0).into(),
                ..default()
            }),
            ..default()
        }).set(AssetPlugin {
            file_path: "assets".to_string(),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(GamePlugins)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
