pub mod board;
pub mod patterns;
pub mod tetromino;

use bevy::{audio::PlaybackMode, prelude::*};

use bevy::window::PrimaryWindow;
use board::{Board, Movement};
use tetromino::Tetromino;

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource)]
pub struct GameState {
    pub board: Board,
    pub timer: Timer,
    pub next: Tetromino,
    pub score: usize,
    pub count: usize,
}

impl Default for GameState {
    
    fn default() -> Self {
        let mut next = Tetromino::random();
        next.i = 4;
        next.j = -6;

        GameState {
            board: Board::new(20, 15),
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            next,
            count: 0,
            score: 0,
        }
    }
}

const COLUMNS: isize = 15;
const ROWS: isize = 20;
const TILE_SIZE: f32 = 30.0;

#[derive(Component)]
pub struct BoardComponent;

#[derive(Component)]
pub struct TetrominoComponent {}

#[derive(Component)]
pub struct TextComponent {}

#[derive(Component)]
pub struct ScheduledSound {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Block {}

#[derive(Component)]
pub struct Tile;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum State {
    MainMenu,
    #[default]
    Running,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "R-Tetris".into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .init_resource::<GameState>()
        .add_state::<State>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, handle_main_menu.run_if(in_state(State::MainMenu)))
        .add_systems(Update, handle_game_over.run_if(in_state(State::GameOver)))
        .add_systems(OnEnter(State::MainMenu), on_menu_enter)
        .add_systems(
            OnEnter(State::Running),
            (draw_rectangle, spawn_components, on_game_start),
        )
        .add_systems(Update, update_tetromino.run_if(in_state(State::Running)))
        .add_systems(OnEnter(State::GameOver), on_game_over)
        .run();
}

fn handle_main_menu(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<State>>) {
    if keyboard_input.just_released(KeyCode::Space) {
        app_state.set(State::Running);
    }
}

fn draw_rectangle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: ResMut<GameState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    draw_rect(
        &mut commands,
        &asset_server,
        &window_query,
        [0, 0, ROWS - 1, COLUMNS - 1],
        &[],
    );

    draw_text(
        &mut commands,
        &asset_server,
        "Score",
        0, -6,
        width, height
    );

    let score = format!("     {}", state.score);
    
    draw_text(
        &mut commands,
        &asset_server,
        &score,
        1, -6,
        width, height
    );

    draw_rect(
        &mut commands,
        &asset_server,
        &window_query,
        [0, -8, 2, 6],
        &[2, 3, 4],
    );

    draw_text(
        &mut commands,
        &asset_server,
        " Next",
        3, -6,
        width, height
    );

    draw_rect(
        &mut commands,
        &asset_server,
        &window_query,
        [3, -8, 6, 6],
        &[2, 3, 4],
    );    
}

fn draw_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text: &str,
    i: isize,
    j: isize,
    width: f32,
    height: f32,
) {
    let w = TILE_SIZE * COLUMNS as f32;
    let h = TILE_SIZE * ROWS as f32;    

    let x = (width/2.0) - (w/2.0) + (TILE_SIZE * j as f32);
    let y = (height/2.0) - (h/2.0) + (TILE_SIZE * i as f32);

    commands.spawn((
        TextComponent {},
        TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(y),
            left: Val::Px(x),
            margin: UiRect {top: Val::Px(-15.0), ..default()},
            ..default()
        }),
    ));
}

