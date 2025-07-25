use crate::database::stock_action::{StockActionHandler, StockActionRecord};

/// 获取股票操作记录 - 适配Tauri
#[tauri::command]
pub fn get_action_list(stock_id: i32) -> Vec<StockActionRecord> {
    println!("get_action_list: stock_id={}", stock_id);
    let list = StockActionHandler::get_actions_by_stock_id(stock_id).unwrap_or_else(|e| {
        println!("Error getting actions: {}", e);
        Vec::new()
    });
    list
}

#[tauri::command]
pub fn handle_add_position() {}

#[tauri::command]
pub fn handle_reduce_position() {}

#[tauri::command]
pub fn handle_back_position() {}

#[tauri::command]
pub fn handle_close_position() {}
