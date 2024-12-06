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
        match command_string.chars().next() {
            Some('q') => Command::Quit,
            Some('l') => Command::Look,
            Some('g') => Command::Go,
            Some('r') => Self::parse_command_rename(command_string),
            _ => Command::Unknown,
        }
    }

    fn parse_command_rename(command_string: &str) -> Self {
        let Some(new_name) = command_string.split(' ').nth(1) else {
            return Command::Unknown;
        };

        let new_name = new_name.to_string();

        Command::Rename(CommandRenameProps { new_name })
    }
}
