use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use dot_wars_core::*;
use dot_wars_world::*;

// GameState enum
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

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, (
                main_menu_ui.run_if(in_state(GameState::MainMenu)),
                world_map_ui.run_if(in_state(GameState::WorldMap)),
                battle_ui.run_if(in_state(GameState::Battle)),
            ));
    }
}

fn main_menu_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            
            // Ana başlık - Türkçe karakterler destekleniyor
            ui.heading("🏛️ Nokta Savaşları");
            ui.label("Büyük Strateji ve Savaş Simülatörü");
            
            ui.add_space(50.0);
            
            // Güzel butonlar
            if ui.add_sized([200.0, 40.0], egui::Button::new("🗺️ Yeni Oyun")).clicked() {
                next_state.set(GameState::WorldMap);
            }
            
            ui.add_space(10.0);
            
            if ui.add_sized([200.0, 40.0], egui::Button::new("📂 Oyun Yükle")).clicked() {
                // TODO: Load game
            }
            
            ui.add_space(10.0);
            
            if ui.add_sized([200.0, 40.0], egui::Button::new("⚙️ Ayarlar")).clicked() {
                next_state.set(GameState::Settings);
            }
            
            ui.add_space(10.0);
            
            if ui.add_sized([200.0, 40.0], egui::Button::new("🚪 Çıkış")).clicked() {
                std::process::exit(0);
            }
        });
    });
}

fn world_map_ui(
    mut contexts: EguiContexts,
    _world_map: Option<Res<WorldMap>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Sol panel - Faction bilgileri
    egui::SidePanel::left("faction_panel")
        .resizable(true)
        .default_width(300.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("🏴 Faction Bilgileri");
            
            ui.separator();
            
            // Resource display
            ui.label("💰 Kaynaklar:");
            ui.horizontal(|ui| {
                ui.label("🪙 Altın: 1,250");
                ui.label("🌾 Yiyecek: 800");
            });
            ui.horizontal(|ui| {
                ui.label("🔨 Malzeme: 450");
                ui.label("👥 İnsan Gücü: 320");
            });
            
            ui.add_space(20.0);
            
            // Quick actions
            ui.label("⚡ Hızlı İşlemler:");
            if ui.button("🏗️ Yapı İnşa Et").clicked() {
                // TODO: Open building menu
            }
            if ui.button("🎖️ Ordu Oluştur").clicked() {
                // TODO: Open army creation
            }
            if ui.button("🤝 Diplomasi").clicked() {
                next_state.set(GameState::Diplomacy);
            }
            if ui.button("🔬 Teknoloji").clicked() {
                next_state.set(GameState::Technology);
            }
        });
    
    // Üst panel - Ana menü butonu
    egui::TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("🏠 Ana Menü").clicked() {
                next_state.set(GameState::MainMenu);
            }
            
            ui.separator();
            
            ui.label("📅 Tur: 1");
            ui.label("📍 Seçili Bölge: Yok");
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⏭️ Sonraki Tur").clicked() {
                    // TODO: Process turn
                }
            });
        });
    });
    
    // Ana harita alanı
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading("🗺️ Dünya Haritası");
        ui.label("Burada harita görünecek...");
        
        // Geçici test butonu
        if ui.button("⚔️ Test Savaşı").clicked() {
            next_state.set(GameState::Battle);
        }
    });
}

fn battle_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Üst panel - Savaş kontrolü
    egui::TopBottomPanel::top("battle_top").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("🏠 Dünya Haritası").clicked() {
                next_state.set(GameState::WorldMap);
            }
            
            ui.separator();
            
            ui.label("⚔️ Savaş: Test Muharebesi");
            ui.label("📊 Aşama: Dağıtım");
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⏸️ Duraklat").clicked() {
                    // TODO: Pause battle
                }
                if ui.button("▶️ Başlat").clicked() {
                    // TODO: Start battle
                }
            });
        });
    });
    
    // Sol panel - Birim kontrolü
    egui::SidePanel::left("unit_panel")
        .resizable(true)
        .default_width(280.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("🎖️ Birimler");
            
            ui.separator();
            
            // Unit list
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 1..=5 {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            match i {
                                1 => { ui.label("🛡️ Piyade Alayı"); },
                                2 => { ui.label("🐎 Süvari Birliği"); },
                                3 => { ui.label("🏹 Okçu Takımı"); },
                                4 => { ui.label("💣 Top Birliği"); },
                                _ => { ui.label("⭐ Elit Birlik"); },
                            }
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("{}/100", 85 + i * 3));
                            });
                        });
                        
                        // Health bar
                        let health = (85 + i * 3) as f32 / 100.0;
                        ui.add(egui::ProgressBar::new(health).text("Sağlık"));
                        
                        // Morale bar
                        let morale = (70 + i * 5) as f32 / 100.0;
                        ui.add(egui::ProgressBar::new(morale).text("Moral"));
                    });
                    ui.add_space(5.0);
                }
            });
        });
    
    // Ana savaş alanı
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading("⚔️ Savaş Alanı");
        ui.label("Burada 2D savaş sahası görünecek...");
        
        ui.add_space(20.0);
        
        // Formation controls
        ui.horizontal(|ui| {
            ui.label("📐 Formasyon:");
            if ui.selectable_label(true, "➖ Dizi").clicked() {
                // TODO: Set line formation
            }
            if ui.selectable_label(false, "🔲 Kolon").clicked() {
                // TODO: Set column formation
            }
            if ui.selectable_label(false, "⬛ Kare").clicked() {
                // TODO: Set square formation
            }
        });
    });
}
