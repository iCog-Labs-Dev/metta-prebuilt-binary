use std::process::{Command, Stdio};
use std::{env, fs};

pub fn run(file_path: String) -> String {
    // cehck if the file exists
    if !fs::metadata(&file_path).is_ok() {
        eprintln!("File not found: {}", file_path);
        std::process::exit(1);
    }

    let venv_dir = format!("{}/metta-bin/venv", env::var("HOME").unwrap());

    // Activate the virtual environment and run the file with metta
    let activate_script = format!("{}/bin/activate", venv_dir);
    let metta_runner = format!("source {} && metta {}", activate_script, file_path);

    let metta_output = Command::new("bash")
        .arg("-c")
        .arg(&metta_runner)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    let metta_output_str = String::from_utf8_lossy(&metta_output.stdout);
    let metta_output_stderr = String::from_utf8_lossy(&metta_output.stderr);

    if !metta_output.status.success() {
        eprintln!("{}", metta_output_stderr);
        std::process::exit(1);
    }

    // Deactivate the virtual environment
    Command::new("bash")
        .arg("-c")
        .arg("deactivate")
        .output()
        .expect("Failed to deactivate virtual environment");

    format!("{}{}", metta_output_stderr, metta_output_str)
        .trim()
        .to_string()
}
