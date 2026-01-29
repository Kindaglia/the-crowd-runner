mod levels;

use bevy::asset::AssetPlugin;
use bevy::audio::AudioPlugin;
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::prelude::*;

const TRACK_WIDTH: f32 = 8.0;
const TRACK_LENGTH: f32 = 90.0;
const PLAYER_SPEED: f32 = 7.5;
const LATERAL_SPEED: f32 = 8.0;
const PLAYER_MARGIN: f32 = 0.8;
const COLLISION_RANGE: f32 = 1.1;
const DISPLAY_HEIGHT: f32 = 2.2;
const DIGIT_WIDTH: f32 = 0.7;
const DIGIT_HEIGHT: f32 = 1.1;
const DIGIT_DEPTH: f32 = 0.12;
const DIGIT_GAP: f32 = 0.15;
const SYMBOL_GAP: f32 = 0.25;
const LEVEL_COUNT: usize = 3;
const LEVEL_NAMES: [&str; LEVEL_COUNT] = ["Level 1", "Level 2", "Level 3"];

#[derive(Resource, Clone)]
struct MeshAssets {
    unit_cube: Handle<Mesh>,
    stickman: Handle<Mesh>,
}

#[derive(Resource, Clone)]
struct MaterialAssets {
    ground: Handle<StandardMaterial>,
    rail: Handle<StandardMaterial>,
    player: Handle<StandardMaterial>,
    enemy: Handle<StandardMaterial>,
    gate_add: Handle<StandardMaterial>,
    gate_mul: Handle<StandardMaterial>,
    obstacle: Handle<StandardMaterial>,
    display_player: Handle<StandardMaterial>,
    display_enemy: Handle<StandardMaterial>,
    display_gate: Handle<StandardMaterial>,
    control_hint: Handle<StandardMaterial>,
    finish_light: Handle<StandardMaterial>,
    finish_dark: Handle<StandardMaterial>,
}

#[derive(Resource, Clone)]
struct UiAssets {
    font: Handle<Font>,
}

#[derive(Resource, Default)]
struct SwipeState {
    dragging: bool,
    last_position: Vec2,
    axis: f32,
}

#[derive(Resource, Default)]
struct InputAxis {
    value: f32,
}

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppScreen {
    #[default]
    MainMenu,
    LevelSelect,
    Settings,
    Playing,
}

#[derive(Resource, Clone)]
struct LevelProgress {
    completed: [bool; LEVEL_COUNT],
}

impl Default for LevelProgress {
    fn default() -> Self {
        Self {
            completed: [false; LEVEL_COUNT],
        }
    }
}

#[derive(Resource, Clone, Copy)]
struct CurrentLevel {
    index: usize,
}

#[derive(Resource)]
struct GameState {
    running: bool,
    outcome: Option<GameOutcome>,
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
struct LevelSelectRoot;

#[derive(Component)]
struct SettingsRoot;

#[derive(Component)]
struct GameplayUiRoot;

#[derive(Component)]
struct MenuPlayButton;

#[derive(Component)]
struct MenuSettingsButton;

#[derive(Component)]
struct MenuQuitButton;

#[derive(Component)]
struct BackToMenuButton;

#[derive(Component)]
struct LevelButton {
    index: usize,
}

#[derive(Component)]
struct LevelLabel;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct CrowdCount {
    current: i32,
}

#[derive(Component)]
struct CrowdFormation;

#[derive(Component)]
struct EnemyCount {
    current: i32,
}

#[derive(Component)]
struct EnemyGroup {
    width: f32,
}

#[derive(Component)]
struct Gate {
    kind: GateKind,
    width: f32,
}

#[derive(Component)]
struct Obstacle {
    damage: i32,
    width: f32,
}

#[derive(Component)]
struct NumberDisplay {
    value: i32,
    prefix: LabelPrefix,
    material: Handle<StandardMaterial>,
    kind: DisplayKind,
}

#[derive(Component)]
struct FollowCamera;

#[derive(Component)]
struct LevelEntity;

#[derive(Component)]
struct Boss {
    required: i32,
}

#[derive(Component)]
struct FinishLine;

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct EndScreenNextButton;

#[derive(Component)]
struct EndScreenMenuButton;

#[derive(Component)]
struct EndScreenRoot;

#[derive(Resource)]
struct EndScreenUi {
    root: Entity,
    label: Entity,
    restart_button: Entity,
    next_button: Option<Entity>,
    menu_button: Entity,
}

#[derive(Clone, Copy)]
enum GateKind {
    Add(i32),
    Multiply(i32),
}

#[derive(Clone, Copy, PartialEq)]
enum LabelPrefix {
    None,
    Plus,
    Minus,
    Times,
}

#[derive(Clone, Copy, PartialEq)]
enum DisplayKind {
    Player,
    Enemy,
}

#[derive(Clone, Copy)]
enum GameOutcome {
    Win,
    Lose,
}

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.08, 0.09, 0.12)))
        .insert_resource(AmbientLight {
            color: Color::rgb(0.9, 0.9, 1.0),
            brightness: 600.0,
        })
        .init_resource::<SwipeState>()
        .init_resource::<InputAxis>()
        .init_state::<AppScreen>()
        .insert_resource(GameState {
            running: false,
            outcome: None,
        })
        .insert_resource(LevelProgress::default())
        .insert_resource(CurrentLevel { index: 0 })
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<AudioPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Crowd Runner".into(),
                        resolution: (390.0, 780.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "/usr/share/fonts".into(),
                    ..default()
                }),
        )
        .add_systems(Startup, (setup_assets, setup_ui_camera).chain())
        .add_systems(OnEnter(AppScreen::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppScreen::MainMenu), cleanup_main_menu)
        .add_systems(OnEnter(AppScreen::LevelSelect), setup_level_select)
        .add_systems(OnExit(AppScreen::LevelSelect), cleanup_level_select)
        .add_systems(OnEnter(AppScreen::Settings), setup_settings)
        .add_systems(OnExit(AppScreen::Settings), cleanup_settings)
        .add_systems(OnEnter(AppScreen::Playing), setup_playing)
        .add_systems(OnExit(AppScreen::Playing), cleanup_playing)
        .add_systems(
            Update,
            handle_main_menu_buttons.run_if(in_state(AppScreen::MainMenu)),
        )
        .add_systems(
            Update,
            handle_level_select_buttons.run_if(in_state(AppScreen::LevelSelect)),
        )
        .add_systems(
            Update,
            handle_back_to_menu.run_if(in_state(AppScreen::LevelSelect)),
        )
        .add_systems(
            Update,
            handle_back_to_menu.run_if(in_state(AppScreen::Settings)),
        )
        .add_systems(
            Update,
            (
                update_swipe_axis,
                update_input_axis,
                move_player,
                handle_collisions,
                update_end_screen_ui,
                handle_ui_restart,
                handle_ui_next_level,
                handle_ui_back_to_menu,
                update_level_label,
                sync_displays,
                refresh_player_formation,
                refresh_enemy_formation,
                follow_camera,
                handle_restart,
                handle_keyboard_quit,
            )
                .chain()
                .run_if(in_state(AppScreen::Playing)),
        )
        .run();
}

fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let unit_cube = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let stickman = meshes.add(Mesh::from(Cuboid::new(0.25, 0.6, 0.25)));

    let mesh_assets = MeshAssets {
        unit_cube,
        stickman,
    };

    let material_assets = MaterialAssets {
        ground: materials.add(Color::rgb(0.16, 0.18, 0.22)),
        rail: materials.add(Color::rgb(0.22, 0.23, 0.26)),
        player: materials.add(Color::rgb(0.2, 0.78, 0.9)),
        enemy: materials.add(Color::rgb(0.85, 0.2, 0.2)),
        gate_add: materials.add(Color::rgb(0.25, 0.75, 0.38)),
        gate_mul: materials.add(Color::rgb(0.28, 0.45, 0.85)),
        obstacle: materials.add(Color::rgb(0.9, 0.6, 0.2)),
        display_player: materials.add(Color::rgb(0.92, 0.96, 1.0)),
        display_enemy: materials.add(Color::rgb(1.0, 0.72, 0.72)),
        display_gate: materials.add(Color::rgb(1.0, 0.95, 0.8)),
        control_hint: materials.add(Color::rgb(0.5, 0.52, 0.58)),
        finish_light: materials.add(Color::rgb(0.92, 0.92, 0.92)),
        finish_dark: materials.add(Color::rgb(0.12, 0.12, 0.12)),
    };

    commands.insert_resource(mesh_assets);
    commands.insert_resource(material_assets);
    commands.insert_resource(UiAssets {
        font: asset_server.load("Adwaita/AdwaitaSans-Regular.ttf"),
    });
}

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });
}

fn setup_main_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(18.0),
                    ..default()
                },
                background_color: Color::rgb(0.06, 0.07, 0.1).into(),
                ..default()
            },
            MainMenuRoot,
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "The Crowd Runner",
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 44.0,
                color: Color::rgb(0.95, 0.96, 1.0),
            },
        ));
        parent.spawn(TextBundle::from_section(
            "Main Menu",
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 20.0,
                color: Color::rgb(0.7, 0.72, 0.78),
            },
        ));

        let (normal, _, _) = menu_button_colors();
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(220.0),
                        height: Val::Px(56.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: normal.into(),
                    ..default()
                },
                MenuPlayButton,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 28.0,
                        color: Color::rgb(0.95, 0.95, 0.95),
                    },
                ));
            });
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(220.0),
                        height: Val::Px(56.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: normal.into(),
                    ..default()
                },
                MenuSettingsButton,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 28.0,
                        color: Color::rgb(0.95, 0.95, 0.95),
                    },
                ));
            });
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(220.0),
                        height: Val::Px(56.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.75, 0.2, 0.2).into(),
                    ..default()
                },
                MenuQuitButton,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Quit",
                    TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 28.0,
                        color: Color::rgb(0.95, 0.95, 0.95),
                    },
                ));
            });
    });
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    cleanup_screen(&mut commands, &query);
}

fn setup_level_select(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    progress: Res<LevelProgress>,
) {
    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::rgb(0.05, 0.06, 0.09).into(),
                ..default()
            },
            LevelSelectRoot,
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Select Level",
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 36.0,
                color: Color::rgb(0.95, 0.96, 1.0),
            },
        ));

        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    width: Val::Px(328.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
                    row_gap: Val::Px(12.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|row| {
                for index in 0..LEVEL_COUNT {
                    let completed = progress.completed[index];
                    let (normal, _, _) = level_button_colors(completed);
                    row.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(56.0),
                                height: Val::Px(56.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: normal.into(),
                            ..default()
                        },
                        LevelButton { index },
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            format!("{}", index + 1),
                            TextStyle {
                                font: ui_assets.font.clone(),
                                font_size: 22.0,
                                color: Color::rgb(0.97, 0.98, 1.0),
                            },
                        ));
                    });
                }
            });

        let (normal, _, _) = menu_button_colors();
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: normal.into(),
                    ..default()
                },
                BackToMenuButton,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Back",
                    TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 22.0,
                        color: Color::rgb(0.95, 0.95, 0.95),
                    },
                ));
            });
    });
}

fn cleanup_level_select(mut commands: Commands, query: Query<Entity, With<LevelSelectRoot>>) {
    cleanup_screen(&mut commands, &query);
}

fn setup_settings(mut commands: Commands, ui_assets: Res<UiAssets>) {
    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    ..default()
                },
                background_color: Color::rgb(0.05, 0.06, 0.09).into(),
                ..default()
            },
            SettingsRoot,
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Settings",
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 36.0,
                color: Color::rgb(0.95, 0.96, 1.0),
            },
        ));
        parent.spawn(TextBundle::from_section(
            "Coming soon",
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 20.0,
                color: Color::rgb(0.7, 0.72, 0.78),
            },
        ));

        let (normal, _, _) = menu_button_colors();
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: normal.into(),
                    ..default()
                },
                BackToMenuButton,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Back",
                    TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 22.0,
                        color: Color::rgb(0.95, 0.95, 0.95),
                    },
                ));
            });
    });
}

