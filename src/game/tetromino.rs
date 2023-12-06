use crate::board::Movement;
use crate::tetromino::Tetromino;
use bevy::window::PrimaryWindow;
use bevy::{audio::PlaybackMode, prelude::*};

use super::{to_transform, GameState, State, COLUMNS, ROWS};

pub struct TetrominoHandler;

impl Plugin for TetrominoHandler {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, update_tetromino.run_if(in_state(State::Running)))
        .add_systems(
            OnEnter(State::Running),
            spawn_components,
        );
    }
}

#[derive(Component)]
pub struct BoardComponent;

#[derive(Component)]
pub struct TetrominoComponent {}

#[derive(Component)]
pub struct ScheduledSound {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Block {}

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

    spawn_tetromino(
        &mut commands,
        width,
        height,
        &asset_server,
        &state.board.tetromino,
    );

    spawn_tetromino(&mut commands, width, height, &asset_server, &state.next);
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

        spawn_tetromino(
            &mut commands,
            width,
            height,
            &asset_server,
            &state.board.tetromino,
        );
        spawn_tetromino(&mut commands, width, height, &asset_server, &state.next);
    }
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
    tetromino: &Tetromino,
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
