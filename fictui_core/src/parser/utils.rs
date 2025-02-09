use nom::{
    character::complete::{alpha1, space0},
    error::ErrorKind,
    sequence::terminated,
    IResult, Parser,
};

use crate::database::types::direction::Direction;

pub(super) fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, next_word) = parse_next_word(input)?;

    let direction = match next_word {
        "n" | "north" => Direction::North,
        "e" | "east" => Direction::East,
        "s" | "south" => Direction::South,
        "w" | "west" => Direction::West,
        "u" | "up" => Direction::Up,
        "d" | "down" => Direction::Down,
        _ => {
            return Err(nom::Err::Error(nom::error::Error {
                input,
                code: ErrorKind::Alpha,
            }))
        }
    };

    Ok((input, direction))
}

pub(super) fn parse_next_word(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, space0).parse(input)
}
