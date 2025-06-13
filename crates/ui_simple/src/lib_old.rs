use bevy::prelude::*;
use dot_wars_core::GameState;

// SimpleUIPlugin'i public olarak export et
#[derive(Default)]
pub struct SimpleUIPlugin;

// Font handles
#[derive(Resource)]
pub struct UiFonts {
    pub regular: Handle<Font>,
    pub bold: Handle<Font>,
    pub emoji: Handle<Font>,
    pub emoji_color: Handle<Font>,
}

// UI Animation Timer
#[derive(Resource)]
pub struct UiAnimationTimer {
    pub timer: Timer,
    pub pulse_timer: Timer,
}

impl Default for UiAnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.02, TimerMode::Repeating),
            pulse_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

// Modern Color Palette
pub struct ModernColors;

impl ModernColors {
    pub const DARK_BG: Color = Color::srgb(0.05, 0.05, 0.08);
    pub const CARD_BG: Color = Color::srgb(0.1, 0.12, 0.18);
    pub const ACCENT_BLUE: Color = Color::srgb(0.2, 0.6, 1.0);
    pub const ACCENT_PURPLE: Color = Color::srgb(0.6, 0.2, 1.0);
    pub const ACCENT_GOLD: Color = Color::srgb(1.0, 0.8, 0.2);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.97);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.75, 0.8);
    pub const BUTTON_HOVER: Color = Color::srgb(0.15, 0.18, 0.25);
    pub const BUTTON_PRESSED: Color = Color::srgb(0.08, 0.1, 0.15);
}

impl Plugin for SimpleUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiAnimationTimer>()
            .add_systems(Startup, load_fonts)
            .add_systems(Update, (
                check_fonts_and_setup_ui,
                animate_ui_elements,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
                handle_world_map_ui.run_if(in_state(GameState::WorldMap))
            ))
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(OnEnter(GameState::WorldMap), setup_world_map_ui)
            .add_systems(OnExit(GameState::WorldMap), cleanup_world_map_ui);
        
        println!("🎨 Modern UI Plugin loaded - Havalı animasyonlu arayüz sistemi yüklendi!");
    }
}

// Font yükleme sistemi
fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_fonts = UiFonts {
        regular: asset_server.load("fonts/NotoSans-Regular.ttf"),
        bold: asset_server.load("fonts/NotoSans-Bold.ttf"),
        emoji: asset_server.load("fonts/NotoEmoji-Regular.ttf"),
        emoji_color: asset_server.load("fonts/NotoColorEmoji.ttf"),
    };
    
    commands.insert_resource(ui_fonts);
    println!("🎨 Modern fontlar yükleniyor...");
}

// Fontların yüklenip yüklenmediğini kontrol et ve UI'ı kur
fn check_fonts_and_setup_ui(
    mut commands: Commands,
    ui_fonts: Option<Res<UiFonts>>,
    asset_server: Res<AssetServer>,
    current_state: Res<State<GameState>>,
    main_menu_query: Query<Entity, With<MainMenuUI>>,
) {
    if let Some(fonts) = ui_fonts {
        if *current_state.get() == GameState::MainMenu 
            && main_menu_query.is_empty() {
            
            let regular_loaded = asset_server.load_state(&fonts.regular) == bevy::asset::LoadState::Loaded;
            let bold_loaded = asset_server.load_state(&fonts.bold) == bevy::asset::LoadState::Loaded;
            let emoji_loaded = asset_server.load_state(&fonts.emoji) == bevy::asset::LoadState::Loaded;
            
            if regular_loaded && bold_loaded && emoji_loaded {
                println!("🚀 Modern arayüz oluşturuluyor...");
                setup_main_menu_internal(&mut commands, &fonts);
            }
        }
    }
}

// Ana menü komponenti
#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct OptionsButton;

#[derive(Component)]
pub struct ExitButton;

// Animation components
#[derive(Component)]
pub struct AnimatedElement {
    pub base_scale: f32,
    pub pulse_amplitude: f32,
    pub phase_offset: f32,
}

