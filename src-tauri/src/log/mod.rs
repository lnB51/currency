use std::{
    fs::OpenOptions,
    io::Write,
    path::Path, process::Command,
};
use chrono::Local;

#[tauri::command(rename_all = "snake_case")]
pub fn log(level: String, msg: String, data_dir: String) -> Result<(), String> {
    let log_path = Path::new(&data_dir).join("logs.txt");

    // Open the log file in append mode, creating it if it doesn't exist
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|err| err.to_string())?;

    // Get the current time
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Format the log entry
    let log_entry = format!("{} - {} - {}\n", current_time, level, msg);

    // Write the log entry to the file
    log_file
        .write_all(log_entry.as_bytes())
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_log(data_dir: String) -> Result<String, String> {
    let log_path = Path::new(&data_dir).join("logs.txt");

    // Check if the log file exists
    if !log_path.exists() {
        return Err("Log file not found".to_string());
    }

    // Open the log file in Notepad
    Command::new("notepad")
        .arg(&log_path)
        .spawn()
        .map_err(|err| err.to_string())?;

    let _ = log("INFO".to_string(), "Logs file opened".to_string(), data_dir);

    Ok("Log file opened in Notepad".to_string())
}