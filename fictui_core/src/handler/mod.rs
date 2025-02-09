use std::path::PathBuf;

use crate::{database::Database, parser::Command, save_data::SaveData, types::BoxedError};

pub mod edit;
pub mod play;

pub struct Handler {
    database: Database,
}

pub struct HandlerResponse {
    pub message: String,
    pub save_data: SaveData,
}

impl Handler {
    pub fn build(file: PathBuf) -> Result<Self, BoxedError> {
        let database = Database::establish(file)?;
        Ok(Self { database })
    }

    pub fn handle(&mut self, command: Command, save_data: SaveData) -> HandlerResponse {
        match command {
            Command::Unknown => self.handle_unknown(save_data),
            Command::Quit => self.handle_quit(save_data),
            Command::Look(props) => self.handle_look(save_data, props),
            Command::Go(props) => self.handle_go(save_data, props),
            Command::Rename(props) => self.handle_rename(save_data, props),
            Command::Initialize => self.handle_initialize(save_data),
            Command::Extend(props) => self.handle_extend(save_data, props),
        }
        .unwrap()
    }
}
