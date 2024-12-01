// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::archive::decompress;
use crate::models::MCP_DB_DEFAULT_PATH;
use crate::sqlite::{create_tables, load_codices};
use std::path::Path;
use std::process::Command;

mod archive;
mod models;
mod sqlite;

#[tauri::command]
fn load(bytes: Vec<u8>) -> String {
    let full_db_path = shellexpand::tilde(MCP_DB_DEFAULT_PATH);
    let db_path = Path::new(full_db_path.as_ref());

    match decompress(&bytes) {
        Ok(s) => {
            let decompressed_path = Path::new(&s);
            let result = create_tables(decompressed_path, db_path)
                .and_then(|conn| load_codices(decompressed_path, &conn))
                .and_then(|_| {
                    Command::new("open")
                        .arg("/Applications/Claude.app")
                        .spawn()
                        .map_err(anyhow::Error::new)
                });

            if let Err(e) = result {
                e.to_string()
            } else {
                "Successfully created the database and launched Claude Desktop!".into()
            }
        }
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
fn launch() -> String {
    match Command::new("open")
        .arg("/Applications/Claude.app")
        .spawn() {
        Ok(_) => {
            "Successfully launched Claude Desktop!".into()
        }
        Err(e) => e.to_string()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![load, launch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