fn cleanup_settings(mut commands: Commands, query: Query<Entity, With<SettingsRoot>>) {
    cleanup_screen(&mut commands, &query);
}

fn setup_playing(
    mut commands: Commands,
    meshes: Res<MeshAssets>,
    materials: Res<MaterialAssets>,
    ui_assets: Res<UiAssets>,
    current_level: Res<CurrentLevel>,
    mut game_state: ResMut<GameState>,
    mut axis: ResMut<InputAxis>,
    mut swipe: ResMut<SwipeState>,
) {
    levels::spawn_level(current_level.index, &mut commands, &meshes, &materials);
    spawn_play_ui(&mut commands, &ui_assets, current_level.index);
    game_state.running = true;
    game_state.outcome = None;
    axis.value = 0.0;
    *swipe = SwipeState::default();
}

fn cleanup_playing(
    mut commands: Commands,
    level_query: Query<Entity, With<LevelEntity>>,
    ui_query: Query<Entity, With<GameplayUiRoot>>,
    mut game_state: ResMut<GameState>,
) {
    cleanup_screen(&mut commands, &level_query);
    cleanup_screen(&mut commands, &ui_query);
    commands.remove_resource::<EndScreenUi>();
    game_state.running = false;
    game_state.outcome = None;
}

fn spawn_play_ui(commands: &mut Commands, ui_assets: &UiAssets, level_index: usize) {
    commands.spawn((
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(24.0),
                left: Val::Px(24.0),
                ..default()
            },
            text: Text::from_section(
                LEVEL_NAMES[level_index],
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 24.0,
                    color: Color::rgb(0.92, 0.94, 1.0),
                },
            ),
            ..default()
        },
        GameplayUiRoot,
        LevelLabel,
    ));

    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(18.0),
                    display: Display::None,
                    ..default()
                },
                ..default()
            },
            EndScreenRoot,
            GameplayUiRoot,
        ))
        .id();

    let label = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "WIN",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 72.0,
                    color: Color::rgb(0.95, 0.96, 1.0),
                },
            ),
            ..default()
        })
        .id();

    let button_color = Color::rgb(0.18, 0.2, 0.22);
    let restart_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(56.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: button_color.into(),
                ..default()
            },
            RestartButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 28.0,
                    color: Color::rgb(0.95, 0.95, 0.95),
                },
            ));
        })
        .id();

    let has_next = level_index + 1 < LEVEL_COUNT;
    let next_button = if has_next {
        Some(
            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.2, 0.7, 0.38).into(),
                        ..default()
                    },
                    EndScreenNextButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Next Level",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 26.0,
                            color: Color::rgb(0.95, 0.98, 0.95),
                        },
                    ));
                })
                .id(),
        )
    } else {
        None
    };

    let menu_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(56.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: menu_button_colors().0.into(),
                ..default()
            },
            EndScreenMenuButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Back to Menu",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 24.0,
                    color: Color::rgb(0.95, 0.95, 0.95),
                },
            ));
        })
        .id();

    commands.entity(root).add_child(label);
    commands.entity(root).add_child(restart_button);
    if let Some(next_button) = next_button {
        commands.entity(root).add_child(next_button);
    }
    commands.entity(root).add_child(menu_button);
    commands.insert_resource(EndScreenUi {
        root,
        label,
        restart_button,
        next_button,
        menu_button,
    });
}

fn cleanup_screen<T: Component>(commands: &mut Commands, query: &Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn menu_button_colors() -> (Color, Color, Color) {
    (
        Color::rgb(0.18, 0.2, 0.22),
        Color::rgb(0.28, 0.3, 0.34),
        Color::rgb(0.4, 0.42, 0.46),
    )
}

fn level_button_colors(completed: bool) -> (Color, Color, Color) {
    if completed {
        (
            Color::rgb(0.2, 0.7, 0.38),
            Color::rgb(0.26, 0.78, 0.44),
            Color::rgb(0.16, 0.6, 0.33),
        )
    } else {
        (
            Color::rgb(0.2, 0.22, 0.26),
            Color::rgb(0.3, 0.32, 0.36),
            Color::rgb(0.16, 0.18, 0.22),
        )
    }
}

fn next_button_colors() -> (Color, Color, Color) {
    (
        Color::rgb(0.2, 0.7, 0.38),
        Color::rgb(0.26, 0.78, 0.44),
        Color::rgb(0.16, 0.6, 0.33),
    )
}

fn handle_main_menu_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&MenuPlayButton>,
            Option<&MenuSettingsButton>,
            Option<&MenuQuitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    let (normal, hovered, pressed) = menu_button_colors();
    let quit_colors = (
        Color::rgb(0.75, 0.2, 0.2),
        Color::rgb(0.86, 0.3, 0.3),
        Color::rgb(0.95, 0.4, 0.4),
    );

    for (interaction, mut color, play, settings, quit) in interaction_query.iter_mut() {
        let (base, highlight, down) = if quit.is_some() {
            quit_colors
        } else {
            (normal, hovered, pressed)
        };
        match *interaction {
            Interaction::Pressed => {
                if play.is_some() {
                    next_state.set(AppScreen::LevelSelect);
                }
                if settings.is_some() {
                    next_state.set(AppScreen::Settings);
                }
                if quit.is_some() {
                    std::process::exit(0);
                }
                *color = down.into();
            }
            Interaction::Hovered => {
                *color = highlight.into();
            }
            Interaction::None => {
                *color = base.into();
            }
        }
    }
}

fn handle_level_select_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &LevelButton),
        (Changed<Interaction>, With<Button>),
    >,
    progress: Res<LevelProgress>,
    mut current_level: ResMut<CurrentLevel>,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        let completed = progress.completed[button.index];
        let (normal, hovered, pressed) = level_button_colors(completed);
        match *interaction {
            Interaction::Pressed => {
                current_level.index = button.index;
                next_state.set(AppScreen::Playing);
                *color = pressed.into();
            }
            Interaction::Hovered => {
                *color = hovered.into();
            }
            Interaction::None => {
                *color = normal.into();
            }
        }
    }
}

