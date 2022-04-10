use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, other: (i8, i8)) -> Self::Output {
        Self {
            x: (self.x as i16 + other.0 as i16) as u16,
            y: (self.y as i16 + other.1 as i16) as u16,
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn safely_handles_underflow_with_coordinates() {
        let coordinates = Coordinates { x: 0, y: 0 };
        let coordinates = coordinates - Coordinates { x: 1, y: 1 };
        assert_eq!(coordinates, Coordinates { x: 0, y: 0 });
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn panics_on_overflow_with_coordinates() {
        let coordinates = Coordinates {
            x: 40_000,
            y: 40_000,
        };
        let coordinates = coordinates
            + Coordinates {
                x: 40_000,
                y: 40_000,
            };
        assert_eq!(
            coordinates,
            Coordinates {
                x: 20_000,
                y: 20_000
            }
        );
    }

    #[test]
    fn underflows_with_i8() {
        let coordinates = Coordinates { x: 0, y: 0 };
        let coordinates = coordinates + (-1, -1);
        assert_eq!(coordinates, Coordinates { x: 65535, y: 65535 });
    }

    #[test]
    fn safely_handles_potential_overflows_with_i8() {
        let coordinates = Coordinates {
            x: 40_000,
            y: 40_000,
        };
        let coordinates = coordinates + (100, 100);
        assert_eq!(
            coordinates,
            Coordinates {
                x: 40_100,
                y: 40_100
            }
        );
    }
}
