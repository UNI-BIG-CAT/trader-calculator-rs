// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod constant;
mod database;
mod handler;
//
use crate::handler::background::check_background_image;
use crate::handler::stock::{
    handle_delete_stock, handle_get_all_stocks, handle_get_stock_info, handle_update_stock_sort,
};
use crate::handler::stock_action::{
    handle_add_position, handle_back_position, handle_close_position, handle_get_action_list,
    handle_open_position, handle_reduce_position,
};
use crate::handler::stock_action_info::handle_save_action_info;
//
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handle_get_all_stocks,
            handle_get_stock_info,
            handle_delete_stock,
            handle_update_stock_sort,
            //
            handle_get_action_list,
            handle_open_position,
            handle_add_position,
            handle_back_position,
            handle_reduce_position,
            handle_close_position,
            handle_delete_stock,
            //
            handle_save_action_info,
            check_background_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
