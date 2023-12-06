use bevy::{audio::PlaybackMode, prelude::*};

use crate::board::Board;
use crate::tetromino::Tetromino;
use bevy::window::PrimaryWindow;

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use self::menu::MenuHandler;
use self::tetromino::{ScheduledSound, TetrominoHandler};

pub mod menu;
pub mod tetromino;

const COLUMNS: isize = 15;
const ROWS: isize = 20;
const TILE_SIZE: f32 = 30.0;

pub fn to_transform(i: isize, j: isize, width: f32, height: f32) -> Transform {
    let x_m = width / 2.0;
    let y_m = height / 2.0;

    let x: f32 = x_m - ((COLUMNS as f32 * TILE_SIZE) / 2.0) + (j as f32 * TILE_SIZE);
    let y: f32 = y_m + ((ROWS as f32 * TILE_SIZE) / 2.0) - (i as f32 * TILE_SIZE);

    Transform::from_xyz(x, y, 0.0)
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum State {
    MainMenu,
    #[default]
    Running,
    Paused,
    GameOver,
}

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

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
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

pub fn start_game() {
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
        .add_plugins(TetrominoHandler)
        .add_plugins(MenuHandler)        
        .add_state::<State>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, handle_game_over.run_if(in_state(State::GameOver)))
        .run();
}
