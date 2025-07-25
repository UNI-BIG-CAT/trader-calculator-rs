// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod constant;
mod database;
mod handler;
//
use crate::handler::stock::{get_all_stocks, handle_delete_stock, open_stock};
use crate::handler::stock_action::get_action_list;
//
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_all_stocks,
            get_action_list,
            open_stock,
            handle_delete_stock
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
