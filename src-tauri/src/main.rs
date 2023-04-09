// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod exif;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Tauri command for exif data
#[tauri::command]
fn read_exif(path: &str) -> String {
    match exif::get_exif(path) {
        Ok(data) => {
            let mut exif_data = String::new();
            for (key, value) in data {
                exif_data.push_str(&format!("{}: {}\n", key, value));
            }
            exif_data
        }
        Err(e) => format!("Error: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read_exif])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
