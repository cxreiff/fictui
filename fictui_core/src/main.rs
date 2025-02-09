use std::path::PathBuf;

use clap::{Parser, Subcommand};
use fictui_core::{database::Database, types::BoxedError};

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
    InsertGate,
    ListGates,
}

fn main() -> Result<(), BoxedError> {
    let Args { command, file } = Args::parse();

    let mut _db = Database::establish(file)?;

    match command {
        Commands::InsertTile => {
            //
        }
        Commands::ListTiles => todo!(),
        Commands::InsertTileInstance => todo!(),
        Commands::ListTileInstances => todo!(),
        Commands::InsertGate => todo!(),
        Commands::ListGates => todo!(),
    };

    Ok(())
}
