use crate::core::saves::SaveData;

use super::command::{Command, Response};

impl Command {
    pub fn handle_command_unknown(self, save_data: &SaveData) -> Response {
        Response {
            message: format!("I did not understand that, {}.", save_data.name),
            new_save_data: save_data.clone(),
        }
    }

    pub fn handle_command_quit(self, save_data: &SaveData) -> Response {
        Response {
            message: "Quitting...".into(),
            new_save_data: save_data.clone(),
        }
    }

    pub fn handle_command_look(self, save_data: &SaveData) -> Response {
        Response {
            message: "You see your surroundings.".into(),
            new_save_data: save_data.clone(),
        }
    }

    pub fn handle_command_go(self, save_data: &SaveData) -> Response {
        Response {
            message: "You proceed.".into(),
            new_save_data: save_data.clone(),
        }
    }

    pub fn handle_command_rename(self, save_data: &SaveData) -> Response {
        let Command::Rename { new_name } = self else {
            unreachable!()
        };

        Response {
            message: format!("Your new name is {}", new_name),
            new_save_data: SaveData {
                name: new_name,
                ..*save_data
            },
        }
    }
}
