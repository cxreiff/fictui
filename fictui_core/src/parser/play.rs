use nom::IResult;

use crate::database::fields::direction::Direction;

use super::{
    utils::{parse_direction, parse_next_word},
    Command,
};

pub struct CommandLookProps {
    pub direction: Option<Direction>,
}

pub struct CommandGoProps {
    pub direction: Direction,
}

pub struct CommandRenameProps {
    pub new_name: String,
}

pub(super) fn parse_command_look(input: &str) -> IResult<&str, Command> {
    let direction = parse_direction(input).map(|(_, d)| d).ok();

    Ok(("", Command::Look(CommandLookProps { direction })))
}

pub(super) fn parse_command_go(input: &str) -> IResult<&str, Command> {
    let (_, direction) = parse_direction(input)?;

    Ok(("", Command::Go(CommandGoProps { direction })))
}

pub(super) fn parse_command_rename(input: &str) -> IResult<&str, Command> {
    let (input, new_name) = parse_next_word(input)?;

    let new_name = new_name.to_string();

    Ok((input, Command::Rename(CommandRenameProps { new_name })))
}
