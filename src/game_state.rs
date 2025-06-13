use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    WorldMap,
    Battle,
    Diplomacy,
    Technology,
    Settings,
    Loading,
}
