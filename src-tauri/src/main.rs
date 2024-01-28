// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{PhysicalSize, Size};



#[tauri::command]
fn executor_resize(window: tauri::Window) {
    window.set_size(Size::Physical(PhysicalSize{width: 1000, height: 500}));
    window.set_title("MSHX - Executor");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![executor_resize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