fn handle_back_to_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackToMenuButton>),
    >,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    let (normal, hovered, pressed) = menu_button_colors();
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppScreen::MainMenu);
                *color = pressed.into();
            }
            Interaction::Hovered => {
                *color = hovered.into();
            }
            Interaction::None => {
                *color = normal.into();
            }
        }
    }
}

pub(super) fn spawn_ground(
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
) {
    let mesh = meshes.unit_cube.clone();

    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: materials.ground.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -0.05, TRACK_LENGTH * 0.5))
                .with_scale(Vec3::new(TRACK_WIDTH, 0.1, TRACK_LENGTH)),
            ..default()
        })
        .insert(LevelEntity);

    let rail_height = 0.6;
    let rail_length = TRACK_LENGTH;
    let rail_thickness = 0.2;
    for side in [-1.0, 1.0] {
        commands
            .spawn(PbrBundle {
                mesh: mesh.clone(),
                material: materials.rail.clone(),
                transform: Transform::from_translation(Vec3::new(
                    side * (TRACK_WIDTH * 0.5 + rail_thickness * 0.5),
                    rail_height * 0.5,
                    TRACK_LENGTH * 0.5,
                ))
                .with_scale(Vec3::new(rail_thickness, rail_height, rail_length)),
                ..default()
            })
            .insert(LevelEntity);
    }

    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(12.0, 14.0, -10.0)
                .looking_at(Vec3::new(0.0, 0.0, 40.0), Vec3::Y),
            ..default()
        })
        .insert(LevelEntity);
}

pub(super) fn spawn_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 18.0, -20.0)
                    .looking_at(Vec3::new(0.0, 0.0, 15.0), Vec3::Y),
                ..default()
            },
            FollowCamera,
            LevelEntity,
        ))
        .id()
}

pub(super) fn spawn_player(
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
) {
    let mesh = meshes.stickman.clone();

    let player = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 4.0)),
            Player,
            CrowdCount { current: 12 },
            LevelEntity,
        ))
        .id();

    let formation = commands
        .spawn((SpatialBundle::default(), CrowdFormation))
        .id();
    commands.entity(player).add_child(formation);

    let display = NumberDisplay {
        value: 12,
        prefix: LabelPrefix::None,
        material: materials.display_player.clone(),
        kind: DisplayKind::Player,
    };
    let display_entity = commands
        .spawn((
            SpatialBundle::from_transform(
                Transform::from_xyz(0.0, DISPLAY_HEIGHT, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ),
            NumberDisplay {
                value: 12,
                prefix: LabelPrefix::None,
                material: materials.display_player.clone(),
                kind: DisplayKind::Player,
            },
        ))
        .id();
    commands.entity(player).add_child(display_entity);
    rebuild_display(commands, display_entity, &display, meshes.unit_cube.clone());

    spawn_formation(commands, formation, 12, mesh, materials.player.clone());
}

pub(super) fn spawn_gate(
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
    position: Vec3,
    kind: GateKind,
) {
    let mesh = meshes.unit_cube.clone();

    let width = 3.4;
    let height = 2.2;
    let thickness = 0.16;
    let material = match kind {
        GateKind::Add(_) => materials.gate_add.clone(),
        GateKind::Multiply(_) => materials.gate_mul.clone(),
    };

    let gate = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position)),
            Gate { kind, width },
            LevelEntity,
        ))
        .id();

    commands.entity(gate).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-width * 0.5, height * 0.5, 0.0)
                .with_scale(Vec3::new(thickness, height, thickness)),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(width * 0.5, height * 0.5, 0.0)
                .with_scale(Vec3::new(thickness, height, thickness)),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, height + 0.1, 0.0).with_scale(Vec3::new(
                width + thickness,
                thickness,
                thickness,
            )),
            ..default()
        });
    });

    let (prefix, value) = match kind {
        GateKind::Add(amount) => {
            let prefix = if amount < 0 {
                LabelPrefix::Minus
            } else {
                LabelPrefix::Plus
            };
            (prefix, amount.abs())
        }
        GateKind::Multiply(multiplier) => (LabelPrefix::Times, multiplier),
    };

    spawn_static_label(
        commands,
        meshes,
        gate,
        prefix,
        value,
        materials.display_gate.clone(),
        Vec3::new(0.0, height + 0.6, 0.0),
    );
}

pub(super) fn spawn_obstacle(
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
    position: Vec3,
    damage: i32,
) {
    let mesh = meshes.unit_cube.clone();
    let width = 2.0;
    let height = 0.6;
    let depth = 0.6;

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: materials.obstacle.clone(),
            transform: Transform::from_translation(position + Vec3::new(0.0, height * 0.5, 0.0))
                .with_scale(Vec3::new(width, height, depth)),
            ..default()
        },
        Obstacle { damage, width },
        LevelEntity,
    ));
}

pub(super) fn spawn_enemy_group(
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
    position: Vec3,
    count: i32,
) {
    let mesh = meshes.stickman.clone();

    let enemy = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position)),
            EnemyGroup { width: 4.0 },
            EnemyCount { current: count },
            LevelEntity,
        ))
        .id();

    let formation = commands
        .spawn((SpatialBundle::default(), CrowdFormation))
        .id();
    commands.entity(enemy).add_child(formation);

    let display = NumberDisplay {
        value: count,
        prefix: LabelPrefix::None,
        material: materials.display_enemy.clone(),
        kind: DisplayKind::Enemy,
    };
    let display_entity = commands
        .spawn((
            SpatialBundle::from_transform(
                Transform::from_xyz(0.0, DISPLAY_HEIGHT, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ),
            NumberDisplay {
                value: count,
                prefix: LabelPrefix::None,
                material: materials.display_enemy.clone(),
                kind: DisplayKind::Enemy,
            },
        ))
        .id();
    commands.entity(enemy).add_child(display_entity);
    rebuild_display(commands, display_entity, &display, meshes.unit_cube.clone());

    spawn_formation(commands, formation, count, mesh, materials.enemy.clone());
}

