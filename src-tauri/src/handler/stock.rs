use crate::constant::{action_type::ActionType, fee_rate::FeeRates, stock_type::StockType};
use crate::database::stock::{StockHandler, StockRecord};
use crate::database::stock_action::StockActionHandler;

// Tauri专用的简化函数
/// 获取所有股票 - 适配Tauri
#[tauri::command]
pub fn handle_get_all_stocks() -> Vec<StockRecord> {
    println!("get_all_stocks");
    let list = StockHandler::get_all_stocks().unwrap();
    println!("list: {:?}", list);
    list
}

// 获取股票信息
#[tauri::command]
pub fn handle_get_stock_info(stock_id: i32) -> Result<Option<StockRecord>, String> {
    println!("get_stock_info:{stock_id}");
    let stock = StockHandler::get_stock_by_id(stock_id).map_err(|e| e.to_string())?;
    Ok(stock)
}

/// 股票删除
#[tauri::command]
pub fn handle_delete_stock(stock_id: i32) -> Result<(), String> {
    println!("handle_delete_stock:{stock_id}");
    StockHandler::delete_stock(stock_id).map_err(|e| e.to_string())?;
    Ok(())
}
