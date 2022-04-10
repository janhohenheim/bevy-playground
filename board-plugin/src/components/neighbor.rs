use bevy::prelude::*;

/// Mine neighbor marker
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Neighbor {
    /// Number of neighboring mines
    pub count: u8,
}
