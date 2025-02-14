use crate::{
    parser::play::{CommandGoProps, CommandLookProps, CommandRenameProps},
    save_data::SaveData,
};

use super::{Handler, HandlerResponse};

impl Handler {
    pub(super) fn handle_ping(
        &self,
        save_data: SaveData,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        let tile_extended = self.database.extended_tile_query(save_data.current_tile)?;

        Ok(HandlerResponse {
            message: "Pinged.".into(),
            save_data,
            aux_data: Some(tile_extended),
        })
    }

    pub(super) fn handle_unknown(
        &self,
        save_data: SaveData,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        Ok(HandlerResponse {
            message: format!("I did not understand that, {}...", save_data.name),
            save_data,
            aux_data: None,
        })
    }

    pub(super) fn handle_quit(
        &self,
        save_data: SaveData,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        Ok(HandlerResponse {
            message: "Quitting...".into(),
            save_data,
            aux_data: None,
        })
    }

    pub(super) fn handle_look(
        &mut self,
        save_data: SaveData,
        props: CommandLookProps,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        let tile_extended = self.database.extended_tile_query(save_data.current_tile)?;

        let message = if let Some(ref direction) = props.direction {
            tile_extended.neighbor_in_direction(direction).map_or(
                "There is no passage in that direction.".into(),
                |neighbor| neighbor.gate.summary.clone(),
            )
        } else {
            tile_extended.tile.body.clone()
        };

        Ok(HandlerResponse {
            message,
            save_data,
            aux_data: Some(tile_extended),
        })
    }

    pub(super) fn handle_go(
        &mut self,
        save_data: SaveData,
        props: CommandGoProps,
    ) -> Result<HandlerResponse, rusqlite::Error> {
        let tile_extended = self.database.extended_tile_query(save_data.current_tile)?;
        let neighbor = tile_extended.neighbor_in_direction(&props.direction);

        let Some(neighbor) = neighbor else {
            return Ok(HandlerResponse {
                message: "There is no passage in that direction".into(),
                save_data,
                aux_data: Some(tile_extended),
            });
        };

        let gate_body = &neighbor.gate.body;

        let new_tile_extended = self
            .database
            .extended_tile_query(neighbor.tile_instance.id)?;

        let tile_body = &new_tile_extended.tile.body;

        Ok(HandlerResponse {
            message: format!("{gate_body}\n\n{tile_body}"),
            save_data: SaveData {
                current_tile: new_tile_extended.tile_instance.id,
                ..save_data
            },
            aux_data: Some(new_tile_extended),
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
            aux_data: None,
        })
    }
}
