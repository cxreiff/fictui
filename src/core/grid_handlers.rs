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
    const INVALID_GRID_DATA: &str = "The grid has invalid data.";
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
        let Some(current_tile_id) = self.data.tiles_map.get(&save_data.coords) else {
            return GridResponse {
                message: GridResponse::INVALID_SAVE_DATA.to_string(),
                new_save_data: save_data.clone(),
            };
        };

        let Some(current_tile_index) = self.maps.tiles.get(current_tile_id) else {
            return GridResponse {
                message: GridResponse::INVALID_GRID_DATA.to_string(),
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
        let (x, y, z) = save_data.coords;

        let next_coords = match props.direction {
            super::command::CommandDirection::North => (x, y + 1, z),
            super::command::CommandDirection::East => (x + 1, y, z),
            super::command::CommandDirection::South => (x, y - 1, z),
            super::command::CommandDirection::West => (x - 1, y, z),
            super::command::CommandDirection::Up => (x, y, z + 1),
            super::command::CommandDirection::Down => (x, y, z - 1),
        };

        let Some(next_tile_id) = self.data.tiles_map.get(&next_coords) else {
            return GridResponse {
                message: "There is no way to go in that direction.".into(),
                new_save_data: save_data.clone(),
            };
        };

        let Some(next_tile_index) = self.maps.tiles.get(next_tile_id) else {
            return GridResponse {
                message: GridResponse::INVALID_GRID_DATA.to_string(),
                new_save_data: save_data.clone(),
            };
        };

        let Some(next_tile) = self.data.tiles.get(*next_tile_index) else {
            unreachable!();
        };

        GridResponse {
            message: format!(
                "You proceed {}. {}.",
                props.direction,
                next_tile.summary.clone()
            ),
            new_save_data: SaveData {
                coords: next_coords,
                ..save_data.clone()
            },
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
