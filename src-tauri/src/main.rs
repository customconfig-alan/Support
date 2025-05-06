#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use std::process::Command;
use std::path::PathBuf;

#[command]
fn get_serial_number() -> Result<String, String> {
    let output = Command::new("system_profiler")
        .arg("SPHardwareDataType")
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.trim().starts_with("Serial Number") {
            return Ok(line.split(':').nth(1).unwrap_or("").trim().to_string());
        }
    }

    Err("Serial number not found".to_string())
}

#[command]
fn save_user_data(user: String) -> Result<(), String> {
    let mut app_data_path = PathBuf::from(std::env::var("HOME").map_err(|e| e.to_string())?);
    app_data_path.push("Library/Application Support/CustomConfig/.login.json");
    std::fs::write(app_data_path, user).map_err(|e| e.to_string())
}

#[command]
fn load_user_data() -> Result<String, String> {
    let mut app_data_path = PathBuf::from(std::env::var("HOME").map_err(|e| e.to_string())?);
    app_data_path.push("Library/Application Support/CustomConfig/.login.json");

    if !app_data_path.exists() {
        std::fs::create_dir_all(app_data_path.parent().unwrap()).map_err(|e| e.to_string())?;
        std::fs::write(&app_data_path, "{}").map_err(|e| e.to_string())?;
    }

    std::fs::read_to_string(app_data_path).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_serial_number, save_user_data, load_user_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}