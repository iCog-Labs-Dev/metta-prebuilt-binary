use clap::Subcommand;

use super::{binary_tree_formatter, constraint_tree_formatter, output_formatter};

#[derive(Subcommand, Debug, Clone)]
pub enum FormatterCommands {
    #[command(name = "fbt", about = "Format binary tree")]
    Fbt,

    #[command(name = "fct", about = "Format constraint tree")]
    Fct,

    #[command(name = "fgt", about = "Format constraint tree guardset")]
    Fgt,
}

pub fn format(metta_output: (String, String), command: FormatterCommands) {
    let (ref metta_output_stderr, ref metta_output_str) = metta_output;
    let metta_augmented = format!("{}{}", metta_output_stderr, metta_output_str);
    match command {
        FormatterCommands::Fbt => binary_tree_formatter::format(metta_output),
        FormatterCommands::Fct => constraint_tree_formatter::format(metta_output),
        FormatterCommands::Fgt => guardset_tree_formatter::format(metta_output),
    }
}