fn draw_rect(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    coordinates: [isize; 4],
    ignore: &[isize]
) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    let [i, j, rows, columns] = coordinates;

    let mut transform = to_transform(i, j, width, height);
    commands.spawn((
        SpriteBundle {
            transform: Transform { ..transform },
            texture: asset_server.load("sprites/border_topleft.png"),
            ..default()
        },
        Tile,
    ));
    let mut t = Transform { ..transform };
    t.translation.x += TILE_SIZE * columns as f32;
    commands.spawn((
        SpriteBundle {
            transform: t,
            texture: asset_server.load("sprites/border_topright.png"),
            ..default()
        },
        Tile,
    ));

    transform.translation.y -= TILE_SIZE;

    for _ in 1..rows {
        commands.spawn((
            SpriteBundle {
                transform: Transform { ..transform },
                texture: asset_server.load("sprites/border.png"),
                ..default()
            },
            Tile,
        ));

        let mut t = Transform { ..transform };
        t.translation.x += TILE_SIZE * columns as f32;
        commands.spawn((
            SpriteBundle {
                transform: t,
                texture: asset_server.load("sprites/border_right.png"),
                ..default()
            },
            Tile,
        ));

        transform.translation.y -= TILE_SIZE;
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform { ..transform },
            texture: asset_server.load("sprites/border_bottomleft.png"),
            ..default()
        },
        Tile,
    ));

    let mut t = Transform { ..transform };
    t.translation.x += TILE_SIZE * columns as f32;

    commands.spawn((
        SpriteBundle {
            transform: t,
            texture: asset_server.load("sprites/border_bottomright.png"),
            ..default()
        },
        Tile,
    ));

    let mut transform = to_transform(i, j, width, height);
    transform.translation.x += TILE_SIZE;

    for _j in 1..columns {
        let mut skip = false;

        for to_skip in ignore {
            if *to_skip == _j {
                skip = true;
            }
        }
        if !skip {
            commands.spawn((
                SpriteBundle {
                    transform: Transform { ..transform },
                    texture: asset_server.load("sprites/border_top.png"),
                    ..default()
                },
                Tile,
            ));
        }

        let mut t = Transform { ..transform };
        t.translation.y -= TILE_SIZE * rows as f32;
        commands.spawn((
            SpriteBundle {
                transform: t,
                texture: asset_server.load("sprites/border_bottom.png"),
                ..default()
            },
            Tile,
        ));

        transform.translation.x += TILE_SIZE;
    }    
}

fn handle_game_over(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<State>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut scheduled_sound: Query<(Entity, &mut ScheduledSound)>,
) {
    if keyboard_input.just_released(KeyCode::Space) {
        app_state.set(State::MainMenu);
    }

    for (ent, mut sound) in scheduled_sound.iter_mut() {
        sound.timer.tick(time.delta());

        if sound.timer.finished() {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/oogas.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
            });

            commands.entity(ent).despawn();
        }
    }
}

fn on_game_start(mut commands: Commands, text: Query<Entity, With<TextComponent>>) {
    for ent in text.iter() {
        commands.entity(ent).despawn();
    }
}

fn on_menu_enter(
    mut commands: Commands,
    text: Query<Entity, With<TextComponent>>,
    asset_server: Res<AssetServer>,
) {
    for ent in text.iter() {
        commands.entity(ent).despawn();
    }

    commands.spawn((
        TextComponent {},
        TextBundle::from_section(
            "Ó O GÁAAAAAAAAAAS!!!!!! Aperte espaço para iniciar o jogo.",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
    ));
}

fn on_game_over(
    mut commands: Commands,
    tetromino: Query<Entity, With<TetrominoComponent>>,
    blocks: Query<Entity, With<Block>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextComponent {},
        TextBundle::from_section(
            "Perdeu, mané! Não amola. Aperte espaço pra voltar pro menu.",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
    ));

    for ent in &blocks {
        commands.entity(ent).despawn();
    }

    for ent in &tetromino {
        commands.entity(ent).despawn();
    }

    for i in 1..10 {
        commands.spawn(ScheduledSound {
            timer: Timer::from_seconds(0.2 * i as f32, TimerMode::Once),
        });
    }
}

