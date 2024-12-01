use super::command::Command;

impl Command {
    pub fn parse_command_rename(command_string: &str) -> Self {
        let Some(new_name) = command_string.split(' ').nth(1) else {
            return Command::Unknown;
        };

        let new_name = new_name.to_string();

        Command::Rename { new_name }
    }
}
