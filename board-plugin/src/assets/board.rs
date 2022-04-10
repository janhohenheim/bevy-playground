use crate::assets::SpriteMaterial;
use bevy::prelude::*;

/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[derive(Debug, Clone)]
pub struct BoardAssets {
    pub label: String,
    pub board_material: SpriteMaterial,
    pub tile_material: SpriteMaterial,
    pub covered_tile_material: SpriteMaterial,
    pub mine_material: SpriteMaterial,
    pub flag_material: SpriteMaterial,
    /// Used for the tiles neighboring mines
    pub neighbor_font: Handle<Font>,
    /// First one will be used for neighbors of a single mine, second color for neighbors of two mines, etc.
    /// After running out of numbers, the last color will be used.
    /// If no colors are provided, `Color::WHITE` is used.
    pub mine_counter_colors: Vec<Color>,
}
impl BoardAssets {
    /// Default bomb counter color set
    pub fn default_colors() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::PURPLE,
        ]
    }

    /// Safely retrieves the color matching a bomb counter.
    /// After running out of numbers, the last color will be used.
    /// If no colors are provided, `Color::WHITE` is used.
    pub fn get_mine_counter_color(&self, count: u8) -> Color {
        let index = count.saturating_sub(1) as usize;
        match self.mine_counter_colors.get(index) {
            Some(color) => *color,
            None => self
                .mine_counter_colors
                .last()
                .copied()
                .unwrap_or(Color::WHITE),
        }
    }
}
