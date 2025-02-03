use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use fictui_core::database::{Database, Tile};
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
    CreateTile,
    ListTiles,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { command, file } = Args::parse();

    let db = Database::establish(file)?;

    match command {
        Commands::CreateTile => db.create_tile(Tile {
            id: 1,
            name: "test".into(),
        })?,
        Commands::ListTiles => db.list_tiles().iter().for_each(|tile| println!("{tile:?}")),
    };

    Ok(())
}
