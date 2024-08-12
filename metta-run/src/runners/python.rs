use std::{
    env, fs,
    process::{Command, Stdio},
};

pub fn run(file_path: &String, arg: &String) -> String {
    // cehck if the file exists
    if !fs::metadata(&file_path).is_ok() {
        eprintln!("File not found: {}", file_path);
        std::process::exit(1);
    }

    let venv_dir = format!("{}/metta-bin/venv", env::var("HOME").unwrap());
    let python_interpreter = format!("{}/bin/python3", venv_dir);

    let python_output = Command::new(&python_interpreter)
        .arg(&file_path)
        .arg(&arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute Python script");

    if !python_output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&python_output.stderr));
        std::process::exit(1);
    }

    let python_output_str = String::from_utf8_lossy(&python_output.stdout);

    // Deactivate the virtual environment
    Command::new("bash")
        .arg("-c")
        .arg("deactivate")
        .output()
        .expect("Failed to deactivate virtual environment");

    python_output_str.to_string()
}
