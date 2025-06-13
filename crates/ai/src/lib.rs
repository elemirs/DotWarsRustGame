use bevy::prelude::*;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        println!("AI Plugin loaded - Artificial Intelligence systems initialized");
    }
}
