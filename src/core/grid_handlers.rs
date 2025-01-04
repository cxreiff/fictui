use crate::core::{command::Command, save_data::SaveData};

use super::{
    command::{CommandGoProps, CommandLookProps, CommandRenameProps},
    grid::Grid,
};

pub struct GridResponse {
    pub message: String,
    pub new_save_data: SaveData,
}

impl GridResponse {
    const INVALID_SAVE_DATA: &str = "Your save data is corrupted.";
}

impl Grid {
    pub fn handle(&self, command: &Command, save_data: &SaveData) -> GridResponse {
        match command {
            Command::Unknown => self.handle_command_unknown(save_data),
            Command::Quit => self.handle_command_quit(save_data),
            Command::Look(props) => self.handle_command_look(save_data, props),
            Command::Go(props) => self.handle_command_go(save_data, props),
            Command::Rename(props) => self.handle_command_rename(save_data, props),
        }
    }

    fn handle_command_unknown(&self, save_data: &SaveData) -> GridResponse {
        GridResponse {
            message: format!("I did not understand that, {}.", save_data.name),
            new_save_data: save_data.clone(),
        }
    }

    fn handle_command_quit(&self, save_data: &SaveData) -> GridResponse {
        GridResponse {
            message: "Quitting...".into(),
            new_save_data: save_data.clone(),
        }
    }

    fn handle_command_look(&self, save_data: &SaveData, _props: &CommandLookProps) -> GridResponse {
        let Some(current_tile_index) = self.maps.tiles.get(&save_data.tile) else {
            return GridResponse {
                message: GridResponse::INVALID_SAVE_DATA.to_string(),
                new_save_data: save_data.clone(),
            };
        };

        let Some(current_tile) = self.data.tiles.get(*current_tile_index) else {
            unreachable!();
        };

        GridResponse {
            message: current_tile.summary.clone(),
            new_save_data: save_data.clone(),
        }
    }

    fn handle_command_go(&self, save_data: &SaveData, props: &CommandGoProps) -> GridResponse {
        GridResponse {
            message: format!("You proceed {}.", props.direction),
            new_save_data: save_data.clone(),
        }
    }

    fn handle_command_rename(
        &self,
        save_data: &SaveData,
        props: &CommandRenameProps,
    ) -> GridResponse {
        GridResponse {
            message: format!("Your new name is {}", props.new_name),
            new_save_data: SaveData {
                name: props.new_name.clone(),
                ..save_data.clone()
            },
        }
    }
}
