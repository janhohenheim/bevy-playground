mod bounds;
pub mod components;
pub mod resources;
mod systems;

use bevy::log;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
#[cfg(feature = "debug")]
use components::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::startup::create_board);
        log::info!("Loaded board plugin");
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>()
                .register_inspectable::<Neighbor>()
                .register_inspectable::<Mine>()
                .register_inspectable::<Uncovered>();
        }
    }
}