#[derive(Component)]
pub struct FloatingElement {
    pub base_y: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct GlowEffect {
    pub base_color: Color,
    pub glow_intensity: f32,
    pub pulse_speed: f32,
}

// Animation system
fn animate_ui_elements(
    time: Res<Time>,
    mut timer: ResMut<UiAnimationTimer>,
    mut animated_query: Query<(&mut Transform, &AnimatedElement)>,
    mut floating_query: Query<(&mut Style, &FloatingElement), Without<AnimatedElement>>,
    mut glow_query: Query<(&mut BackgroundColor, &GlowEffect)>,
) {
    timer.timer.tick(time.delta());
    timer.pulse_timer.tick(time.delta());
    
    let global_time = time.elapsed_seconds();
    
    // Pulse animations for buttons and elements
    for (mut transform, animated) in animated_query.iter_mut() {
        let pulse = (global_time * 2.0 + animated.phase_offset).sin() * animated.pulse_amplitude;
        let scale = animated.base_scale + pulse;
        transform.scale = Vec3::splat(scale);
    }
    
    // Floating animations
    for (mut style, floating) in floating_query.iter_mut() {
        let offset = (global_time * floating.frequency + floating.phase).sin() * floating.amplitude;
        if let Val::Px(base_y) = Val::Px(floating.base_y) {
            style.top = Val::Px(base_y + offset);
        }
    }
    
    // Glow effects
    for (mut bg_color, glow) in glow_query.iter_mut() {
        let pulse = (global_time * glow.pulse_speed).sin() * 0.5 + 0.5;
        let intensity = glow.glow_intensity * pulse;
        
        let base = glow.base_color;
        bg_color.0 = Color::srgb(
            (base.to_srgba().red + intensity).min(1.0),
            (base.to_srgba().green + intensity).min(1.0),
            (base.to_srgba().blue + intensity).min(1.0),
        );
    }
}

// Modern Ana menü kurulumu - Havalı tasarım
fn setup_main_menu_internal(commands: &mut Commands, ui_fonts: &UiFonts) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: ModernColors::DARK_BG.into(),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Hero title
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "⚔️ ",
                        TextStyle {
                            font: ui_fonts.emoji.clone(),
                            font_size: 80.0,
                            color: ModernColors::ACCENT_GOLD,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "NOKTA SAVAŞLARI",
                        TextStyle {
                            font: ui_fonts.bold.clone(),
                            font_size: 72.0,
                            color: ModernColors::TEXT_PRIMARY,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        " 🏆",
                        TextStyle {
                            font: ui_fonts.emoji.clone(),
                            font_size: 80.0,
                            color: ModernColors::ACCENT_GOLD,
                            ..default()
                        },
                    ),
                ]).with_style(Style {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                }),
                AnimatedElement {
                    base_scale: 1.0,
                    pulse_amplitude: 0.02,
                    phase_offset: 0.0,
                },
            ));

            // Subtitle
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "🌍 Büyük Strateji Savaş Simülatörü 🎯",
                    TextStyle {
                        font: ui_fonts.regular.clone(),
                        font_size: 26.0,
                        color: ModernColors::TEXT_SECONDARY,
                        ..default()
                    },
                ),
            ]).with_style(Style {
                margin: UiRect::bottom(Val::Px(60.0)),
                ..default()
            }));

            // Button panel
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    gap: Val::Px(20.0),
                    ..default()
                },
                ..default()
            }).with_children(|buttons| {
                // Start Game Button
                buttons.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(70.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(3.0)),
                            ..default()
                        },
                        background_color: ModernColors::CARD_BG.into(),
                        border_color: ModernColors::ACCENT_BLUE.into(),
                        ..default()
                    },
                    StartGameButton,
                    AnimatedElement {
                        base_scale: 1.0,
                        pulse_amplitude: 0.03,
                        phase_offset: 0.0,
                    },
                )).with_children(|button| {
                    button.spawn(TextBundle::from_sections([
                        TextSection::new(
                            "🚀 SAVAŞA BAŞLA 🚀",
                            TextStyle {
                                font: ui_fonts.bold.clone(),
                                font_size: 24.0,
                                color: ModernColors::TEXT_PRIMARY,
                                ..default()
                            },
                        ),
                    ]));
                });

                // Options Button
                buttons.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: ModernColors::CARD_BG.into(),
                        border_color: ModernColors::ACCENT_PURPLE.into(),
                        ..default()
                    },
                    OptionsButton,
                )).with_children(|button| {
                    button.spawn(TextBundle::from_sections([
                        TextSection::new(
                            "⚙️ Ayarlar",
                            TextStyle {
                                font: ui_fonts.regular.clone(),
                                font_size: 20.0,
                                color: ModernColors::TEXT_SECONDARY,
                                ..default()
                            },
                        ),
                    ]));
                });

                // Exit Button
                buttons.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: ModernColors::CARD_BG.into(),
                        border_color: Color::srgb(0.8, 0.3, 0.3).into(),
                        ..default()
                    },
                    ExitButton,
                )).with_children(|button| {
                    button.spawn(TextBundle::from_sections([
                        TextSection::new(
                            "🚪 Çıkış",
                            TextStyle {
                                font: ui_fonts.regular.clone(),
                                font_size: 20.0,
                                color: ModernColors::TEXT_SECONDARY,
                                ..default()
                            },
                        ),
                    ]));
                });
            });
        });
}

