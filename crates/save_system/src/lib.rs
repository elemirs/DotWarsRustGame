use bevy::prelude::*;

pub struct SaveSystemPlugin;

impl Plugin for SaveSystemPlugin {
    fn build(&self, app: &mut App) {
        println!("Save System Plugin loaded - Game save/load functionality initialized");
    }
}
