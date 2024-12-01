use crate::core::saves::SaveData;

pub enum Command {
    Unknown,
    Quit,
    Look,
    Go,
    Rename { new_name: String },
}

pub struct Response {
    pub message: String,
    pub new_save_data: SaveData,
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

    pub fn handle(self, save_data: &SaveData) -> Response {
        match self {
            Command::Unknown => self.handle_command_unknown(save_data),
            Command::Quit => self.handle_command_quit(save_data),
            Command::Look => self.handle_command_look(save_data),
            Command::Go => self.handle_command_go(save_data),
            Command::Rename { .. } => self.handle_command_rename(save_data),
        }
    }
}
