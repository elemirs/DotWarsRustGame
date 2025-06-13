use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        println!("Graphics Plugin loaded - 2D rendering systems initialized");
    }
}
