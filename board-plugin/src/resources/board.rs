use crate::{bounds::Bounds2, components::Coordinates, resources::TileMap};
use bevy::prelude::*;

#[derive(Debug)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
}

impl Board {
    /// Translate a mouse position to board coordinates
    pub fn convert_mouse_to_coordinates(
        &self,
        mouse_position: Vec2,
        window: &Window,
    ) -> Option<Coordinates> {
        // Window to world space
        // World space has origin at center of board, but window space has origin at bottom left
        let window_size = Vec2::new(window.width(), window.height());
        let world_position = mouse_position - window_size / 2.;

        if !self.bounds.in_bounds(world_position) {
            return None;
        }

        // World space to board space
        let absolute_coordinates = world_position - self.bounds.position;
        Some(Coordinates {
            x: (absolute_coordinates.x / self.tile_size) as u16,
            y: (absolute_coordinates.y / self.tile_size) as u16,
        })
    }
}
