use rusqlite::OptionalExtension;

use crate::{parser::edit::CommandExtendProps, save_data::SaveData};

use super::{Handler, HandlerResponse};

impl Handler {
    pub(crate) fn handle_initialize(
        &mut self,
        save_data: SaveData,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        let count: i32 = self
            .database
            .query_row("SELECT COUNT(*) FROM tiles", [], |row| row.get(0))?;

        if count > 0 {
            return Ok(HandlerResponse {
                message: "Cannot initialize a non-empty table.".into(),
                save_data,
            });
        }

        let tx = self.database.transaction()?;

        tx.execute(
            "INSERT INTO tiles (name, summary, body) VALUES (?1, ?2, ?3)",
            (
                "starting tile",
                "starting summary",
                "this is the first tile",
            ),
        )?;

        tx.execute(
            "INSERT INTO tiles (name, summary, body) VALUES (?1, ?2, ?3)",
            (
                "starting tile",
                "starting summary",
                "this is the first tile",
            ),
        )?;

        tx.commit()?;

        Ok(HandlerResponse {
            message: "Initialized grid with a starting tile.".into(),
            save_data: SaveData::default(),
        })
    }

    pub(crate) fn handle_extend(
        &mut self,
        save_data: SaveData,
        props: CommandExtendProps,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        let gate = self
            .database
            .query_row(
                "SELECT id FROM gates WHERE source_id = ?1 AND direction = ?2",
                (&save_data.current_tile, &props.direction),
                |_| Ok(()),
            )
            .optional()?;

        if gate.is_some() {
            return Ok(HandlerResponse {
                message: "There is already a tile in that direction.".into(),
                save_data,
            });
        }

        let tile_id: Option<i32> = self
            .database
            .query_row(
                "SELECT id FROM tiles WHERE name = ?1",
                (&props.name,),
                |row| row.get(0),
            )
            .optional()?;

        // TODO: probably a bug below here.

        if tile_id.is_none() {
            self.database.execute(
                "INSERT INTO tiles (name, summary, body) VALUES (?1, ?2, ?3)",
                (&props.name, "new summary", "new body"),
            )?;
        }

        let tile_id = tile_id.unwrap_or(self.database.last_insert_rowid().try_into().unwrap());

        self.database.execute(
            "INSERT INTO tile_instances (tile_id) VALUES (?1)",
            (tile_id,),
        )?;

        // TODO: create all possible gates.

        Ok(HandlerResponse {
            message: format!("Extended in the direction {}", props.direction),
            save_data,
        })
    }
}
