pub mod board;
pub mod patterns;
pub mod tetromino;

use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use board::{Board, Movement};
use tetromino::Tetromino;

#[derive(Resource)]
pub struct GameState {
    pub board: Board,
    pub timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            board: Board::new(20, 15),
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

const COLUMNS: isize = 16;
const ROWS: isize = 20;
const TILE_SIZE: f32 = 30.0;

#[derive(Component)]
pub struct BoardComponent;

#[derive(Component)]
pub struct TetrominoComponent {}

#[derive(Component)]
pub struct Block {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .add_systems(Startup, (spawn_camera, spawn_components))
        .add_systems(Update, update_tetromino)
        .run();
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

    spawn_tetromino(commands, &state, width, height, asset_server);
}

fn spawn_blocks(
    state: &GameState,
    commands: &mut Commands<'_, '_>,
    width: f32,
    height: f32,
    asset_server: &Res<'_, AssetServer>,
) {
    for (i, j) in state.board.blocks() {
        commands.spawn((
            SpriteBundle {
                transform: to_transform(i, j, width, height),
                texture: asset_server.load("sprites/bujaum.png"),
                ..default()
            },
            Block {},
        ));
    }
}

fn spawn_tetromino(
    mut commands: Commands<'_, '_>,
    state: &GameState,
    width: f32,
    height: f32,
    asset_server: Res<'_, AssetServer>,
) {
    let tetromino = &state.board.tetromino;

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
) {
    state.timer.tick(time.delta());

    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    let mut movement: Option<Movement> = None;

    if keyboard_input.just_released(KeyCode::Up) {
        movement = Some(Movement::RotateRight);
    }

    if keyboard_input.just_released(KeyCode::Down) {
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
            state.board.merge();
            state.board.tetromino = Tetromino::random();

            for ent in &blocks {
                commands.entity(ent).despawn();
            }

            spawn_blocks(&state, &mut commands, width, height, &asset_server);
        }

        for ent in &tetromino {
            commands.entity(ent).despawn();
        }

        spawn_tetromino(commands, &state, width, height, asset_server);
    }
}
