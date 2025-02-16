use nom::IResult;

use crate::database::fields::direction::Direction;

use super::{utils::parse_direction, Command};

pub struct CommandExtendProps {
    pub direction: Direction,
    pub name: String,
}

pub(super) fn parse_command_initialize(input: &str) -> IResult<&str, Command> {
    Ok((input, Command::Initialize))
}

pub(super) fn parse_command_extend(input: &str) -> IResult<&str, Command> {
    let (input, direction) = parse_direction(input)?;

    let name = input.to_string();

    Ok(("", Command::Extend(CommandExtendProps { direction, name })))
}
