use nom::{
    character::complete::{alpha1, space0},
    sequence::terminated,
    IResult,
};

pub enum Command {
    Unknown,
    Quit,
    Look,
    Go,
    Rename(CommandRenameProps),
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
        let (input, first_word) = terminated(alpha1, space0)(input)?;

        let (input, command) = match first_word {
            "q" | "quit" => (input, Self::Quit),
            "l" | "look" => (input, Self::Look),
            "g" | "go" => (input, Self::Go),
            "r" | "rename" => Self::parse_command_rename(input)?,
            _ => (input, Self::Unknown),
        };

        Ok((input, command))
    }

    fn parse_command_rename(input: &str) -> IResult<&str, Self> {
        let (input, new_name) = terminated(alpha1, space0)(input)?;

        let new_name = new_name.to_string();

        Ok((input, Command::Rename(CommandRenameProps { new_name })))
    }
}
