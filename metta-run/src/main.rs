use std::env;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let venv_dir = format!("{}/metta-bin/venv", env::var("HOME").unwrap());
    let python_interpreter = format!("{}/bin/python3", venv_dir);
    let formatter_path = format!(
        "{}/metta-bin/tools/formatter/tree-formater.py",
        env::var("HOME").unwrap()
    );

    // Argument parsing
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} [--ft] <file>", args[0]);
        std::process::exit(1);
    }

    let mut format_tree = false;
    let file_arg: &String;

    if args.len() == 3 {
        if args[1] == "--ft" {
            format_tree = true;
            file_arg = &args[2];
        } else {
            eprintln!("Usage: {} [--ft] <file>", args[0]);
            std::process::exit(1);
        }
    } else {
        file_arg = &args[1];
    }

    if !fs::metadata(file_arg).is_ok() {
        eprintln!("File not found: {}", file_arg);
        std::process::exit(1);
    }

    // Activate the virtual environment
    let activate_script = format!("{}/bin/activate", venv_dir);
    let activate_command = format!("source {} && metta {}", activate_script, file_arg);

    let metta_output = Command::new("sh")
        .arg("-c")
        .arg(&activate_command)
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

    if format_tree {
        // Run the formatter with the metta output
        let python_output = Command::new(&python_interpreter)
            .arg(&formatter_path)
            .arg(metta_output_str.trim())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to execute Python script");

        if !python_output.status.success() {
            eprintln!("Failed to format tree");
            std::process::exit(1);
        }
    } else {
        print!("{}", metta_output_str);
    }

    // Deactivate the virtual environment
    Command::new("sh")
        .arg("-c")
        .arg("deactivate")
        .output()
        .expect("Failed to deactivate virtual environment");

    Ok(())
}
