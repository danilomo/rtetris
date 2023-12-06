use game::start_game;

pub mod board;
pub mod game;
pub mod patterns;
pub mod tetromino;

fn main() {
    start_game();
    /*App::new()
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
    .run();*/
}