pub(super) fn spawn_finish_line(
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
    position: Vec3,
) {
    let mesh = meshes.unit_cube.clone();
    let tile_size = 0.5;
    let rows = 2;
    let columns = (TRACK_WIDTH / tile_size).ceil() as i32;
    let height = 0.04;

    let finish = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position)),
            FinishLine,
            LevelEntity,
        ))
        .id();

    commands.entity(finish).with_children(|parent| {
        for row in 0..rows {
            for col in 0..columns {
                let is_light = (row + col) % 2 == 0;
                let material = if is_light {
                    materials.finish_light.clone()
                } else {
                    materials.finish_dark.clone()
                };
                let offset_x = (col as f32 + 0.5) * tile_size - TRACK_WIDTH * 0.5;
                let offset_z = (row as f32 + 0.5) * tile_size - (rows as f32 * tile_size * 0.5);
                parent.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material,
                    transform: Transform::from_translation(Vec3::new(
                        offset_x,
                        height * 0.5,
                        offset_z,
                    ))
                    .with_scale(Vec3::new(tile_size, height, tile_size)),
                    ..default()
                });
            }
        }
    });
}

pub(super) fn spawn_boss(
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
    position: Vec3,
    count: i32,
) {
    let mesh = meshes.stickman.clone();
    let required = count;

    let boss = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position)),
            EnemyGroup { width: 4.0 },
            EnemyCount { current: count },
            Boss { required },
            LevelEntity,
        ))
        .id();

    let formation = commands
        .spawn((SpatialBundle::default(), CrowdFormation))
        .id();
    commands.entity(boss).add_child(formation);

    let display = NumberDisplay {
        value: count,
        prefix: LabelPrefix::None,
        material: materials.display_enemy.clone(),
        kind: DisplayKind::Enemy,
    };
    let display_entity = commands
        .spawn((
            SpatialBundle::from_transform(
                Transform::from_xyz(0.0, DISPLAY_HEIGHT, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ),
            NumberDisplay {
                value: count,
                prefix: LabelPrefix::None,
                material: materials.display_enemy.clone(),
                kind: DisplayKind::Enemy,
            },
        ))
        .id();
    commands.entity(boss).add_child(display_entity);
    rebuild_display(commands, display_entity, &display, meshes.unit_cube.clone());

    spawn_formation(commands, formation, count, mesh, materials.enemy.clone());
}

pub(super) fn spawn_control_hint(
    commands: &mut Commands,
    camera: Entity,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
) {
    let mesh = meshes.unit_cube.clone();

    commands.entity(camera).with_children(|parent| {
        parent
            .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                0.0, -3.5, -6.0,
            )))
            .with_children(|hint| {
                hint.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.control_hint.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                        .with_scale(Vec3::new(4.0, 0.12, 0.4)),
                    ..default()
                });

                spawn_arrow(
                    hint,
                    mesh.clone(),
                    materials.control_hint.clone(),
                    -1.2,
                    0.4,
                    true,
                );
                spawn_arrow(
                    hint,
                    mesh.clone(),
                    materials.control_hint.clone(),
                    1.2,
                    0.4,
                    false,
                );
            });
    });
}

fn spawn_arrow(
    parent: &mut ChildBuilder,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    x: f32,
    y: f32,
    left: bool,
) {
    let angle: f32 = if left { 45.0 } else { -45.0 };
    let direction = if left { -1.0 } else { 1.0 };
    parent.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_translation(Vec3::new(x, y, 0.0))
            .with_rotation(Quat::from_rotation_z(angle.to_radians()))
            .with_scale(Vec3::new(0.6, 0.1, 0.1)),
        ..default()
    });
    parent.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(Vec3::new(x + direction * 0.25, y, 0.0))
            .with_rotation(Quat::from_rotation_z(-angle.to_radians()))
            .with_scale(Vec3::new(0.6, 0.1, 0.1)),
        ..default()
    });
}

fn spawn_formation(
    commands: &mut Commands,
    formation: Entity,
    count: i32,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) {
    commands.entity(formation).despawn_descendants();
    let count = count.max(0) as usize;
    let columns = (count as f32).sqrt().ceil().max(1.0) as usize;
    let spacing = 0.55;
    for index in 0..count {
        let row = index / columns;
        let col = index % columns;
        let offset_x = (col as f32 - (columns as f32 - 1.0) * 0.5) * spacing;
        let offset_z = -(row as f32) * spacing;
        commands.entity(formation).with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(offset_x, 0.0, offset_z)),
                ..default()
            });
        });
    }
}

fn update_swipe_axis(
    mut swipe: ResMut<SwipeState>,
    mut touch_events: EventReader<TouchInput>,
    mut cursor_events: EventReader<CursorMoved>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    swipe.axis = 0.0;
    if let Ok(window) = windows.get_single() {
        for event in touch_events.read() {
            match event.phase {
                TouchPhase::Started => {
                    swipe.dragging = true;
                    swipe.last_position = event.position;
                }
                TouchPhase::Moved if swipe.dragging => {
                    let delta = event.position - swipe.last_position;
                    swipe.last_position = event.position;
                    swipe.axis = (delta.x / window.width() as f32) * 6.0;
                }
                TouchPhase::Ended | TouchPhase::Canceled => {
                    swipe.dragging = false;
                }
                _ => {}
            }
        }

        if mouse_buttons.pressed(MouseButton::Left) {
            for event in cursor_events.read() {
                if !swipe.dragging {
                    swipe.dragging = true;
                    swipe.last_position = event.position;
                    continue;
                }
                let delta = event.position - swipe.last_position;
                swipe.last_position = event.position;
                swipe.axis = (delta.x / window.width() as f32) * 6.0;
            }
        } else {
            swipe.dragging = false;
        }
    }
}

fn update_input_axis(
    keys: Res<ButtonInput<KeyCode>>,
    swipe: Res<SwipeState>,
    mut axis: ResMut<InputAxis>,
) {
    let mut value = 0.0;
    if keys.pressed(KeyCode::ArrowLeft) {
        value -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        value += 1.0;
    }
    value += swipe.axis;
    axis.value = value.clamp(-1.0, 1.0);
}

