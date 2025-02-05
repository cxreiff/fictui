use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use fictui_core::database::{
    models::{tile_instances::NewTileInstance, tiles::NewTile},
    Database,
};

#[derive(Parser)]
struct Args {
    file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[command(disable_help_subcommand(true))]
enum Commands {
    InsertTile,
    ListTiles,
    InsertTileInstance,
    ListTileInstances,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let Args { command, file } = Args::parse();

    let mut db = Database::establish(file)?;

    match command {
        Commands::InsertTile => {
            db.insert_tile(NewTile {
                name: "test tile",
                summary: "test tile summary",
                body: "test tile body",
            });
        }
        Commands::ListTiles => {
            db.list_tiles().iter().for_each(|tile| println!("{tile:?}"));
        }
        Commands::InsertTileInstance => {
            db.insert_tile_instance(NewTileInstance { tile_id: &1 });
        }
        Commands::ListTileInstances => {
            db.list_tile_instances()
                .iter()
                .for_each(|tile_instance| println!("{tile_instance:?}"));
        }
    };

    Ok(())
}
