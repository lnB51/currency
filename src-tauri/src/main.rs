// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, WindowEvent};
use window_shadows::set_shadow;

mod config;
mod log;
mod req;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .on_window_event(|e| {
            if let WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        })
        .invoke_handler(tauri::generate_handler![
            config::modify_config,
            config::read_config_value,
            config::open_data_dir,
            config::open_exchange_rate,
            log::open_log,
            req::fetch_and_save_currency_rates,
            req::get_currency_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
