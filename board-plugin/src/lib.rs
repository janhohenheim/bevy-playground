pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::{tile_map::TileMap, BoardOptions};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Loaded board plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete Board
    pub(crate) fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Option<Res<WindowDescriptor>>,
    ) {
        let options = match board_options {
            None => Default::default(),
            Some(options) => options.clone(),
        };
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.place_mines(options.mine_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}
