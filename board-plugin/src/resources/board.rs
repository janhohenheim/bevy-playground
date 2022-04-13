use crate::{bounds::Bounds2, components::Coordinates, resources::TileMap};
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub marked_tiles: Vec<Coordinates>,
    pub entity: Entity,
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

    /// Retrieve a tile entity if it is still covered and unmarked.
    pub fn get_covered_tile(&self, coordinates: Coordinates) -> Option<Entity> {
        if self.is_marked(coordinates) {
            None
        } else {
            self.covered_tiles.get(&coordinates).copied()
        }
    }

    /// Try to uncover a tile
    /// Will remove a mark without uncovering the tile and return `None`
    pub fn uncover_tile(&mut self, coordinates: Coordinates) -> Option<Entity> {
        if self.is_marked(coordinates) {
            self.unmark_tile(coordinates)
                .expect("Failed to unmark tile that is saved as marked tile");
            None
        } else {
            self.covered_tiles.remove(&coordinates)
        }
    }

    /// Retrieve adjacent covered tiles
    pub fn get_covered_neighbors(&self, coordinates: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coordinates)
            .filter_map(|coordinates| self.get_covered_tile(coordinates))
            .collect()
    }

    /// Toggles marked state of a tile and returns the new state
    pub fn toggle_mark(&mut self, coordinates: Coordinates) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(&coordinates)?;
        let new_state = if self.is_marked(coordinates) {
            self.unmark_tile(coordinates)
                .expect("Failed to unmark tile that is saved as marked tile");
            false
        } else {
            self.marked_tiles.push(coordinates);
            true
        };
        Some((entity, new_state))
    }

    fn unmark_tile(&mut self, coordinates: Coordinates) -> Option<Coordinates> {
        self.marked_tiles
            .drain_filter(|marked_coordinates| *marked_coordinates == coordinates)
            .next()
    }

    pub fn is_completed(&self) -> bool {
        self.tile_map.mine_count as usize == self.covered_tiles.len()
    }

    fn is_marked(&self, coordinates: Coordinates) -> bool {
        self.marked_tiles.contains(&coordinates)
    }
}