fn move_player(
    time: Res<Time>,
    axis: Res<InputAxis>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&mut Transform, &CrowdCount), With<Player>>,
) {
    if !game_state.running {
        return;
    }

    for (mut transform, count) in player_query.iter_mut() {
        if count.current <= 0 {
            game_state.running = false;
            game_state.outcome = Some(GameOutcome::Lose);
            return;
        }
        transform.translation.z += PLAYER_SPEED * time.delta_seconds();
        transform.translation.x += -axis.value * LATERAL_SPEED * time.delta_seconds();
        let limit = TRACK_WIDTH * 0.5 - PLAYER_MARGIN;
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
    }
}

fn handle_collisions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&Transform, &mut CrowdCount), With<Player>>,
    mut gate_query: Query<(Entity, &Transform, &Gate)>,
    mut obstacle_query: Query<(Entity, &Transform, &Obstacle)>,
    finish_query: Query<(Entity, &Transform), With<FinishLine>>,
    mut enemy_query: Query<(
        Entity,
        &Transform,
        &mut EnemyCount,
        &EnemyGroup,
        Option<&Boss>,
    )>,
) {
    if !game_state.running {
        return;
    }
    let (player_transform, mut crowd) = match player_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    let player_pos = player_transform.translation;

    for (entity, transform, gate) in gate_query.iter_mut() {
        if (player_pos.z - transform.translation.z).abs() < COLLISION_RANGE
            && (player_pos.x - transform.translation.x).abs() < gate.width * 0.5
        {
            match gate.kind {
                GateKind::Add(amount) => crowd.current += amount,
                GateKind::Multiply(multiplier) => crowd.current *= multiplier,
            }
            commands.entity(entity).despawn_recursive();
            break;
        }
    }

    for (entity, transform, obstacle) in obstacle_query.iter_mut() {
        if (player_pos.z - transform.translation.z).abs() < COLLISION_RANGE
            && (player_pos.x - transform.translation.x).abs() < obstacle.width * 0.5
        {
            crowd.current -= obstacle.damage;
            commands.entity(entity).despawn_recursive();
            break;
        }
    }

    for (entity, transform) in finish_query.iter() {
        if (player_pos.z - transform.translation.z).abs() < COLLISION_RANGE
            && (player_pos.x - transform.translation.x).abs() < TRACK_WIDTH * 0.5
        {
            let boss_required = {
                let mut required = 0;
                for (_, _, _, _, boss) in enemy_query.iter_mut() {
                    if let Some(boss) = boss {
                        required = required.max(boss.required);
                    }
                }
                required
            };
            let win = crowd.current >= boss_required;
            game_state.running = false;
            game_state.outcome = Some(if win {
                GameOutcome::Win
            } else {
                GameOutcome::Lose
            });
            commands.entity(entity).despawn_recursive();
            return;
        }
    }

    for (entity, transform, mut enemy_count, enemy, boss) in enemy_query.iter_mut() {
        if (player_pos.z - transform.translation.z).abs() < COLLISION_RANGE
            && (player_pos.x - transform.translation.x).abs() < enemy.width * 0.5
        {
            if let Some(boss) = boss {
                let win = crowd.current >= boss.required;
                game_state.running = false;
                game_state.outcome = Some(if win {
                    GameOutcome::Win
                } else {
                    GameOutcome::Lose
                });
                if win {
                    commands.entity(entity).despawn_recursive();
                }
                break;
            }
            if crowd.current > enemy_count.current {
                crowd.current -= enemy_count.current;
                enemy_count.current = 0;
                commands.entity(entity).despawn_recursive();
            } else {
                enemy_count.current -= crowd.current;
                crowd.current = 0;
            }
            break;
        }
    }
}

