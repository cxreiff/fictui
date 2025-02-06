use std::path::PathBuf;

use save_data::SaveData;

use crate::{
    database::Database,
    parser::{Command, CommandLookProps},
    types::BoxedError,
};

pub mod save_data;

pub struct Grid {
    database: Database,
}

pub struct GridResponse {
    pub message: String,
    pub save_data: SaveData,
}

impl Grid {
    pub fn build(file: PathBuf) -> Result<Self, BoxedError> {
        let database = Database::establish(file)?;
        Ok(Self { database })
    }

    pub fn handle(&mut self, command: Command, save_data: SaveData) -> GridResponse {
        match command {
            Command::Unknown => self.handle_unknown(save_data),
            Command::Quit => self.handle_quit(save_data),
            Command::Look(props) => self.handle_look(save_data, props),
            Command::Go(_props) => todo!(),
            Command::Rename(_props) => todo!(),
        }
    }

    fn handle_unknown(&self, save_data: SaveData) -> GridResponse {
        GridResponse {
            message: format!("I did not understand that, {}...", save_data.name),
            save_data,
        }
    }

    fn handle_quit(&self, save_data: SaveData) -> GridResponse {
        GridResponse {
            message: "Quitting...".into(),
            save_data,
        }
    }

    fn handle_look(&mut self, save_data: SaveData, props: CommandLookProps) -> GridResponse {
        if let Some(direction) = props.direction {
            let gate = self
                .database
                .select_gate_by_source_and_direction(save_data.current_tile, direction);

            let message = gate.map_or("There is no passage in that direction.".into(), |gate| {
                gate.body
            });

            GridResponse { message, save_data }
        } else {
            let tile = self
                .database
                .select_tile(save_data.current_tile)
                .expect("invalid save data");

            GridResponse {
                message: tile.body,
                save_data,
            }
        }
    }
}
