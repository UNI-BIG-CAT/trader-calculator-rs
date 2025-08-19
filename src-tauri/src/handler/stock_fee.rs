use crate::database::stock_fee::StockFeeRate;

/// 获取股票操作记录
#[tauri::command]
pub fn handle_stock_fee() -> Result<StockFeeRate, String> {
    let stock_fee = StockFeeRate::get_fee().map_err(|e| e.to_string())?;
    Ok(stock_fee)
}

#[tauri::command]
pub fn handle_stock_fee_update(
    commission_fee_rate: f64,
    tax_fee_rate: f64,
    regulatory_fee_rate: f64,
    brokerage_fee_rate: f64,
    transfer_fee_rate: f64,
) -> Result<(), String> {
    println!("handle_stock_fee_create");
    StockFeeRate::update(
        commission_fee_rate,
        tax_fee_rate,
        regulatory_fee_rate,
        brokerage_fee_rate,
        transfer_fee_rate,
    )
    .map_err(|err| {
        eprintln!("Error updating stock fee: {}", err);
        format!("Failed to update stock fee: {}", err)
    })?;
    Ok(())
}
