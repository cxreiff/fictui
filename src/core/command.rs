#[derive(Clone)]
pub struct SaveData {
    pub name: String,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            name: "Player".into(),
        }
    }
}

pub struct Response {
    pub command: String,
    pub message: String,
    pub new_save_data: SaveData,
}

enum Command {
    Unknown,
    Look,
    Go,
}

pub fn handle_command(command_string: &str, save_data: &SaveData) -> Response {
    let command = parse_command(command_string);

    match command {
        Command::Unknown => handle_command_unknown(command_string, save_data),
        Command::Look => handle_command_look(command_string, save_data),
        Command::Go => handle_command_go(command_string, save_data),
    }
}

fn parse_command(command_string: &str) -> Command {
    match command_string.chars().next() {
        Some('l') => Command::Look,
        Some('g') => Command::Go,
        _ => Command::Unknown,
    }
}

fn handle_command_unknown(command_string: &str, save_data: &SaveData) -> Response {
    Response {
        command: command_string.into(),
        message: format!("I did not understand that, {}.", save_data.name),
        new_save_data: save_data.clone(),
    }
}

fn handle_command_look(command_string: &str, save_data: &SaveData) -> Response {
    Response {
        command: command_string.into(),
        message: "You see your surroundings.".into(),
        new_save_data: save_data.clone(),
    }
}

fn handle_command_go(command_string: &str, save_data: &SaveData) -> Response {
    Response {
        command: command_string.into(),
        message: "You proceed.".into(),
        new_save_data: save_data.clone(),
    }
}
