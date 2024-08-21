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
    let  (ref metta_output_stderr,ref metta_output_str) = metta_output;
    let metta_augmented_output = format!("{}{}",metta_output_stderr,metta_output_str);

    let _ = tools::logger::stop_timer(start_time, &metta_augmented_output);

    if let Some(command) = args.commands {
        match command {
            Commands::Format(command) => format(metta_output, command),
        }
    } else {
        println!("{}", metta_augmented_output);
    }
    Ok(())
}
