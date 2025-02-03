use nom::{
    character::complete::{alpha1, space0},
    error::ErrorKind,
    sequence::terminated,
    Err::Error,
    IResult,
};

use super::types::Direction;

pub enum Command {
    Unknown,
    Quit,
    Look(CommandLookProps),
    Go(CommandGoProps),
    Rename(CommandRenameProps),
}

pub struct CommandLookProps {
    pub _direction: Option<Direction>,
}

pub struct CommandGoProps {
    pub direction: Direction,
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

    fn parse_direction(input: &str) -> IResult<&str, Direction> {
        let (input, next_word) = Self::parse_next_word(input)?;

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

        Ok((input, direction))
    }

    fn parse_next_word(input: &str) -> IResult<&str, &str> {
        terminated(alpha1, space0)(input)
    }
}