// Modern Ana menü buton işlemleri
fn handle_main_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&mut Transform>),
        (Changed<Interaction>, With<Button>),
    >,
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    exit_button_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    // Modern buton hover efektleri
    for (interaction, mut color, mut transform) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = ModernColors::BUTTON_PRESSED.into();
                if let Some(ref mut t) = transform {
                    t.scale = Vec3::splat(0.95);
                }
            }
            Interaction::Hovered => {
                *color = ModernColors::BUTTON_HOVER.into();
                if let Some(ref mut t) = transform {
                    t.scale = Vec3::splat(1.05);
                }
            }
            Interaction::None => {
                *color = ModernColors::CARD_BG.into();
                if let Some(ref mut t) = transform {
                    t.scale = Vec3::splat(1.0);
                }
            }
        }
    }

    // Oyunu başlat butonu
    for interaction in &start_button_query {
        if *interaction == Interaction::Pressed {
            println!("🚀 SAVAŞA HAZIRLANIN! Dünya haritası yükleniyor... 🗺️⚔️");
            next_state.set(GameState::WorldMap);
        }
    }

    // Çıkış butonu
    for interaction in &exit_button_query {
        if *interaction == Interaction::Pressed {
            println!("🚪 Savaşçı, başka bir gün tekrar bekleriz! Güle güle! 👋");
            exit.send(bevy::app::AppExit::Success);
        }
    }
}

// Ana menüyü temizle
fn cleanup_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuUI>>) {
    for entity in &menu_query {
        commands.entity(entity).despawn_recursive();
    }
}

// Dünya haritası UI komponenti
#[derive(Component)]
pub struct WorldMapUI;

#[derive(Component)]
pub struct BackToMenuButton;

// Dünya haritası UI kurulumu
fn setup_world_map_ui(mut commands: Commands, ui_fonts: Option<Res<UiFonts>>) {
    if let Some(fonts) = ui_fonts {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: ModernColors::DARK_BG.into(),
                    ..default()
                },
                WorldMapUI,
            ))
            .with_children(|parent| {
                // Header
                parent.spawn(TextBundle::from_sections([
                    TextSection::new(
                        "🗺️ DÜNYA HARİTASI 🌍",
                        TextStyle {
                            font: fonts.bold.clone(),
                            font_size: 32.0,
                            color: ModernColors::TEXT_PRIMARY,
                            ..default()
                        },
                    ),
                ]));

                // Back button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            margin: UiRect::top(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: ModernColors::CARD_BG.into(),
                        border_color: ModernColors::ACCENT_GOLD.into(),
                        ..default()
                    },
                    BackToMenuButton,
                )).with_children(|button| {
                    button.spawn(TextBundle::from_sections([
                        TextSection::new(
                            "🏠 Ana Menü",
                            TextStyle {
                                font: fonts.regular.clone(),
                                font_size: 18.0,
                                color: ModernColors::TEXT_PRIMARY,
                                ..default()
                            },
                        ),
                    ]));
                });
            });
    }
}

// Dünya haritası UI işlemleri
fn handle_world_map_ui(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    back_button_query: Query<&Interaction, (Changed<Interaction>, With<BackToMenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Buton hover efektleri
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = ModernColors::BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                *color = ModernColors::BUTTON_HOVER.into();
            }
            Interaction::None => {
                *color = ModernColors::CARD_BG.into();
            }
        }
    }

    // Ana menüye dön butonu
    for interaction in &back_button_query {
        if *interaction == Interaction::Pressed {
            println!("🏠 Ana menüye dönülüyor...");
            next_state.set(GameState::MainMenu);
        }
    }
}

