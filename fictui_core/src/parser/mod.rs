use edit::{parse_command_extend, parse_command_initialize, CommandExtendProps};
use nom::IResult;
use play::{
    parse_command_go, parse_command_look, parse_command_rename, CommandGoProps, CommandLookProps,
    CommandRenameProps,
};
use utils::parse_next_word;

pub mod edit;
pub mod play;
pub mod utils;

pub enum Command {
    Unknown,
    Quit,
    Look(CommandLookProps),
    Go(CommandGoProps),
    Rename(CommandRenameProps),

    Initialize,
    Extend(CommandExtendProps),
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
        "i" | "init" => parse_command_initialize(input)?,
        "e" | "extend" => parse_command_extend(input)?,
        _ => (input, Command::Unknown),
    };

    Ok((input, command))
}
