use bevy::log;
use bevy::prelude::*;
use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
    Restarting,
    Out,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Minesweeper".to_string(),
        width: 800.,
        height: 600.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(camera_setup);

    #[cfg(feature = "debug")]
    // If this is moved further down, we get an error for some reason
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_state(AppState::InGame)
        .add_plugin(BoardPlugin::<AppState> {
            current_state: AppState::InGame,
        })
        .add_system(state_handler)
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            mine_count: 40,
            tile_padding: 3.,
            safe_start: true,
            ..Default::default()
        });

    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state
                .set(AppState::Out)
                .unwrap_or_else(|error| panic!("Failed to clear game: {}", error))
        }
    }

    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        log::info!("loading game");
        state
            .set(AppState::Restarting)
            .unwrap_or_else(|error| panic!("Failed to restart game: {}", error))
    }

    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("pause detected");

        if state.current() == &AppState::Paused {
            log::info!("resuming game");
            state
                .pop()
                .unwrap_or_else(|error| panic!("Failed to resume game: {}", error))
        } else {
            log::info!("pausing game");
            state
                .push(AppState::Paused)
                .unwrap_or_else(|error| panic!("Failed to pause game: {}", error))
        }
    }
}
