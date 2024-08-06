use crate::runners;
use std::{env, fs};

pub fn format(file: String) {
    // cehck if the file exists
    if !fs::metadata(&file).is_ok() {
        eprintln!("File not found: {}", file);
        std::process::exit(1);
    }
    let formatter_path = format!(
        "{}/metta-bin/tools/formatter/tree-formater.py",
        env::var("HOME").unwrap()
    );

    let metta_output = runners::metta::run(file);

    //check if there are tree in metta output if there is tree format them
    for line in metta_output.lines() {
        if line.starts_with("[(TreeNode") {
            runners::python::run(&formatter_path, &line.to_string());
        } else {
            println!("{}", line);
        }
    }
}
