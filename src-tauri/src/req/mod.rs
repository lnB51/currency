use crate::{
    config::{modify_config, read_config_value},
    log::log,
};
use chrono::{TimeZone, Utc};
use reqwest;
use serde_json::Value;
use std::{fs::File, io::prelude::*, path::PathBuf, str::FromStr};
use xml2json_rs::JsonBuilder;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_currency_data(data_dir: String) -> Result<String, String> {
    let format = read_config_value("output".to_string(), data_dir.clone())?;
    log(
        "INFO".to_string(),
        format!("Retrieving currency data in {} format", format),
        data_dir.clone(),
    )?;

    // Checking the output format to avoid errors
    let file_extension = match format.as_str() {
        "JSON" => "json",
        "XML" => "xml",
        _ => return Err("Invalid format specified".to_string()),
    };

    let file_path = PathBuf::from(&data_dir).join(format!("currency.{}", file_extension));
    use std::fs::File;

    // Open file, if not exist create new with new request and retry
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            // Log the error
            log(
                "WARNING".to_string(),
                format!(
                    "Failed to open file: {}, make request to solve the problem",
                    err
                ),
                data_dir.clone(),
            )?;
            // Call fetch_and_save_currency_rates
            fetch_and_save_currency_rates(data_dir.clone(), "true".to_string()).await?;
            // Attempt to open the file again
            match File::open(&file_path) {
                Ok(file) => file,
                Err(err) => {
                    // Log the error if opening the file fails again
                    log(
                        "ERROR".to_string(),
                        format!("Failed to open file again after fetching data: {}", err),
                        data_dir.clone(),
                    )?;
                    // Return the error
                    return Err(format!(
                        "Failed to open file again after fetching data: {}",
                        err
                    ));
                }
            }
        }
    };


    // Read file to string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| format!("Failed to read file: {}", err))?;
    let json_builder = JsonBuilder::default();

    // Parse the JSON string into a serde_json::Value
    let data = match format.as_str() {
        "JSON" => serde_json::Value::from_str(&contents.to_string())
            .map_err(|err| format!("Failed to parse JSON: {}", err))?,
        "XML" => json_builder
            .build_from_xml(&contents)
            .map_err(|err| format!("Failed to parse XML: {}", err))?,
        _ => return Err("Invalid format specified".into()),
    };

    // Filter the currency data by the "cc" field
    let filtered_data = match &data["exchange"]["currency"] {
        Value::Array(arr) => {
            let filtered_arr: Vec<&Value> = arr
                .iter()
                .filter(|item| {
                    item["cc"].as_array().map_or(false, |cc| {
                        cc.iter().any(|cc_value| {
                            cc_value.as_str() == Some("CNY")
                                || cc_value.as_str() == Some("USD")
                                || cc_value.as_str() == Some("EUR")
                                || cc_value.as_str() == Some("CAD")
                                || cc_value.as_str() == Some("CZK")
                        })
                    })
                })
                .collect();
            Value::Array(filtered_arr.into_iter().cloned().collect())
        }
        _ => Value::Null, // Return null if the currency data is not in the expected format
    };

    // Serialize the filtered data back into a JSON string
    let filtered_json = serde_json::to_string(&filtered_data)
        .map_err(|err| format!("Failed to serialize JSON: {}", err))?;

    Ok(filtered_json)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn fetch_and_save_currency_rates(data_dir: String, manual: String) -> Result<(), String> {

    // Read config params and calculate duration of last update
    let last_update = read_config_value("last_update".to_string(), data_dir.clone())?
        .parse::<i64>()
        .unwrap_or(0);
    let current_time = Utc::now();
    let frequency = read_config_value("frequency".to_string(), data_dir.clone())?
        .parse::<u32>()
        .unwrap_or(1); // Assuming a default value of 1 if the frequency is not specified
    let duration_since_last_update = current_time
        .signed_duration_since(Utc.timestamp(last_update, 0))
        .num_seconds();

    // If frequency between 1 and 5 and last update was 24 / frq (if frequency is 4, register will update each 6 hours) make request
    if (frequency > 0
        && frequency < 5
        && (duration_since_last_update >= (24 / frequency) as i64 * 3600))
        || manual == "true"
    {
        let format = read_config_value("output".to_string(), data_dir.clone()).unwrap();
        log(
            "INFO".to_string(),
            format!("Fetching currency rates from the API in {} format", format),
            data_dir.clone(),
        )?;

        // Make a request to the API
        let response = reqwest::get("https://bank.gov.ua/NBUStatService/v1/statdirectory/exchange")
            .await
            .map_err(|err| format!("Failed to fetch currency rates: {}", err))?;

        // Check if the request was successful
        if !response.status().is_success() {
            log(
                "WARNING".to_string(),
                format!("Failed to fetch currency rates: {}", response.status()),
                data_dir.clone(),
            )?;
            return Err(format!(
                "Failed to fetch currency rates: {}",
                response.status()
            ));
        }

        // Read the response body
        let body = response
            .text()
            .await
            .map_err(|err| format!("Failed to read response body: {}", err))?;

        // Save the currency rates to a file
        save_currency_rates(format, &data_dir, body)?;

        // Update last_update variable in config
        modify_config(
            current_time.timestamp().to_string(), // Convert current_time to a string representing the timestamp
            "last_update".to_string(),
            data_dir,
        )?;
    }

    Ok(())
}

fn save_currency_rates(
    format: String,
    data_dir: &str,
    currency_rates: String,
) -> Result<(), String> {
    let file_extension = match format.as_str() {
        "JSON" => "json",
        "XML" => "xml",
        _ => return Err("Invalid format specified".to_string()),
    };

    // Construct the file path
    let file_path = PathBuf::from(&data_dir).join(format!("currency.{}", file_extension));
    let json_builder = JsonBuilder::default();
    // Serialize the currency rates based on the specified format
    let serialized_data = match format.as_str() {
        "JSON" => json_builder
            .build_pretty_string_from_xml(&currency_rates)
            .map_err(|err| format!("Failed to serialize JSON: {}", err))?,
        "XML" => currency_rates,
        _ => return Err("Invalid format specified".to_string()),
    };

    // Write the serialized data to the file
    let mut file =
        File::create(&file_path).map_err(|err| format!("Failed to create file: {}", err))?;
    file.write_all(serialized_data.as_bytes())
        .map_err(|err| format!("Failed to write to file: {}", err))?;

    Ok(())
}
