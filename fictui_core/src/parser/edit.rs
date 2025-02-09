use nom::IResult;

use crate::database::types::direction::Direction;

use super::{
    utils::{parse_direction, parse_next_word},
    Command,
};

pub struct CommandExtendProps {
    pub direction: Direction,
    pub name: String,
}

pub(super) fn parse_command_initialize(input: &str) -> IResult<&str, Command> {
    Ok((input, Command::Initialize))
}

pub(super) fn parse_command_extend(input: &str) -> IResult<&str, Command> {
    let (input, direction) = parse_direction(input)?;

    let (_, name) = parse_next_word(input)?;
    let name = name.to_string();

    Ok(("", Command::Extend(CommandExtendProps { direction, name })))
}
