use rusqlite::OptionalExtension;

use crate::{
    database::{
        conversions::{Countable, Storable},
        tables::{gates::Gate, tiles::Tile},
    },
    parser::edit::CommandExtendProps,
    save_data::SaveData,
};

use super::{Handler, InnerHandlerResponse};

impl Handler {
    pub(crate) fn handle_initialize(
        &mut self,
        save_data: SaveData,
    ) -> rusqlite::Result<InnerHandlerResponse> {
        let count = Tile::count(&self.database)?;

        if count > 0 {
            return Ok(InnerHandlerResponse {
                message: "Cannot initialize a non-empty table.".into(),
                save_data,
            });
        }

        Tile {
            name: "starting tile".into(),
            summary: "starting summary".into(),
            body: "this is the first tile".into(),
        }
        .store(&self.database)?;

        Ok(InnerHandlerResponse {
            message: "Initialized grid with a starting tile.".into(),
            save_data: SaveData::default(),
        })
    }

    pub(crate) fn handle_extend(
        &mut self,
        save_data: SaveData,
        props: CommandExtendProps,
    ) -> Result<InnerHandlerResponse, rusqlite::Error> {
        let SaveData { current_tile, .. } = save_data;
        let CommandExtendProps { name, direction } = props;

        let gate = self
            .database
            .query_row(
                "SELECT id FROM gates WHERE source_id = ?1 AND direction = ?2",
                (&current_tile, &direction),
                |_| Ok(()),
            )
            .optional()?;

        if gate.is_some() {
            return Ok(InnerHandlerResponse {
                message: "There is already a tile in that direction.".into(),
                save_data,
            });
        }

        // TODO: Validate new tile coordinate-wise (check for spatial overlap).

        let tx = self.database.transaction()?;

        let new_tile_id = Tile {
            name: name.clone(),
            summary: format!("{name} summary"),
            body: format!("{name} body"),
        }
        .store(&tx)
        .unwrap();

        Gate {
            name: format!("{direction} gate"),
            summary: format!("{direction} gate summary"),
            body: format!("{direction} gate body"),
            direction: direction.clone(),
            source_id: current_tile,
            destination_id: new_tile_id,
        }
        .store(&tx)?;

        let opposite_direction = !direction.clone();

        Gate {
            name: format!("{opposite_direction} gate"),
            summary: format!("{opposite_direction} gate summary"),
            body: format!("{opposite_direction} gate body"),
            direction: opposite_direction.clone(),
            source_id: new_tile_id,
            destination_id: current_tile,
        }
        .store(&tx)?;

        tx.commit()?;

        Ok(InnerHandlerResponse {
            message: format!("Extended in the direction {direction}"),
            save_data,
        })
    }
}
