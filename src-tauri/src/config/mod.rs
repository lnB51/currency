use serde_json::{from_str, json, Value};
use std::{
    fs::{self, File, OpenOptions},
    io::prelude::*,
    path::{Path, PathBuf}, process::Command,
};

use crate::log::log;

fn ensure_config_exist(config_path: &PathBuf, data_dir: &String) -> Result<String, String> {
    // Ensure the directory exists before writing the file
    if let Some(parent_dir) = config_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).map_err(|err| err.to_string())?;
        }
    }

    // Create the file if it doesn't exist
    if !config_path.exists() {
        let _ = log(
            "INFO".to_string(),
            "Settings file not exist".to_string(),
            data_dir.clone(),
        );

        let config_template = json!({
            "frequency": "1",
            "language": "en",
            "output": "JSON",
            "last_update": "171"
        });

        let mut config_file = File::create(&config_path).map_err(|err| err.to_string())?;
        config_file
            .write_all(config_template.to_string().as_bytes())
            .map_err(|err| err.to_string())?;
    }
    Ok("Config exist".to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn modify_config(value: String, id: String, data_dir: String) -> Result<String, String> {
    let config_path = Path::new(&data_dir).join("config.json");

    let _ = ensure_config_exist(&config_path, &data_dir);

    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config_path)
        .map_err(|err| err.to_string())?;

    let mut content = String::new();
    config_file
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;

    let mut config_json: Value = if content.trim().is_empty() {
        json!({})
    } else {
        serde_json::from_str(&content).map_err(|err| err.to_string())?
    };

    // Modify the id to value or add it if it doesn't exist
    config_json[&id] = json!(value);

    // Write the modified JSON back to the file
    let modified_content =
        serde_json::to_string_pretty(&config_json).map_err(|err| err.to_string())?;
        
    config_file
        .seek(std::io::SeekFrom::Start(0))
        .map_err(|err| err.to_string())?; // Move to the beginning of the file

    config_file.set_len(0).map_err(|err| err.to_string())?; // Clear the file contents

    config_file
        .write_all(modified_content.as_bytes())
        .map_err(|err| err.to_string())?;
    let _ = log(
        "INFO".to_string(),
        format!("Config value: {id} modified to: {value}"),
        data_dir,
    );
    Ok("Config file modified successfully".to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn read_config_value(key: String, data_dir: String) -> Result<String, String> {
    let config_path = Path::new(&data_dir).join("config.json");

    let _ = ensure_config_exist(&config_path, &data_dir);

    // Open the config file
    let mut config_file = File::open(&config_path).map_err(|err| err.to_string())?;

    // Read the content of the config file into a string
    let mut content = String::new();
    config_file
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;

    // Parse the content into a JSON Value
    let config_json: Value = from_str(&content).map_err(|err| err.to_string())?;

    // Retrieve the value associated with the key
    let value = config_json[key.as_str()].clone();

    // Convert the value to a string
    let value_str = match value {
        Value::String(s) => s,
        _ => value.to_string(),
    };

    // Remove quotation marks if present
    let value_str = value_str.trim_matches('"');
    // Log the action
    let _ = log(
        "INFO".to_string(),
        format!("Get config value: {key}"),
        data_dir,
    );
    Ok(value_str.to_string())
}


#[tauri::command(rename_all = "snake_case")]
pub fn open_data_dir(data_dir: String) -> Result<String, String> {
    // Log the action
    log(
        "INFO".to_string(),
        format!("Open DataDir"),
        data_dir.clone(),
    ).unwrap_or_else(|err| {
        eprintln!("Failed to log action: {}", err);
    });

    // Open the data directory in the file explorer
    let result = if cfg!(target_os = "windows") {
        Command::new("explorer")
            .arg(&data_dir)
            .spawn()
            .map_err(|err| err.to_string())
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(&data_dir)
            .spawn()
            .map_err(|err| err.to_string())
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .arg(&data_dir)
            .spawn()
            .map_err(|err| err.to_string())
    } else {
        Err("Unsupported operating system".to_string())
    };

    match result {
        Ok(_) => Ok("Data directory opened in file explorer".to_string()),
        Err(err) => Err(err),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_exchange_rate(data_dir: String) -> Result<String, String> {
    let output = match read_config_value("output".to_string(), data_dir.clone())?.as_str() {
        "JSON" => Ok("json"),
        "XML" => Ok("xml"),
        _ => Err("Invalid format specified".to_string()),
    };
    let log_path = Path::new(&data_dir).join(format!("currency.{}", output.unwrap()));

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