// Dünya haritası UI'sini temizle
fn cleanup_world_map_ui(mut commands: Commands, ui_query: Query<Entity, With<WorldMapUI>>) {
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for SimpleUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiAnimationTimer>()
            .add_systems(Startup, load_fonts)
            .add_systems(Update, (
                check_fonts_and_setup_ui,
                animate_ui_elements,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
                handle_world_map_ui.run_if(in_state(GameState::WorldMap))
            ))
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(OnEnter(GameState::WorldMap), setup_world_map_ui)
            .add_systems(OnExit(GameState::WorldMap), cleanup_world_map_ui);
        
        println!("🎨 Modern UI Plugin loaded - Havalı animasyonlu arayüz sistemi yüklendi!");
    }
}

// Font yükleme sistemi
fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_fonts = UiFonts {
        regular: asset_server.load("fonts/NotoSans-Regular.ttf"),
        bold: asset_server.load("fonts/NotoSans-Bold.ttf"),
        emoji: asset_server.load("fonts/NotoEmoji-Regular.ttf"),
        emoji_color: asset_server.load("fonts/NotoColorEmoji.ttf"),
    };
    
    commands.insert_resource(ui_fonts);
    println!("Türkçe karakter ve emoji destekli fontlar yükleniyor...");
}

// Fontların yüklenip yüklenmediğini kontrol et ve UI'ı kur
fn check_fonts_and_setup_ui(
    mut commands: Commands,
    ui_fonts: Option<Res<UiFonts>>,
    asset_server: Res<AssetServer>,
    current_state: Res<State<GameState>>,
    main_menu_query: Query<Entity, With<MainMenuUI>>,
) {
    if let Some(fonts) = ui_fonts {
        // Ana menü state'indeyiz ve ana menü UI'si yoksa ve fontlar yüklendiyse
        if *current_state.get() == GameState::MainMenu 
            && main_menu_query.is_empty() {
            
            let regular_loaded = asset_server.load_state(&fonts.regular) == bevy::asset::LoadState::Loaded;
            let bold_loaded = asset_server.load_state(&fonts.bold) == bevy::asset::LoadState::Loaded;
            let emoji_loaded = asset_server.load_state(&fonts.emoji) == bevy::asset::LoadState::Loaded;
            let _emoji_color_loaded = asset_server.load_state(&fonts.emoji_color) == bevy::asset::LoadState::Loaded;
            
            println!("Font durumları - Regular: {:?}, Bold: {:?}, Emoji: {:?}, EmojiColor: {:?}", 
                asset_server.load_state(&fonts.regular),
                asset_server.load_state(&fonts.bold),
                asset_server.load_state(&fonts.emoji),
                asset_server.load_state(&fonts.emoji_color)
            );
            
            if regular_loaded && bold_loaded {
                println!("Türkçe fontlar başarıyla yüklendi! Ana menü oluşturuluyor...");
                if emoji_loaded {
                    println!("🎉 Emoji fontları da yüklendi! Desteklenen kategoriler:");
                    println!("🎮 Oyun: 🔥💥⭐🎯🚀🏆👑💎⚡🌟");
                    println!("⚔️ Savaş: 🗡️🛡️🏰🏹🔫💣🎖️");
                    println!("🌍 Dünya: 🌎🌏🗺️🏔️🏞️🌊🏙️");
                    println!("😊 Duygular: 😎😍🤔😮😡🥳🎉");
                }
                setup_main_menu_internal(&mut commands, &fonts);
            }
        }
    }
}

// Ana menü komponenti
#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct OptionsButton;

#[derive(Component)]
pub struct ExitButton;

// Animation components
#[derive(Component)]
pub struct AnimatedElement {
    pub base_scale: f32,
    pub pulse_amplitude: f32,
    pub phase_offset: f32,
}

