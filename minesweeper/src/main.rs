use bevy::prelude::*;
use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

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
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugin(BoardPlugin).insert_resource(BoardOptions {
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
