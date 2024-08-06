use clap::{Parser, Subcommand};
use formatters::commands::{format, FormatterCommands};
use std::io;
mod formatters;
mod runners;

fn main() -> io::Result<()> {
    #[derive(Parser)]
    #[command(name = "metta-run")]
    #[command(about = "A CLI tool for Metta", long_about = None)]
    struct Args {
        #[clap(subcommand)]
        commands: Commands,
    }

    #[derive(Subcommand, Debug, Clone)]
    enum Commands {
        #[command(flatten)]
        Format(FormatterCommands),
    }

    let args = Args::parse();
    match args.commands {
        Commands::Format(command) => format(command),
    }

    Ok(())
}
