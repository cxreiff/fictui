use rusqlite::OptionalExtension;

use crate::{
    database::{
        tables::{gates::Gate, tiles::Tile},
        TableRow,
    },
    parser::play::{CommandGoProps, CommandLookProps, CommandRenameProps},
    save_data::SaveData,
};

use super::{Handler, HandlerResponse};

impl Handler {
    pub(super) fn handle_unknown(
        &self,
        save_data: SaveData,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        Ok(HandlerResponse {
            message: format!("I did not understand that, {}...", save_data.name),
            save_data,
        })
    }

    pub(super) fn handle_quit(
        &self,
        save_data: SaveData,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        Ok(HandlerResponse {
            message: "Quitting...".into(),
            save_data,
        })
    }

    pub(super) fn handle_look(
        &mut self,
        save_data: SaveData,
        props: CommandLookProps,
    ) -> Result<HandlerResponse, rusqlite::Error> {
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
                .optional()?;

            let message = gate.map_or("There is no passage in that direction.".into(), |gate| {
                gate.summary
            });

            Ok(HandlerResponse { message, save_data })
        } else {
            let tile = self.database.query_row(
                &format!("SELECT {} FROM tiles WHERE id = ?1", Tile::columns_string()),
                (save_data.current_tile,),
                Tile::try_from_row,
            )?;

            Ok(HandlerResponse {
                message: tile.body,
                save_data,
            })
        }
    }

    pub(super) fn handle_go(
        &mut self,
        save_data: SaveData,
        props: CommandGoProps,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        let result: Option<(String, i32, String)> = self
            .database
            .query_row(
                "SELECT
                    g.body,
                    ti.id,
                    t.body
                FROM gates AS g
                    INNER JOIN tile_instances AS ti ON g.destination_id = ti.id
                    INNER JOIN tiles AS t ON ti.tile_id = t.id
                WHERE source_id = ?1 AND direction = ?2",
                (save_data.current_tile, props.direction),
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()?;

        let Some((gate_body, tile_instance_id, tile_body)) = result else {
            return Ok(HandlerResponse {
                message: "There is no passage in that direction".into(),
                save_data,
            });
        };

        Ok(HandlerResponse {
            message: format!("{gate_body}\n\n{tile_body}"),
            save_data: SaveData {
                current_tile: tile_instance_id,
                ..save_data
            },
        })
    }

    pub(super) fn handle_rename(
        &mut self,
        save_data: SaveData,
        props: CommandRenameProps,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        Ok(HandlerResponse {
            message: format!("You are now called {}", props.new_name),
            save_data: SaveData {
                name: props.new_name,
                ..save_data
            },
        })
    }
}