fn update_end_screen_ui(
    game_state: Res<GameState>,
    end_screen: Res<EndScreenUi>,
    ui_assets: Res<UiAssets>,
    mut progress: ResMut<LevelProgress>,
    current_level: Res<CurrentLevel>,
    mut label_query: Query<
        (&mut Text, &mut Visibility),
        (
            Without<RestartButton>,
            Without<EndScreenNextButton>,
            Without<EndScreenMenuButton>,
            Without<EndScreenRoot>,
        ),
    >,
    mut button_visibility_query: Query<
        &mut Visibility,
        (
            Or<(
                With<RestartButton>,
                With<EndScreenNextButton>,
                With<EndScreenMenuButton>,
            )>,
            Without<EndScreenRoot>,
        ),
    >,
    mut root_query: Query<&mut Style, With<EndScreenRoot>>,
) {
    if let Ok(mut style) = root_query.get_mut(end_screen.root) {
        style.display = if game_state.outcome.is_some() {
            Display::Flex
        } else {
            Display::None
        };
    }

    if let Ok((mut text, mut visibility)) = label_query.get_mut(end_screen.label) {
        if let Some(outcome) = game_state.outcome {
            let (value, color) = match outcome {
                GameOutcome::Win => {
                    progress.completed[current_level.index] = true;
                    ("WIN", Color::rgb(0.3, 0.9, 0.55))
                }
                GameOutcome::Lose => ("LOSE", Color::rgb(0.95, 0.4, 0.4)),
            };
            text.sections = vec![TextSection::new(
                value,
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 72.0,
                    color,
                },
            )];
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    let mut button_entities = vec![end_screen.restart_button, end_screen.menu_button];
    if let Some(next_button) = end_screen.next_button {
        button_entities.push(next_button);
    }

    for button_entity in button_entities {
        if let Ok(mut visibility) = button_visibility_query.get_mut(button_entity) {
            *visibility = if game_state.outcome.is_some() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn reset_level(
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
    game_state: &mut GameState,
    axis: &mut InputAxis,
    swipe: &mut SwipeState,
    current_level: &CurrentLevel,
    level_query: &Query<Entity, With<LevelEntity>>,
) {
    for entity in level_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    levels::spawn_level(current_level.index, commands, meshes, materials);
    game_state.running = true;
    game_state.outcome = None;
    axis.value = 0.0;
    swipe.dragging = false;
    swipe.axis = 0.0;
}

fn handle_ui_restart(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut game_state: ResMut<GameState>,
    mut axis: ResMut<InputAxis>,
    mut swipe: ResMut<SwipeState>,
    meshes: Res<MeshAssets>,
    materials: Res<MaterialAssets>,
    current_level: Res<CurrentLevel>,
    level_query: Query<Entity, With<LevelEntity>>,
) {
    if game_state.outcome.is_none() {
        return;
    }

    let normal = Color::rgb(0.18, 0.2, 0.22);
    let hovered = Color::rgb(0.28, 0.3, 0.34);
    let pressed = Color::rgb(0.4, 0.42, 0.46);

    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = pressed.into();
                reset_level(
                    &mut commands,
                    &meshes,
                    &materials,
                    &mut game_state,
                    &mut axis,
                    &mut swipe,
                    &current_level,
                    &level_query,
                );
            }
            Interaction::Hovered => {
                *color = hovered.into();
            }
            Interaction::None => {
                *color = normal.into();
            }
        }
    }
}

fn handle_restart(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut touch_events: EventReader<TouchInput>,
    mut game_state: ResMut<GameState>,
    mut axis: ResMut<InputAxis>,
    mut swipe: ResMut<SwipeState>,
    meshes: Res<MeshAssets>,
    materials: Res<MaterialAssets>,
    current_level: Res<CurrentLevel>,
    level_query: Query<Entity, With<LevelEntity>>,
) {
    if game_state.outcome.is_none() {
        return;
    }

    let mut restart =
        keys.just_pressed(KeyCode::Space) || mouse_buttons.just_pressed(MouseButton::Left);
    for event in touch_events.read() {
        if event.phase == TouchPhase::Started {
            restart = true;
            break;
        }
    }

    if !restart {
        return;
    }

    reset_level(
        &mut commands,
        &meshes,
        &materials,
        &mut game_state,
        &mut axis,
        &mut swipe,
        &current_level,
        &level_query,
    );
}

fn handle_ui_next_level(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EndScreenNextButton>),
    >,
    mut current_level: ResMut<CurrentLevel>,
    mut game_state: ResMut<GameState>,
    mut axis: ResMut<InputAxis>,
    mut swipe: ResMut<SwipeState>,
    meshes: Res<MeshAssets>,
    materials: Res<MaterialAssets>,
    level_query: Query<Entity, With<LevelEntity>>,
) {
    if game_state.outcome.is_none() {
        return;
    }

    let (normal, hovered, pressed) = next_button_colors();
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = pressed.into();
                current_level.index = (current_level.index + 1).min(LEVEL_COUNT - 1);
                reset_level(
                    &mut commands,
                    &meshes,
                    &materials,
                    &mut game_state,
                    &mut axis,
                    &mut swipe,
                    &current_level,
                    &level_query,
                );
            }
            Interaction::Hovered => {
                *color = hovered.into();
            }
            Interaction::None => {
                *color = normal.into();
            }
        }
    }
}

fn handle_ui_back_to_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EndScreenMenuButton>),
    >,
    game_state: Res<GameState>,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    if game_state.outcome.is_none() {
        return;
    }

    let (normal, hovered, pressed) = menu_button_colors();
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = pressed.into();
                next_state.set(AppScreen::MainMenu);
            }
            Interaction::Hovered => {
                *color = hovered.into();
            }
            Interaction::None => {
                *color = normal.into();
            }
        }
    }
}

fn update_level_label(
    current_level: Res<CurrentLevel>,
    ui_assets: Res<UiAssets>,
    mut label_query: Query<&mut Text, With<LevelLabel>>,
) {
    if !current_level.is_changed() {
        return;
    }

    let label = LEVEL_NAMES
        .get(current_level.index)
        .copied()
        .unwrap_or(LEVEL_NAMES[0]);
    for mut text in label_query.iter_mut() {
        text.sections = vec![TextSection::new(
            label,
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 24.0,
                color: Color::rgb(0.92, 0.94, 1.0),
            },
        )];
    }
}

fn handle_keyboard_quit(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

fn sync_displays(
    mut commands: Commands,
    player_query: Query<&CrowdCount, With<Player>>,
    enemy_query: Query<&EnemyCount, With<EnemyGroup>>,
    meshes: Res<MeshAssets>,
    mut display_query: Query<(Entity, &Parent, &mut NumberDisplay)>,
) {
    for (entity, parent, mut display) in display_query.iter_mut() {
        let new_value = match display.kind {
            DisplayKind::Player => player_query
                .get(parent.get())
                .map(|count| count.current)
                .ok(),
            DisplayKind::Enemy => enemy_query
                .get(parent.get())
                .map(|count| count.current)
                .ok(),
        };

        if let Some(value) = new_value {
            if display.value != value {
                display.value = value.max(0);
                rebuild_display(&mut commands, entity, &display, meshes.unit_cube.clone());
            }
        }
    }
}

fn refresh_player_formation(
    mut commands: Commands,
    player_query: Query<(Entity, &CrowdCount, &Children), (With<Player>, Changed<CrowdCount>)>,
    formation_query: Query<Entity, With<CrowdFormation>>,
    meshes: Res<MeshAssets>,
    materials: Res<MaterialAssets>,
) {
    for (_entity, count, children) in player_query.iter() {
        if let Some(formation) = children
            .iter()
            .find(|child| formation_query.contains(**child))
        {
            spawn_formation(
                &mut commands,
                *formation,
                count.current,
                meshes.stickman.clone(),
                materials.player.clone(),
            );
        }
    }
}

fn refresh_enemy_formation(
    mut commands: Commands,
    enemy_query: Query<(Entity, &EnemyCount, &Children), (With<EnemyGroup>, Changed<EnemyCount>)>,
    formation_query: Query<Entity, With<CrowdFormation>>,
    meshes: Res<MeshAssets>,
    materials: Res<MaterialAssets>,
) {
    for (_entity, count, children) in enemy_query.iter() {
        if let Some(formation) = children
            .iter()
            .find(|child| formation_query.contains(**child))
        {
            spawn_formation(
                &mut commands,
                *formation,
                count.current,
                meshes.stickman.clone(),
                materials.enemy.clone(),
            );
        }
    }
}

fn follow_camera(
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, With<FollowCamera>>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };
    let target = player_transform.translation;
    for mut transform in camera_query.iter_mut() {
        transform.translation = Vec3::new(target.x, 18.0, target.z - 20.0);
        transform.look_at(Vec3::new(target.x, 0.0, target.z + 15.0), Vec3::Y);
    }
}

