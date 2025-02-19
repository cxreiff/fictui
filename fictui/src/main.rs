use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<Subcommands>,
}

#[derive(Subcommand)]
#[command(disable_help_subcommand(true))]
enum Subcommands {
    Play,
    Create,
    Host,
    Core(CoreArgs),
}

#[derive(Args)]
struct CoreArgs {
    #[command(subcommand)]
    subcommand: CoreSubcommands,
}

#[derive(Subcommand)]
enum CoreSubcommands {
    SelectTiles,
    SelectGates,
}

fn main() {
    let Cli { subcommand } = Cli::parse();

    let Some(subcommand) = subcommand else {
        fictui_play::run();
        return;
    };

    match subcommand {
        Subcommands::Play => fictui_play::run(),
        Subcommands::Create => todo!(),
        Subcommands::Host => todo!(),
        Subcommands::Core(core_args) => match core_args.subcommand {
            CoreSubcommands::SelectTiles => todo!(),
            CoreSubcommands::SelectGates => todo!(),
        },
    }
}