#[derive(Component)]
pub struct FloatingElement {
    pub base_y: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct GlowEffect {
    pub base_color: Color,
    pub glow_intensity: f32,
    pub pulse_speed: f32,
}

// Animation system
fn animate_ui_elements(
    time: Res<Time>,
    mut timer: ResMut<UiAnimationTimer>,
    mut animated_query: Query<(&mut Transform, &AnimatedElement)>,
    mut floating_query: Query<(&mut Style, &FloatingElement), Without<AnimatedElement>>,
    mut glow_query: Query<(&mut BackgroundColor, &GlowEffect)>,
) {
    timer.timer.tick(time.delta());
    timer.pulse_timer.tick(time.delta());
    
    let global_time = time.elapsed_seconds();
    
    // Pulse animations for buttons and elements
    for (mut transform, animated) in animated_query.iter_mut() {
        let pulse = (global_time * 2.0 + animated.phase_offset).sin() * animated.pulse_amplitude;
        let scale = animated.base_scale + pulse;
        transform.scale = Vec3::splat(scale);
    }
    
    // Floating animations
    for (mut style, floating) in floating_query.iter_mut() {
        let offset = (global_time * floating.frequency + floating.phase).sin() * floating.amplitude;
        if let Val::Px(base_y) = Val::Px(floating.base_y) {
            style.top = Val::Px(base_y + offset);
        }
    }
    
    // Glow effects
    for (mut bg_color, glow) in glow_query.iter_mut() {
        let pulse = (global_time * glow.pulse_speed).sin() * 0.5 + 0.5;
        let intensity = glow.glow_intensity * pulse;
        
        let base = glow.base_color;
        bg_color.0 = Color::srgb(
            (base.to_srgba().red + intensity).min(1.0),
            (base.to_srgba().green + intensity).min(1.0),
            (base.to_srgba().blue + intensity).min(1.0),
        );
    }
}

// Ana menü kurulumu - internal - Emoji fontları ile
// Modern Ana menü kurulumu - Havalı tasarım
fn setup_main_menu_internal(commands: &mut Commands, ui_fonts: &UiFonts) {
    // Ana container - Dark gradient background
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: ModernColors::DARK_BG.into(),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Floating particles background effect
            create_particle_background(parent, ui_fonts);
            
            // Main content container
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                ..default()
            }).with_children(|content| {
                // Hero title with glow effect
                create_hero_title(content, ui_fonts);
                
                // Subtitle with floating animation
                create_subtitle(content, ui_fonts);
                
                // Feature showcase
                create_feature_showcase(content, ui_fonts);
                
                // Modern button panel
                create_button_panel(content, ui_fonts);
            });
        });
}

// Create floating particles in background
fn create_particle_background(parent: &mut ChildBuilder, ui_fonts: &UiFonts) {
    let particles = ["⭐", "💫", "✨", "🌟", "💎", "🔹", "🔸", "⚡"];
    
    for (i, particle) in particles.iter().enumerate() {
        parent.spawn((
            TextBundle::from_section(
                *particle,
                TextStyle {
                    font: ui_fonts.emoji.clone(),
                    font_size: 20.0 + (i as f32 * 5.0),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.1 + (i as f32 * 0.05)),
                    ..default()
                }
            ).with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(10.0 + (i as f32 * 10.0)),
                top: Val::Percent(15.0 + (i as f32 * 8.0)),
                ..default()
            }),
            FloatingElement {
                base_y: 100.0 + (i as f32 * 50.0),
                amplitude: 10.0 + (i as f32 * 2.0),
                frequency: 0.5 + (i as f32 * 0.1),
                phase: i as f32 * 0.5,
            },
        ));
    }
}

// Create hero title with glow
fn create_hero_title(parent: &mut ChildBuilder, ui_fonts: &UiFonts) {
    parent.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "⚔️ ",
                TextStyle {
                    font: ui_fonts.emoji.clone(),
                    font_size: 80.0,
                    color: ModernColors::ACCENT_GOLD,
                    ..default()
                },
            ),
            TextSection::new(
                "NOKTA",
                TextStyle {
                    font: ui_fonts.bold.clone(),
                    font_size: 72.0,
                    color: ModernColors::ACCENT_BLUE,
                    ..default()
                },
            ),
            TextSection::new(
                " SAVAŞLARI ",
                TextStyle {
                    font: ui_fonts.bold.clone(),
                    font_size: 72.0,
                    color: ModernColors::TEXT_PRIMARY,
                    ..default()
                },
            ),
            TextSection::new(
                "🏆",
                TextStyle {
                    font: ui_fonts.emoji.clone(),
                    font_size: 80.0,
                    color: ModernColors::ACCENT_GOLD,
                    ..default()
                },
            ),
        ]).with_style(Style {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        }),
        AnimatedElement {
            base_scale: 1.0,
            pulse_amplitude: 0.02,
            phase_offset: 0.0,
        },
    ));
}

