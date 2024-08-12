use crate::runners;
use std::env;

pub fn format(metta_output: String) {
    let formatter_path = format!(
        "{}/metta-bin/tools/formatter/constraint_tree_formatter.py",
        env::var("HOME").unwrap()
    );

    let binary_tree_checker_path = format!(
        "{}/metta-bin/tools/utils/binary_tree_checker.py",
        env::var("HOME").unwrap()
    );

    //check if there are tree in metta output if there is tree format them
    for line in metta_output.lines() {
        if line.starts_with("[(TreeNode") {
            let is_binary_tree = runners::python::run(
                &binary_tree_checker_path,
                &line.to_string().replace("[", "").replace("]", ""),
            );
            if is_binary_tree == "False\n" {
                let formatted_tree = runners::python::run(&formatter_path, &line.to_string());
                println!("{}", formatted_tree);
                continue;
            }
            println!("{}", line);
        } else {
            println!("{}", line);
        }
    }
}
