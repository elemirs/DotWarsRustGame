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
