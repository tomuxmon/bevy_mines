use bevy::log;
use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Reseting,
    Pause,
    Out,
}

fn main() {
    let mut app = App::new();
    app.add_state(AppState::InGame)
        // Window setup
        .insert_resource(WindowDescriptor {
            title: "Mine Sweeper!".to_string(),
            width: 700.,
            height: 800.,
            ..Default::default()
        })
        // Board setup
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 60,
            tile_padding: 3.0,
            safe_start: true,
            ..Default::default()
        })
        // Bevy default plugins
        .add_plugins(DefaultPlugins)
        // mine board plugin
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        })
        .add_system(state_handler)
        .add_startup_system(camera_setup);

    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if state.current() == &AppState::Reseting {
        state.set(AppState::InGame).unwrap();
        return;
    }
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("pause detected");
        if state.current() == &AppState::InGame {
            log::info!("entering pause");
            state.push(AppState::Pause).unwrap();
        }
        if state.current() == &AppState::Pause {
            log::info!("leaving pause");
            state.pop().unwrap();
        }
    }
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.current() == &AppState::Out || state.current() == &AppState::InGame {
            log::info!("reseting game");
            state.set(AppState::Reseting).unwrap();
        }
    }
}
