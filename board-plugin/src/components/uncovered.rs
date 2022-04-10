use bevy::prelude::*;

/// Uncovered tile marker
/// Tells the game to uncover the tile in the next frame, marker is removed after
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncovered;
