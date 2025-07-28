use crate::database::stock::StockRecord;

// Tauri专用的简化函数
/// 获取所有股票 - 适配Tauri
#[tauri::command]
pub fn handle_get_all_stocks() -> Vec<StockRecord> {
    println!("get_all_stocks");
    let list = StockRecord::get_all_stocks().unwrap();
    println!("list: {:?}", list);
    list
}

// 获取股票信息
#[tauri::command]
pub fn handle_get_stock_info(stock_id: i32) -> Result<Option<StockRecord>, String> {
    println!("get_stock_info:{stock_id}");
    let stock = StockRecord::get_stock_by_id(stock_id).map_err(|e| e.to_string())?;
    Ok(stock)
}

/// 股票删除
#[tauri::command]
pub fn handle_delete_stock(stock_id: i32) -> Result<(), String> {
    println!("handle_delete_stock:{stock_id}");
    StockRecord::delete_stock(stock_id).map_err(|e| e.to_string())?;
    Ok(())
}
