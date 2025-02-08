use std::path::PathBuf;

use rusqlite::OptionalExtension;
use save_data::SaveData;

use crate::{
    database::{gates::Gate, tiles::Tile, Database, TableRow},
    parser::{Command, CommandGoProps, CommandLookProps, CommandRenameProps},
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
            Command::Go(props) => self.handle_go(save_data, props),
            Command::Rename(props) => self.handle_rename(save_data, props),
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
                .query_row(
                    &format!(
                        "SELECT {} FROM gates WHERE source_id = ?1 AND direction = ?2",
                        Gate::columns_string()
                    ),
                    (save_data.current_tile, direction),
                    Gate::try_from_row,
                )
                .optional()
                .unwrap();

            let message = gate.map_or("There is no passage in that direction.".into(), |gate| {
                gate.summary
            });

            GridResponse { message, save_data }
        } else {
            let tile = self
                .database
                .query_row(
                    &format!("SELECT {} FROM tiles WHERE id = ?1", Tile::columns_string()),
                    (save_data.current_tile,),
                    Tile::try_from_row,
                )
                .unwrap();

            GridResponse {
                message: tile.body,
                save_data,
            }
        }
    }

    fn handle_go(&mut self, _save_data: SaveData, _props: CommandGoProps) -> GridResponse {
        unimplemented!()
        //     let gate = self.database.select_gate_by_source_and_direction(save_data.current_tile, props.direction);

        //     gates::gates::table.select(Gate::as_select()).

        //     if let Some(gate) = gate {
        //         GridResponse {
        //             message: format!("{gate.body}\n\n{}"),
        //             save_data: SaveData {
        //                 current_tile: gate.destination_tile_id,
        //                 ..save_data,
        //             },
        //         }
        //     } else {
        //         GridResponse {
        //             message: "There is no passage in that direction".into(),
        //             save_data,
        //         }
        //     }
    }

    fn handle_rename(&mut self, save_data: SaveData, props: CommandRenameProps) -> GridResponse {
        GridResponse {
            message: format!("You are now called {}", props.new_name),
            save_data: SaveData {
                name: props.new_name,
                ..save_data
            },
        }
    }
}