// Create animated subtitle
fn create_subtitle(parent: &mut ChildBuilder, ui_fonts: &UiFonts) {
    parent.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "� ",
                TextStyle {
                    font: ui_fonts.emoji.clone(),
                    font_size: 28.0,
                    color: ModernColors::ACCENT_PURPLE,
                    ..default()
                },
            ),
            TextSection::new(
                "Büyük Strateji Savaş Simülatörü",
                TextStyle {
                    font: ui_fonts.regular.clone(),
                    font_size: 26.0,
                    color: ModernColors::TEXT_SECONDARY,
                    ..default()
                },
            ),
            TextSection::new(
                " 🎯",
                TextStyle {
                    font: ui_fonts.emoji.clone(),
                    font_size: 28.0,
                    color: ModernColors::ACCENT_PURPLE,
                    ..default()
                },
            ),
        ]).with_style(Style {
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        }),
        FloatingElement {
            base_y: 0.0,
            amplitude: 3.0,
            frequency: 1.0,
            phase: 1.57, // π/2 offset
        },
    ));
}

// Create feature showcase
fn create_feature_showcase(parent: &mut ChildBuilder, ui_fonts: &UiFonts) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            width: Val::Px(600.0),
            margin: UiRect::bottom(Val::Px(50.0)),
            ..default()
        },
        ..default()
    }).with_children(|features| {
        // Feature cards
        let feature_data = [
            ("🗺️", "Stratejik\nHarita", ModernColors::ACCENT_BLUE),
            ("⚔️", "Epik\nSavaşlar", ModernColors::ACCENT_GOLD),
            ("🤖", "Akıllı\nAI", ModernColors::ACCENT_PURPLE),
        ];
        
        for (i, (emoji, text, color)) in feature_data.iter().enumerate() {
            features.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(120.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: ModernColors::CARD_BG.into(),
                    border_color: color.into(),
                    ..default()
                },
                AnimatedElement {
                    base_scale: 1.0,
                    pulse_amplitude: 0.05,
                    phase_offset: i as f32 * 2.0,
                },
            )).with_children(|card| {
                // Emoji
                card.spawn(TextBundle::from_section(
                    *emoji,
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 32.0,
                        color: *color,
                        ..default()
                    }
                ));
                
                // Text
                card.spawn(TextBundle::from_section(
                    *text,
                    TextStyle {
                        font: ui_fonts.regular.clone(),
                        font_size: 12.0,
                        color: ModernColors::TEXT_SECONDARY,
                        ..default()
                    }
                ).with_text_justify(JustifyText::Center));
            });
        }
    });
}

