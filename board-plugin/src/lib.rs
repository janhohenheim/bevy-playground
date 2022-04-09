pub mod components;
pub mod resources;
mod systems;

use bevy::log;
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::create_board);
        log::info!("Loaded board plugin");
    }
}
