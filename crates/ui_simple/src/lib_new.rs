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

impl Plugin for SimpleUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_fonts)
            .add_systems(Update, (
                check_fonts_and_setup_ui,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
                handle_world_map_ui.run_if(in_state(GameState::WorldMap))
            ))
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(OnEnter(GameState::WorldMap), setup_world_map_ui)
            .add_systems(OnExit(GameState::WorldMap), cleanup_world_map_ui);
        
        println!("Simple UI Plugin loaded - Türkçe karakter ve emoji destekli UI sistemi yüklendi");
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

// Ana menü kurulumu - internal - Emoji fontları ile
fn setup_main_menu_internal(commands: &mut Commands, ui_fonts: &UiFonts) {
    // Ana menü container
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
                background_color: Color::srgb(0.1, 0.1, 0.2).into(),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Oyun başlığı - Karma fontlar ile emoji
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "🎯 ",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 60.0,
                        color: Color::srgb(1.0, 0.8, 0.0),
                        ..default()
                    },
                ),
                TextSection::new(
                    "NOKTA SAVAŞLARI",
                    TextStyle {
                        font: ui_fonts.bold.clone(),
                        font_size: 60.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                TextSection::new(
                    " 🎯",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 60.0,
                        color: Color::srgb(1.0, 0.8, 0.0),
                        ..default()
                    },
                ),
            ]).with_style(Style {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }));

            // Alt başlık - Karma fontlar ile emoji
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "⚔️ ",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 24.0,
                        color: Color::srgb(0.9, 0.3, 0.3),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Büyük Strateji Savaş Simülatörü",
                    TextStyle {
                        font: ui_fonts.regular.clone(),
                        font_size: 24.0,
                        color: Color::srgb(0.7, 0.7, 0.7),
                        ..default()
                    },
                ),
                TextSection::new(
                    " ⚔️",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 24.0,
                        color: Color::srgb(0.9, 0.3, 0.3),
                        ..default()
                    },
                ),
            ]).with_style(Style {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            }));

            // Emoji test bölümü - Karma fontlar
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "🇹🇷 ",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 16.0,
                        color: Color::srgb(0.9, 0.2, 0.2),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Türkçe: ÇĞIİÖŞÜ çğıöşü  ",
                    TextStyle {
                        font: ui_fonts.regular.clone(),
                        font_size: 16.0,
                        color: Color::srgb(0.6, 0.8, 0.6),
                        ..default()
                    },
                ),
                TextSection::new(
                    "🎮 ",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 16.0,
                        color: Color::srgb(0.3, 0.8, 1.0),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Oyun: ",
                    TextStyle {
                        font: ui_fonts.regular.clone(),
                        font_size: 16.0,
                        color: Color::srgb(0.6, 0.8, 0.6),
                        ..default()
                    },
                ),
                TextSection::new(
                    "🔥💥⭐🎯🚀🏆👑💎⚡🌟",
                    TextStyle {
                        font: ui_fonts.emoji.clone(),
                        font_size: 16.0,
                        color: Color::srgb(1.0, 0.8, 0.0),
                        ..default()
                    },
                ),
            ]).with_style(Style {
                margin: UiRect::bottom(Val::Px(60.0)),
                ..default()
            }).with_text_justify(JustifyText::Center));

            // Oyunu Başlat butonu - Emoji ile
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
                        background_color: Color::srgb(0.2, 0.3, 0.5).into(),
                        ..default()
                    },
                    StartGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_sections([
                        TextSection::new(
                            "🚀 ",
                            TextStyle {
                                font: ui_fonts.emoji.clone(),
                                font_size: 20.0,
                                color: Color::srgb(0.3, 0.8, 1.0),
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "Oyunu Başlat",
                            TextStyle {
                                font: ui_fonts.regular.clone(),
                                font_size: 20.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                    ]));
                });

            // Seçenekler butonu - Emoji ile
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
                        background_color: Color::srgb(0.2, 0.3, 0.5).into(),
                        ..default()
                    },
                    OptionsButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_sections([
                        TextSection::new(
                            "⚙️ ",
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

// Ana menü buton işlemleri
fn handle_main_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    exit_button_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    // Buton hover efektleri
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.3, 0.4, 0.6).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.3, 0.5).into();
            }
        }
    }

    // Oyunu başlat butonu
    for interaction in &start_button_query {
        if *interaction == Interaction::Pressed {
            println!("🎮 Oyun başlatılıyor... Dünya haritasına geçiliyor 🗺️");
            next_state.set(GameState::WorldMap);
        }
    }

    // Çıkış butonu
    for interaction in &exit_button_query {
        if *interaction == Interaction::Pressed {
            println!("👋 Oyundan çıkılıyor... Güle güle!");
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
