use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};

pub fn start_timer() -> Instant {
    Instant::now()
}

pub fn stop_timer(start_time: Instant, metta_output: &String) -> Result<(), std::io::Error> {
    let now = Local::now();
    let formatted_date = now.format("%Y-%m-%d").to_string();
    let log_file_name = format!("{}.log", formatted_date);

    let mut output_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&log_file_name)?;

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let final_output = format!(
        "\nStart time: {}\nElapsed time: {:.3}\n{}\n",
        now.format("%Y-%m-%d %H-%m-%s").to_string(),
        elapsed_time.as_secs_f32(),
        metta_output
    );
    output_file.write_all(final_output.as_bytes())?;

    Ok(())
}
