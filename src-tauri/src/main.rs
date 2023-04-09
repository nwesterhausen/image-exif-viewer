// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod png;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Tauri command for exif data
#[tauri::command]
fn read_exif(path: &str) -> String {
    match png::read_png(&path) {
        Ok(data) => format!("{:?}", data),
        Err(e) => format!("Error: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read_exif])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