// Create modern button panel
fn create_button_panel(parent: &mut ChildBuilder, ui_fonts: &UiFonts) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            gap: Val::Px(15.0),
            ..default()
        },
        ..default()
    }).with_children(|buttons| {
        // Start Game Button - Premium style
        buttons.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(320.0),
                    height: Val::Px(70.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: ModernColors::CARD_BG.into(),
                border_color: ModernColors::ACCENT_BLUE.into(),
                ..default()
            },
            StartGameButton,
            GlowEffect {
                base_color: ModernColors::CARD_BG,
                glow_intensity: 0.1,
                pulse_speed: 2.0,
            },
            AnimatedElement {
                base_scale: 1.0,
                pulse_amplitude: 0.03,
                phase_offset: 0.0,
            },
        )).with_children(|button| {
            button.spawn(TextBundle::from_sections([
                TextSection::new(
                    "🚀 ",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 24.0,
                        color: ModernColors::ACCENT_BLUE,
                        ..default()
                    },
                ),
                TextSection::new(
                    "SAVAŞA BAŞLA",
                    TextStyle {
                        font: ui_fonts.bold.clone(),
                        font_size: 22.0,
                        color: ModernColors::TEXT_PRIMARY,
                        ..default()
                    },
                ),
                TextSection::new(
                    " 🚀",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 24.0,
                        color: ModernColors::ACCENT_BLUE,
                        ..default()
                    },
                ),
            ]));
        });

        // Options Button
        buttons.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(320.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: ModernColors::CARD_BG.into(),
                border_color: ModernColors::ACCENT_PURPLE.into(),
                ..default()
            },
            OptionsButton,
            AnimatedElement {
                base_scale: 1.0,
                pulse_amplitude: 0.02,
                phase_offset: 1.0,
            },
        )).with_children(|button| {
            button.spawn(TextBundle::from_sections([
                TextSection::new(
                    "⚙️ ",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 20.0,
                        color: ModernColors::ACCENT_PURPLE,
                        ..default()
                    },
                ),
                TextSection::new(
                    "Ayarlar",
                    TextStyle {
                        font: ui_fonts.regular.clone(),
                        font_size: 18.0,
                        color: ModernColors::TEXT_SECONDARY,
                        ..default()
                    },
                ),
            ]));
        });

        // Exit Button
        buttons.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(320.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: ModernColors::CARD_BG.into(),
                border_color: Color::srgb(0.8, 0.3, 0.3).into(),
                ..default()
            },
            ExitButton,
            AnimatedElement {
                base_scale: 1.0,
                pulse_amplitude: 0.02,
                phase_offset: 2.0,
            },
        )).with_children(|button| {
            button.spawn(TextBundle::from_sections([
                TextSection::new(
                    "🚪 ",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 20.0,
                        color: Color::srgb(0.8, 0.3, 0.3),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Çıkış",
                    TextStyle {
                        font: ui_fonts.regular.clone(),
                        font_size: 18.0,
                        color: ModernColors::TEXT_SECONDARY,
                        ..default()
                    },
                ),
            ]));
        });
    });
}
                            TextStyle {
                                font: ui_fonts.emoji.clone(),
                                font_size: 20.0,
                                color: Color::srgb(0.8, 0.8, 0.8),
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "Seçenekler",
                            TextStyle {
                                font: ui_fonts.regular.clone(),
                                font_size: 20.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                    ]));
                });

            // Çıkış butonu - Emoji ile
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(280.0),
                            height: Val::Px(60.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.5, 0.2, 0.2).into(),
                        ..default()
                    },
                    ExitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_sections([
                        TextSection::new(
                            "🚪 ",
                            TextStyle {
                                font: ui_fonts.emoji.clone(),
                                font_size: 20.0,
                                color: Color::srgb(1.0, 0.6, 0.6),
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "Çıkış",
                            TextStyle {
                                font: ui_fonts.regular.clone(),
                                font_size: 20.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                    ]));
                });
        });
}

// Modern Ana menü buton işlemleri
fn handle_main_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&mut Transform>),
        (Changed<Interaction>, With<Button>),
    >,
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    exit_button_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    // Modern buton hover efektleri
    for (interaction, mut color, mut transform) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = ModernColors::BUTTON_PRESSED.into();
                if let Some(ref mut t) = transform {
                    t.scale = Vec3::splat(0.95);
                }
            }
            Interaction::Hovered => {
                *color = ModernColors::BUTTON_HOVER.into();
                if let Some(ref mut t) = transform {
                    t.scale = Vec3::splat(1.05);
                }
            }
            Interaction::None => {
                *color = ModernColors::CARD_BG.into();
                if let Some(ref mut t) = transform {
                    t.scale = Vec3::splat(1.0);
                }
            }
        }
    }

    // Oyunu başlat butonu - Cool effect
    for interaction in &start_button_query {
        if *interaction == Interaction::Pressed {
            println!("🚀 SAVAŞA HAZIRLANIN! Dünya haritası yükleniyor... 🗺️⚔️");
            next_state.set(GameState::WorldMap);
        }
    }

    // Çıkış butonu
    for interaction in &exit_button_query {
        if *interaction == Interaction::Pressed {
            println!("� Savaşçı, başka bir gün tekrar bekleriz! Güle güle! 👋");
            exit.send(bevy::app::AppExit::Success);
        }
    }
}

// Ana menüyü temizle
fn cleanup_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuUI>>) {
    for entity in &menu_query {
        commands.entity(entity).despawn_recursive();
    }
}

// Dünya haritası UI komponenti
#[derive(Component)]
pub struct WorldMapUI;

#[derive(Component)]
pub struct BackToMenuButton;

