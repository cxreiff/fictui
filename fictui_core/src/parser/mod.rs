use std::str::FromStr;

use nom::{
    character::complete::{alpha1, space0},
    error::ErrorKind,
    sequence::terminated,
    Err::Error,
    IResult, Parser,
};

use crate::database::models::gates::GateDirection;

pub enum Command {
    Unknown,
    Quit,
    Look(CommandLookProps),
    Go(CommandGoProps),
    Rename(CommandRenameProps),
}

pub struct CommandLookProps {
    pub _direction: Option<GateDirection>,
}

pub struct CommandGoProps {
    pub direction: GateDirection,
}

pub struct CommandRenameProps {
    pub new_name: String,
}

impl Command {
    pub fn new(input: &str) -> Self {
        input.parse().unwrap_or(Self::Unknown)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCommandError;

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_command(s)
            .map_err(|_| ParseCommandError)
            .map(|(_, command)| command)
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, next_word) = parse_next_word(input)?;

    let (input, command) = match next_word {
        "q" | "quit" => (input, Command::Quit),
        "l" | "look" => parse_command_look(input)?,
        "g" | "go" => parse_command_go(input)?,
        "r" | "rename" => parse_command_rename(input)?,
        _ => (input, Command::Unknown),
    };

    Ok((input, command))
}

fn parse_command_look(input: &str) -> IResult<&str, Command> {
    Ok((input, Command::Look(CommandLookProps { _direction: None })))
}

fn parse_command_go(input: &str) -> IResult<&str, Command> {
    let (input, next_word) = parse_next_word(input)?;

    let (_, direction) = parse_direction(next_word)?;

    Ok((input, Command::Go(CommandGoProps { direction })))
}

fn parse_command_rename(input: &str) -> IResult<&str, Command> {
    let (input, new_name) = parse_next_word(input)?;

    let new_name = new_name.to_string();

    Ok((input, Command::Rename(CommandRenameProps { new_name })))
}

fn parse_direction(input: &str) -> IResult<&str, GateDirection> {
    let (input, next_word) = parse_next_word(input)?;

    let direction = match next_word {
        "n" | "north" => GateDirection::North,
        "e" | "east" => GateDirection::East,
        "s" | "south" => GateDirection::South,
        "w" | "west" => GateDirection::West,
        "u" | "up" => GateDirection::Up,
        "d" | "down" => GateDirection::Down,
        _ => {
            return Err(Error(nom::error::Error {
                input,
                code: ErrorKind::Alpha,
            }))
        }
    };

    Ok((input, direction))
}

fn parse_next_word(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, space0).parse(input)
}
