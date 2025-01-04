use std::fmt::Display;

use nom::{
    character::complete::{alpha1, space0},
    error::ErrorKind,
    sequence::terminated,
    Err::Error,
    IResult,
};

pub enum Command {
    Unknown,
    Quit,
    Look(CommandLookProps),
    Go(CommandGoProps),
    Rename(CommandRenameProps),
}

pub struct CommandLookProps {
    pub _direction: Option<CommandDirection>,
}

pub struct CommandGoProps {
    pub direction: CommandDirection,
}

pub struct CommandRenameProps {
    pub new_name: String,
}

impl Command {
    pub fn parse(command_string: &str) -> Self {
        let Ok((_, command)) = Self::parse_command(command_string) else {
            return Command::Unknown;
        };

        command
    }

    fn parse_command(input: &str) -> IResult<&str, Self> {
        let (input, next_word) = terminated(alpha1, space0)(input)?;

        let (input, command) = match next_word {
            "q" | "quit" => (input, Self::Quit),
            "l" | "look" => Self::parse_command_look(input)?,
            "g" | "go" => Self::parse_command_go(input)?,
            "r" | "rename" => Self::parse_command_rename(input)?,
            _ => (input, Self::Unknown),
        };

        Ok((input, command))
    }

    fn parse_command_look(input: &str) -> IResult<&str, Self> {
        Ok((input, Self::Look(CommandLookProps { _direction: None })))
    }

    fn parse_command_go(input: &str) -> IResult<&str, Self> {
        let (input, next_word) = Self::parse_next_word(input)?;

        let (_, direction) = Self::parse_direction(next_word)?;

        Ok((input, Self::Go(CommandGoProps { direction })))
    }

    fn parse_command_rename(input: &str) -> IResult<&str, Self> {
        let (input, new_name) = terminated(alpha1, space0)(input)?;

        let new_name = new_name.to_string();

        Ok((input, Command::Rename(CommandRenameProps { new_name })))
    }

    fn parse_direction(input: &str) -> IResult<&str, CommandDirection> {
        let (input, next_word) = Self::parse_next_word(input)?;

        let direction = match next_word {
            "n" | "north" => CommandDirection::North,
            "e" | "east" => CommandDirection::East,
            "s" | "south" => CommandDirection::South,
            "w" | "west" => CommandDirection::West,
            "u" | "up" => CommandDirection::Up,
            "d" | "down" => CommandDirection::Down,
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
        terminated(alpha1, space0)(input)
    }
}

pub enum CommandDirection {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl Display for CommandDirection {
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
