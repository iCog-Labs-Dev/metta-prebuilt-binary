use clap::Subcommand;

use super::binary_tree_formatter;

#[derive(Subcommand, Debug, Clone)]
pub enum FormatterCommands {
    #[command(name = "fbt", about = "Format binary tree")]
    Fbt { file: String },

    #[command(name = "fct", about = "Format constraint tree")]
    Fct,
}

pub fn format(command: FormatterCommands) {
    match command {
        FormatterCommands::Fbt { file } => binary_tree_formatter::format(file),
        FormatterCommands::Fct => println!("under construction"),
    }
}