// Dünya haritası UI kurulumu - Emoji destekli
fn setup_world_map_ui(mut commands: Commands, ui_fonts: Option<Res<UiFonts>>) {
    // Font fallback
    let (regular_font, bold_font, emoji_font) = if let Some(fonts) = ui_fonts {
        (fonts.regular.clone(), fonts.bold.clone(), fonts.emoji.clone())
    } else {
        (Handle::default(), Handle::default(), Handle::default())
    };

    // Üst panel
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
                ..default()
            },
            WorldMapUI,
        ))
        .with_children(|parent| {
            // Oyun başlığı - Emoji ile
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "🗺️ ",
                    TextStyle {
                        font: emoji_font.clone(),
                        font_size: 28.0,
                        color: Color::srgb(0.3, 0.8, 0.3),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Dünya Haritası",
                    TextStyle {
                        font: bold_font.clone(),
                        font_size: 28.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            ]));

            // Ana menüye dön butonu - Emoji ile
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    },
                    BackToMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_sections([
                        TextSection::new(
                            "🏠 ",
                            TextStyle {
                                font: emoji_font.clone(),
                                font_size: 16.0,
                                color: Color::srgb(0.8, 0.6, 0.3),
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "Ana Menü",
                            TextStyle {
                                font: regular_font.clone(),
                                font_size: 16.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                    ]));
                });
        });

    // Merkezi bilgi paneli - Emoji destekli
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgb(0.05, 0.1, 0.05).into(),
                ..default()
            },
            WorldMapUI,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_sections([
                    TextSection::new(
                        "🗺️ ",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 24.0,
                            color: Color::srgb(0.3, 0.8, 0.3),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "Dünya Haritası Yükleniyor... ",
                        TextStyle {
                            font: regular_font.clone(),
                            font_size: 24.0,
                            color: Color::srgb(0.7, 0.8, 0.7),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "🌍\n\n",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 24.0,
                            color: Color::srgb(0.3, 0.8, 0.3),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "✅ BAŞARILI! Emojiler artık görünüyor! ✅\n\n",
                        TextStyle {
                            font: regular_font.clone(),
                            font_size: 22.0,
                            color: Color::srgb(0.3, 1.0, 0.3),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "🎮 ",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.3, 0.8, 1.0),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "Oyun simgeleri: ",
                        TextStyle {
                            font: regular_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.7, 0.8, 0.7),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "🔥💥⭐🎯🚀🏆👑💎⚡🌟\n",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(1.0, 0.8, 0.0),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "⚔️ ",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.9, 0.3, 0.3),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "Savaş simgeleri: ",
                        TextStyle {
                            font: regular_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.7, 0.8, 0.7),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "🗡️🛡️🏰🏹💣🎖️\n",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.9, 0.3, 0.3),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "🌍 ",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.3, 0.8, 0.3),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "Dünya emojileri: ",
                        TextStyle {
                            font: regular_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.7, 0.8, 0.7),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "🌎🗺️🏔️🌊🏙️\n",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.3, 0.8, 0.3),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "😊 ",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(1.0, 0.8, 0.0),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "Duygusal ifadeler: ",
                        TextStyle {
                            font: regular_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(0.7, 0.8, 0.7),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "😎😍🤔😮🥳🎉\n\n",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 20.0,
                            color: Color::srgb(1.0, 0.8, 0.0),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "🇹🇷 ",
                        TextStyle {
                            font: emoji_font.clone(),
                            font_size: 18.0,
                            color: Color::srgb(0.9, 0.2, 0.2),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "Türkçe karakter desteği: ÇĞIİÖŞÜ çğıöşü",
                        TextStyle {
                            font: regular_font.clone(),
                            font_size: 18.0,
                            color: Color::srgb(0.7, 0.8, 0.7),
                            ..default()
                        },
                    ),
                ]).with_text_justify(JustifyText::Center)
            );
        });
}

// Dünya haritası UI işlemleri
fn handle_world_map_ui(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackToMenuButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.2, 0.2, 0.2).into();
                println!("🏠 Ana menüye dönülüyor...");
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.4, 0.4, 0.4).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
        }
    }
}

// Dünya haritası UI'yi temizle
fn cleanup_world_map_ui(mut commands: Commands, ui_query: Query<Entity, With<WorldMapUI>>) {
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
}
