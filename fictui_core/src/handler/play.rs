use crate::{
    aux_data::TileWithNeighbors,
    database::{conversions::Retrievable, tables::tiles::Tile},
    parser::play::{CommandGoProps, CommandLookProps, CommandRenameProps},
    save_data::SaveData,
};

use super::{Handler, InnerHandlerResponse};

impl Handler {
    pub(super) fn handle_unknown(
        &self,
        save_data: SaveData,
    ) -> Result<InnerHandlerResponse, rusqlite::Error> {
        Ok(InnerHandlerResponse {
            message: format!("I did not understand that, {}...", save_data.name),
            save_data,
        })
    }

    pub(super) fn handle_quit(
        &self,
        save_data: SaveData,
    ) -> Result<InnerHandlerResponse, rusqlite::Error> {
        Ok(InnerHandlerResponse {
            message: "Quitting...".into(),
            save_data,
        })
    }

    pub(super) fn handle_look(
        &mut self,
        save_data: SaveData,
        props: CommandLookProps,
    ) -> Result<InnerHandlerResponse, rusqlite::Error> {
        let message = if let Some(ref direction) = props.direction {
            let tile_with_neighbors =
                TileWithNeighbors::retrieve(&self.database, save_data.current_tile)?;

            tile_with_neighbors.neighbor(direction).map_or(
                "There is no passage in that direction.".into(),
                |neighbor| neighbor.gate.summary.clone(),
            )
        } else {
            let tile = Tile::retrieve(&self.database, save_data.current_tile)?;
            tile.body.clone()
        };

        Ok(InnerHandlerResponse { message, save_data })
    }

    pub(super) fn handle_go(
        &mut self,
        save_data: SaveData,
        props: CommandGoProps,
    ) -> Result<InnerHandlerResponse, rusqlite::Error> {
        let tile_with_neighbors =
            TileWithNeighbors::retrieve_recursive(&self.database, save_data.current_tile, 2)?;

        let Some(neighbor) = tile_with_neighbors.neighbor(&props.direction) else {
            return Ok(InnerHandlerResponse {
                message: "There is no passage in that direction".into(),
                save_data,
            });
        };

        let gate_body = &neighbor.gate.body;

        let tile_body = neighbor.tile.as_ref().unwrap().tile.body.clone();

        let message = format!("{gate_body}\n\n{tile_body}");

        let save_data = SaveData {
            current_tile: neighbor.gate.destination_id,
            ..save_data
        };

        Ok(InnerHandlerResponse { message, save_data })
    }

    pub(super) fn handle_rename(
        &mut self,
        save_data: SaveData,
        props: CommandRenameProps,
    ) -> Result<InnerHandlerResponse, rusqlite::Error> {
        Ok(InnerHandlerResponse {
            message: format!("You are now called {}", props.new_name),
            save_data: SaveData {
                name: props.new_name,
                ..save_data
            },
        })
    }
}
