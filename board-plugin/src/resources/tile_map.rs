use crate::{components::Coordinates, resources::tile::Tile};
use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut};

/// Base tile map
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TileMap {
    pub mine_count: u16,
    pub width: u16,
    pub height: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Generate an empty tile map
    pub fn empty(width: u16, height: u16) -> Self {
        Self {
            mine_count: 0,
            width,
            height,
            map: vec![vec![Tile::Empty; width as usize]; height as usize],
        }
    }

    /// Places mine and neighbor tiles
    pub fn place_mines(&mut self, mine_count: u16) -> &Self {
        self.mine_count = mine_count;
        let mut remaining_mines = mine_count;
        let mut rng = thread_rng();
        while remaining_mines > 0 {
            let x = rng.gen_range(0..self.width) as usize;
            let y = rng.gen_range(0..self.height) as usize;

            if self.map[y][x] == Tile::Empty {
                self.map[y][x] = Tile::Mine;
                remaining_mines -= 1;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let coordinates = Coordinates { x, y };
                let mine_count = self.neighbor_count_at(coordinates);
                if mine_count > 0 {
                    self[y as usize][x as usize] = Tile::Neighbor(mine_count);
                }
            }
        }

        self
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}x{}) with {} bombs:\n",
            self.width, self.height, self.mine_count
        );

        let line = "-".repeat((self.width + 2) as usize);
        buffer.push_str(&line);
        buffer.push('\n');

        for line in self.iter().rev() {
            buffer.push('|');
            for tile in line {
                buffer.push_str(&tile.console_output());
            }
            buffer.push_str("|\n");
        }

        buffer.push_str(&line);
        buffer
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |offset| coordinates + offset)
    }

    pub fn is_mine_at(&self, coordinates: Coordinates) -> bool {
        if self.is_out_of_bounds(coordinates) {
            return false;
        }

        self.map[coordinates.y as usize][coordinates.x as usize] == Tile::Mine
    }

    /// Counts the number of mines surrounding the given coordinates
    /// If there is a mine at the coordinates, the number of neighbors is defined as 0
    pub fn neighbor_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_mine_at(coordinates) {
            return 0;
        }

        self.safe_square_at(coordinates)
            .filter(|coordinates| self.is_mine_at(*coordinates))
            .count() as u8
    }

    fn is_out_of_bounds(&self, coordinates: Coordinates) -> bool {
        coordinates.x >= self.width || coordinates.y >= self.height
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

/// Delta coordinates for all 8 square neighbors
/// ```{bash}
/// *--------*-------*-------*
/// | -1, 1  | 0, 1  | 1, 1  |
/// |--------|-------|-------|
/// | -1, 0  | tile  | 1, 0  |
/// |--------|-------|-------|
/// | -1, -1 | 0, -1 | 1, -1 |
/// *--------*-------*-------*
/// ```
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
];
