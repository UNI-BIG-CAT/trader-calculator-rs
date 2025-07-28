use crate::database::stock_action::StockActionRecord;
#[tauri::command]
pub fn handle_save_action_info(
    stock_action_id: i32,
    action_time: String,
    action_info: String,
) -> Result<(), String> {
    StockActionRecord::save_stock_action_info(stock_action_id, action_time, action_info)
        .map_err(|e| e.to_string())?;
    Ok(())
}
