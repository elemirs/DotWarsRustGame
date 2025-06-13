use bevy::prelude::*;
use dot_wars_ui_simple;

// Import plugin modules - UI geçici olarak devre dışı
// pub use dot_wars_ui::UIPlugin;
pub use dot_wars_graphics::GraphicsPlugin;
pub use dot_wars_ai::AIPlugin;
pub use dot_wars_save_system::SaveSystemPlugin;

pub struct CorePlugin;
pub struct WorldPlugin;
pub struct BattlePlugin;
pub struct StrategyPlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Core systems will be implemented here
        println!("Core Plugin loaded");
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // World simulation systems
        println!("World Plugin loaded");
    }
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        // Battle simulation systems
        println!("Battle Plugin loaded");
    }
}

impl Plugin for StrategyPlugin {
    fn build(&self, app: &mut App) {
        // Grand strategy systems
        println!("Strategy Plugin loaded - Büyük strateji sistemleri yüklendi");
    }
}

pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CorePlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(BattlePlugin)
            .add_plugins(dot_wars_ui_simple::SimpleUIPlugin::default())  // Basit UI sistemi aktif
            .add_plugins(GraphicsPlugin)
            .add_plugins(AIPlugin)
            .add_plugins(StrategyPlugin)
            .add_plugins(SaveSystemPlugin);
    }
}
