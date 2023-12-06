use crate::game::tetromino::TetrominoComponent;
use crate::game::State;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{
    tetromino::{Block, ScheduledSound},
    to_transform, GameState, COLUMNS, ROWS, TILE_SIZE,
};

#[derive(Component)]
pub struct TextComponent {}

#[derive(Component)]
pub struct Tile;

pub struct MenuHandler;

impl Plugin for MenuHandler {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_main_menu.run_if(in_state(State::MainMenu)))
            .add_systems(OnEnter(State::MainMenu), on_menu_enter)
            .add_systems(OnEnter(State::GameOver), on_game_over)
            .add_systems(
                OnEnter(State::Running),
                (draw_rectangle, on_game_start)
            );
    }
}

fn handle_main_menu(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<State>>) {
    if keyboard_input.just_released(KeyCode::Space) {
        app_state.set(State::Running);
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

fn on_game_start(mut commands: Commands, text: Query<Entity, With<TextComponent>>) {
    for ent in text.iter() {
        commands.entity(ent).despawn();
    }
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

    draw_text(&mut commands, &asset_server, "Score", 0, -6, width, height);

    let score = format!("     {}", state.score);

    draw_text(&mut commands, &asset_server, &score, 1, -6, width, height);

    draw_rect(
        &mut commands,
        &asset_server,
        &window_query,
        [0, -8, 2, 6],
        &[2, 3, 4],
    );

    draw_text(&mut commands, &asset_server, " Next", 3, -6, width, height);

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

    let x = (width / 2.0) - (w / 2.0) + (TILE_SIZE * j as f32);
    let y = (height / 2.0) - (h / 2.0) + (TILE_SIZE * i as f32);

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
            margin: UiRect {
                top: Val::Px(-15.0),
                ..default()
            },
            ..default()
        }),
    ));
}

fn draw_rect(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    coordinates: [isize; 4],
    ignore: &[isize],
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
