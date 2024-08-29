// tests/integration_test.rs

#[test]
fn test_metta_run() {
    use std::process::Command;

    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/resources/metta_run.metta")
        .output()
        .expect("failed to execute process");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "[(Hello World!)]\n"
    );
}

#[test]
fn test_binary_tree_formatter() {
    use std::process::Command;

    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/resources/binary_tree.metta")
        .arg("fbt")
        .output()
        .expect("failed to execute process");

    let expected_output = "TreeNode ROOT\n    ├──Left  ────  (NilNode)\n    └──Right ────  (TreeNode (Value Nil True OR)\n        ├──Left  ────────  (TreeNode (Value a False LITERAL)\n        └──Right ────────  (TreeNode (Value Nil True AND)\n            ├──Left  ────────────  (TreeNode (Value b False LITERAL)\n            └──Right ────────────  (TreeNode (Value c False LITERAL)\n";

    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);
}

#[test]
fn test_constraint_tree_formatter() {
    use std::process::Command;

    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/resources/constraint_tree.metta")
        .arg("fct")
        .output()
        .expect("failed to execute process");

    let expected_output = "[(TreeNode (Value Nil False OR)\n└──Children's\n    ├─(TreeNode (Value A False LITERAL)\n    ├─(TreeNode (Value B True LITERAL)\n    ├─(TreeNode (Value C False LITERAL)\n    ├─(TreeNode (Value D False LITERAL)\n    ├─(TreeNode (Value E True LITERAL)\n";

    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);
}

#[test]
fn test_nested_constraint_tree_formatter() {
    use std::process::Command;

    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/resources/nested_constraint_tree.metta")
        .arg("fct")
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    let expected_output = "[(TreeNode (Value Nil False OR)\n└──Children's\n    ├─(TreeNode (Value A False LITERAL)\n    └──Children's\n        ├─(TreeNode (Value F False LITERAL)\n    ├─(TreeNode (Value B True LITERAL)\n    ├─(TreeNode (Value C False LITERAL)\n    ├─(TreeNode (Value D False LITERAL)\n    ├─(TreeNode (Value E True LITERAL)\n";

    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);
}

#[test]
fn test_guard_set_formatter() {
    use std::process::Command;

    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/resources/guard_set.metta")
        .arg("fgt")
        .output()
        .expect("failed to execute process");

        let expected_output = "[(TreeNode (Value Nil False OR)\n└──GuardSets's\n    ├─(TreeNode (Value B True LITERAL)\n    ├─(TreeNode (Value D True LITERAL)\n    ├─(TreeNode (Value E False LITERAL)\n";
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);
}
/// test logger functions
#[test]
fn test_logger() {
    use std::fs::File;
    use std::io::Read;
    use core::panic;
    use std::{io::BufRead, process::Command};
    use chrono::Local;

    let now = Local::now();

    let log_file = format!(
        "{}/metta-bin/{}.log",
        std::env::var("HOME").unwrap(),
        chrono::Local::now().format("%Y-%m-%d").to_string()
    );

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("tests/resources/metta_run.metta")
        .output()
        .expect("failed to execute process")
        .stdout
        .lines()
        .last()
        .unwrap();

    let metta_output = match output {
        Ok(output) => output.to_string(),
        Err(_) => panic!("Error reading output"),
    };

    let expected_output = vec![
        metta_output,
        format!("Start time: {}", now.format("%Y-%m-%d %H-%m").to_string()),
    ];

    let mut file = File::open(&log_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let last_three_lines = contents.lines().rev().take(3).collect::<Vec<&str>>();
    let output = vec![last_three_lines[0], &last_three_lines[2][..28]];

    assert_eq!(output, expected_output);
}
