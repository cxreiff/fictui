use nom::{
    character::complete::{alpha1, space0},
    error::ErrorKind,
    sequence::terminated,
    Err::Error,
    IResult, Parser,
};

use crate::database::direction::Direction;

pub enum Command {
    Unknown,
    Quit,
    Look(CommandLookProps),
    Go(CommandGoProps),
    Rename(CommandRenameProps),
}

pub struct CommandLookProps {
    pub direction: Option<Direction>,
}

pub struct CommandGoProps {
    pub direction: Direction,
}

pub struct CommandRenameProps {
    pub new_name: String,
}

impl Command {
    pub fn parse(input: &str) -> Self {
        parse_command(input)
            .map(|(_, command)| command)
            .unwrap_or(Self::Unknown)
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
    let direction = parse_direction(input).map(|(_, d)| d).ok();

    Ok(("", Command::Look(CommandLookProps { direction })))
}

fn parse_command_go(input: &str) -> IResult<&str, Command> {
    let (_, next_word) = parse_next_word(input)?;

    let (_, direction) = parse_direction(next_word)?;

    Ok(("", Command::Go(CommandGoProps { direction })))
}

fn parse_command_rename(input: &str) -> IResult<&str, Command> {
    let (input, new_name) = parse_next_word(input)?;

    let new_name = new_name.to_string();

    Ok((input, Command::Rename(CommandRenameProps { new_name })))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (_, next_word) = parse_next_word(input)?;

    let direction = match next_word {
        "n" | "north" => Direction::North,
        "e" | "east" => Direction::East,
        "s" | "south" => Direction::South,
        "w" | "west" => Direction::West,
        "u" | "up" => Direction::Up,
        "d" | "down" => Direction::Down,
        _ => {
            return Err(Error(nom::error::Error {
                input,
                code: ErrorKind::Alpha,
            }))
        }
    };

    Ok(("", direction))
}

fn parse_next_word(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, space0).parse(input)
}
