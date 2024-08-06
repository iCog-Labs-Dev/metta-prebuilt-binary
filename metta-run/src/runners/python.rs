use std::{
    env,
    process::{Command, Output, Stdio},
};

pub fn run(file_path: &String, arg: &String) -> Output {
    let venv_dir = format!("{}/metta-bin/venv", env::var("HOME").unwrap());
    let python_interpreter = format!("{}/bin/python3", venv_dir);

    let python_output = Command::new(&python_interpreter)
        .arg(&file_path)
        .arg(&arg)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute Python script");

    // Deactivate the virtual environment
    Command::new("bash")
        .arg("-c")
        .arg("deactivate")
        .output()
        .expect("Failed to deactivate virtual environment");

    python_output
}
