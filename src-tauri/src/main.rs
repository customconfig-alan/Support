#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use std::process::Command;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_serial_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}