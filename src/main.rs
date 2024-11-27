use bevy::prelude::*;
use clap::{Parser, Subcommand};

use fictui::play;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(disable_help_subcommand(true))]
enum Commands {
    Play,
    Create,
    Host,
}

fn main() {
    let Args { command } = Args::parse();

    match command {
        Some(Commands::Play) | None => {
            App::new().add_plugins(play::app_plugin).run();
        }
        Some(Commands::Create) => {}
        Some(Commands::Host) => {}
    }
}
