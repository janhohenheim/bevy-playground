use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, coordinates: Vec2) -> bool {
        coordinates.x >= self.position.x
            && coordinates.x < self.position.x + self.size.x
            && coordinates.y >= self.position.y
            && coordinates.y < self.position.y + self.size.y
    }
}
