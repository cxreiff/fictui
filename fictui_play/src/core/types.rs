use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::North => "north",
                Self::East => "east",
                Self::South => "south",
                Self::West => "west",
                Self::Up => "up",
                Self::Down => "down",
            }
        )
    }
}