fn spawn_static_label(
    commands: &mut Commands,
    meshes: &MeshAssets,
    parent: Entity,
    prefix: LabelPrefix,
    value: i32,
    material: Handle<StandardMaterial>,
    offset: Vec3,
) {
    let display = NumberDisplay {
        value,
        prefix,
        material: material.clone(),
        kind: DisplayKind::Player,
    };
    let label = commands
        .spawn((
            SpatialBundle::from_transform(
                Transform::from_translation(offset)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ),
            NumberDisplay {
                value,
                prefix,
                material,
                kind: DisplayKind::Player,
            },
        ))
        .id();

    commands.entity(parent).add_child(label);
    rebuild_display(commands, label, &display, meshes.unit_cube.clone());
}

fn rebuild_display(
    commands: &mut Commands,
    entity: Entity,
    display: &NumberDisplay,
    mesh: Handle<Mesh>,
) {
    commands.entity(entity).despawn_descendants();

    let digits = display.value.max(0).to_string();
    let total_digits = digits.len() as f32;

    let symbol_width = if display.prefix == LabelPrefix::None {
        0.0
    } else {
        DIGIT_WIDTH + SYMBOL_GAP
    };
    let total_width = total_digits * (DIGIT_WIDTH + DIGIT_GAP) - DIGIT_GAP + symbol_width;
    let mut x_offset = -total_width * 0.5;

    commands.entity(entity).with_children(|parent| {
        if display.prefix != LabelPrefix::None {
            spawn_symbol(
                parent,
                mesh.clone(),
                display.material.clone(),
                display.prefix,
                x_offset,
            );
            x_offset += DIGIT_WIDTH + SYMBOL_GAP;
        }

        for ch in digits.chars() {
            if let Some(digit) = ch.to_digit(10) {
                spawn_digit(
                    parent,
                    mesh.clone(),
                    display.material.clone(),
                    digit as u8,
                    x_offset,
                );
                x_offset += DIGIT_WIDTH + DIGIT_GAP;
            }
        }
    });
}

fn spawn_digit(
    parent: &mut ChildBuilder,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    digit: u8,
    x_offset: f32,
) {
    let segments = digit_segments(digit);
    let half_w = DIGIT_WIDTH * 0.5;
    let half_h = DIGIT_HEIGHT * 0.5;
    let segment_thickness = 0.12;
    let vertical_height = DIGIT_HEIGHT * 0.5;

    let positions = [
        (
            Vec3::new(0.0, half_h, 0.0),
            Vec3::new(DIGIT_WIDTH, segment_thickness, DIGIT_DEPTH),
        ),
        (
            Vec3::new(half_w, half_h * 0.5, 0.0),
            Vec3::new(segment_thickness, vertical_height, DIGIT_DEPTH),
        ),
        (
            Vec3::new(half_w, -half_h * 0.5, 0.0),
            Vec3::new(segment_thickness, vertical_height, DIGIT_DEPTH),
        ),
        (
            Vec3::new(0.0, -half_h, 0.0),
            Vec3::new(DIGIT_WIDTH, segment_thickness, DIGIT_DEPTH),
        ),
        (
            Vec3::new(-half_w, -half_h * 0.5, 0.0),
            Vec3::new(segment_thickness, vertical_height, DIGIT_DEPTH),
        ),
        (
            Vec3::new(-half_w, half_h * 0.5, 0.0),
            Vec3::new(segment_thickness, vertical_height, DIGIT_DEPTH),
        ),
        (
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(DIGIT_WIDTH, segment_thickness, DIGIT_DEPTH),
        ),
    ];

    for (index, (pos, scale)) in positions.iter().enumerate() {
        if !segments[index] {
            continue;
        }
        parent.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(x_offset, 0.0, 0.0) + *pos)
                .with_scale(*scale),
            ..default()
        });
    }
}

fn spawn_symbol(
    parent: &mut ChildBuilder,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    prefix: LabelPrefix,
    x_offset: f32,
) {
    match prefix {
        LabelPrefix::Plus => {
            parent.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(x_offset, 0.0, 0.0))
                    .with_scale(Vec3::new(DIGIT_WIDTH, 0.12, DIGIT_DEPTH)),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(x_offset, 0.0, 0.0))
                    .with_scale(Vec3::new(0.12, DIGIT_HEIGHT * 0.8, DIGIT_DEPTH)),
                ..default()
            });
        }
        LabelPrefix::Minus => {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(x_offset, 0.0, 0.0))
                    .with_scale(Vec3::new(DIGIT_WIDTH, 0.12, DIGIT_DEPTH)),
                ..default()
            });
        }
        LabelPrefix::Times => {
            parent.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(x_offset, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_z(45.0_f32.to_radians()))
                    .with_scale(Vec3::new(DIGIT_WIDTH, 0.12, DIGIT_DEPTH)),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(x_offset, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_z(-45.0_f32.to_radians()))
                    .with_scale(Vec3::new(DIGIT_WIDTH, 0.12, DIGIT_DEPTH)),
                ..default()
            });
        }
        LabelPrefix::None => {}
    }
}

fn digit_segments(digit: u8) -> [bool; 7] {
    match digit {
        0 => [true, true, true, true, true, true, false],
        1 => [false, true, true, false, false, false, false],
        2 => [true, true, false, true, true, false, true],
        3 => [true, true, true, true, false, false, true],
        4 => [false, true, true, false, false, true, true],
        5 => [true, false, true, true, false, true, true],
        6 => [true, false, true, true, true, true, true],
        7 => [true, true, true, false, false, false, false],
        8 => [true, true, true, true, true, true, true],
        9 => [true, true, true, true, false, true, true],
        _ => [false; 7],
    }
}