pub fn to_transform(i: isize, j: isize, width: f32, height: f32) -> Transform {
    let x_m = width / 2.0;
    let y_m = height / 2.0;

    let x: f32 = x_m - ((COLUMNS as f32 * TILE_SIZE) / 2.0) + (j as f32 * TILE_SIZE);
    let y: f32 = y_m + ((ROWS as f32 * TILE_SIZE) / 2.0) - (i as f32 * TILE_SIZE);

    Transform::from_xyz(x, y, 0.0)
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_components(
    mut commands: Commands,
    state: Res<GameState>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    spawn_blocks(&state, &mut commands, width, height, &asset_server);

    spawn_tetromino(&mut commands, width, height, &asset_server, &state.board.tetromino);

    spawn_tetromino(&mut commands, width, height, &asset_server, &state.next);
}

fn spawn_blocks(
    state: &GameState,
    commands: &mut Commands<'_, '_>,
    width: f32,
    height: f32,
    asset_server: &Res<'_, AssetServer>,
) {
    for (i, j) in state.board.blocks() {
        let skip = match (i, j) {
            (0, 0) => true,
            (0, col) if col == COLUMNS - 1 => true,
            (row, 0) if row == ROWS - 1 => true,
            (row, col) if row == ROWS - 1 && col == COLUMNS - 1 => true,
            (_, 0) => true,
            (_, col) if col == COLUMNS - 1 => true,
            (0, _) => true,
            (row, _) if row == ROWS - 1 => true,
            _ => false,
        };

        if skip {
            continue;
        }

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    ..to_transform(i, j, width, height)
                },
                texture: asset_server.load("sprites/bujaum.png"),

                ..default()
            },
            Block {},
        ));
    }
}

fn spawn_tetromino(
    commands: &mut Commands<'_, '_>,
    width: f32,
    height: f32,
    asset_server: &Res<'_, AssetServer>,
    tetromino: &Tetromino

) {
    let rotation = tetromino.actual_rotation();
    for i in 0..4 {
        for j in 0..4 {
            let (i_board, j_board) = (tetromino.i + i, tetromino.j + j);

            if rotation[i as usize][j as usize] {
                let t = to_transform(i_board, j_board, width, height);

                commands.spawn((
                    SpriteBundle {
                        transform: t,
                        texture: asset_server.load("sprites/bujaum.png"),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    TetrominoComponent {},
                ));
            }
        }
    }
}

pub fn update_tetromino(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<GameState>,
    tetromino: Query<Entity, With<TetrominoComponent>>,
    blocks: Query<Entity, With<Block>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut app_state: ResMut<NextState<State>>,
) {
    state.timer.tick(time.delta());

    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    let mut movement: Option<Movement> = None;

    if keyboard_input.just_released(KeyCode::Up) {
        movement = Some(Movement::RotateRight);
    }

    if keyboard_input.pressed(KeyCode::Down) {
        movement = Some(Movement::Down);
    }

    if keyboard_input.just_released(KeyCode::Left) {
        movement = Some(Movement::Left);
    }

    if keyboard_input.just_released(KeyCode::Right) {
        movement = Some(Movement::Right);
    }

    if state.timer.finished() {
        movement = Some(Movement::Down);
    }

    if let Some(movement) = movement {
        let status = state.board.apply_movement(movement);

        if !status && movement == Movement::Down {
            state.count += 1;
            state.board.merge();
            let completed = state.board.check_completed_rows();

            if completed > 0 {
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/oogas.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                });
            } else {
                let sound = state.count % 2;
                let path = format!("sounds/sound{}.ogg", sound);

                commands.spawn(AudioBundle {
                    source: asset_server.load(path),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                });
            }

            state.next.i = 2;
            state.next.j = 2;
            state.board.tetromino = state.next;
            let mut next = Tetromino::random();
            next.i = 4;
            next.j = -6;
            state.next = next;
    

            if state.board.overlaps() {
                app_state.set(State::GameOver);
                *state = GameState::default();
                return;
            }

            for ent in &blocks {
                commands.entity(ent).despawn();
            }

            spawn_blocks(&state, &mut commands, width, height, &asset_server);
        }

        for ent in &tetromino {
            commands.entity(ent).despawn();
        }

        spawn_tetromino(&mut commands, width, height, &asset_server, &state.board.tetromino);
        spawn_tetromino(&mut commands, width, height, &asset_server, &state.next);
    }
}
