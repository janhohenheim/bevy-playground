#[cfg(feature = "debug")]
use colored::Colorize;

/// Minesweeper tile
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tile {
    /// Empty tile
    Empty,
    /// Mine tile
    Mine,
    /// Neighbor tile counting surrounding mines
    Neighbor(u8),
}

impl Tile {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Self::Empty => " ".normal(),
                Self::Mine => "*".bright_red(),
                Self::Neighbor(n) => match n {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => n.to_string().red(),
                },
            }
        )
    }
}
