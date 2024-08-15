use clap::{Parser, Subcommand};
use formatters::commands::{format, FormatterCommands};
use std::io;
mod formatters;
mod runners;
mod tools;

fn main() -> io::Result<()> {
    #[derive(Parser)]
    #[command(name = "metta-run")]
    #[command(about = "A CLI tool for Metta", long_about = None)]
    struct Args {
        file: String,
        #[clap(subcommand)]
        commands: Option<Commands>,
    }

    #[derive(Subcommand)]
    enum Commands {
        #[command(flatten)]
        Format(FormatterCommands),
    }

    let args = Args::parse();
    let file = args.file;

    let start_time = tools::logger::start_timer();
    let metta_output = runners::metta::run(file);
    tools::logger::stop_timer(start_time, &metta_output)?;

    if let Some(command) = args.commands {
        match command {
            Commands::Format(command) => format(metta_output, command),
        }
    } else {
        println!("{}", metta_output);
    }
    Ok(())
}
