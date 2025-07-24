use crate::constant::stock_type::StockType;
use crate::database::stock::{StockHandler, StockRecord};
// Tauri专用的简化函数
/// 获取所有股票 - 适配Tauri
#[tauri::command]
pub fn get_all_stocks() -> Vec<StockRecord> {
    StockHandler::get_all_stocks().unwrap()
}

/// 创建股票 - 适配Tauri
#[tauri::command]
pub fn open_stock(
    stock_name: String,
    stock_type: i32,
    current_price: f64,
    transaction_price: f64,
    transaction_amount: i32,
) -> Result<(), String> {
    let commission_fee_rate = match stock_type {
        StockType::from(1) => 0.0001,
        StockType::from(2) => 0.0001, // 深
        StockType::from(3) => 0.0001, // 创业板
        StockType::from(4) => 0.0001, // 科创板
        _ => 0.0001,                  // 默认值
    };
    StockHandler::insert_stock(&stock_name, stock_type, commission_fee_rate)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 负责股票列表显示
#[tauri::command]
pub fn load_stock_list() {}

/// 股票查看
#[tauri::command]
pub fn handle_view_stock(stock_id: i32) {}

/// 股票删除
#[tauri::command]
pub fn handle_delete_stock(stock_id: i32) {}
