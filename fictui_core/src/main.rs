use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use fictui_core::database::{Database, TableRow, Tile};
use rusqlite::Result;

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
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { command, file } = Args::parse();

    let db = Database::establish(file)?;

    match command {
        Commands::InsertTile => {
            Tile::insert(
                &db,
                Tile {
                    name: "test".into(),
                    body: "test body".into(),
                },
            )?;
        }
        Commands::ListTiles => Tile::list(&db).iter().for_each(|tile| println!("{tile:?}")),
    };

    Ok(())
}
