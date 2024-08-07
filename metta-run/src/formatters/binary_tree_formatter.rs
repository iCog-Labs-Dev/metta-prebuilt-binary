use crate::runners;
use std::env;

pub fn format(metta_output: String) {
    let formatter_path = format!(
        "{}/metta-bin/tools/formatter/binary_tree_formater.py",
        env::var("HOME").unwrap()
    );

    //check if there are tree in metta output if there is tree format them
    for line in metta_output.lines() {
        if line.starts_with("[(TreeNode") {
            runners::python::run(&formatter_path, &line.to_string());
        } else {
            println!("{}", line);
        }
    }
}
