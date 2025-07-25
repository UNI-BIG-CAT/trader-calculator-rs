use crate::constant::action_type::ActionType;
use crate::constant::fee_rate::FeeRates;
use crate::constant::stock_type::StockType;
use crate::database::stock::{StockHandler, StockRecord};
use crate::database::stock_action::StockActionHandler;

// Tauri专用的简化函数
/// 获取所有股票 - 适配Tauri
#[tauri::command]
pub fn get_all_stocks() -> Vec<StockRecord> {
    println!("get_all_stocks");
    let list = StockHandler::get_all_stocks().unwrap();
    println!("list: {:?}", list);
    list
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
    println!("open_stock:{stock_name},{stock_type},{current_price},{transaction_price},{transaction_amount}");
    // 插入股票及其费率
    let fee_rates = FeeRates::for_stock_type(StockType::from(stock_type), ActionType::Open);
    let stock_id = StockHandler::insert_stock(
        &stock_name,
        stock_type,
        fee_rates.commission,
        fee_rates.tax,
        fee_rates.regulatory,
        fee_rates.brokerage,
        fee_rates.transfer,
    )
    .map_err(|e| e.to_string())?;

    // 记录开仓价格数据
    let current_cost = transaction_price; // 开仓:成本 = 交易价格
    let total_amount = transaction_amount as f64; // 开仓:总数 = 交易数量
    let transaction_amount_f64 = transaction_amount as f64;
    let transaction_value = transaction_price * transaction_amount_f64;

    // 计算各种费用
    let transaction_commission_fee = transaction_value * fee_rates.commission;
    let transaction_tax_fee = transaction_value * fee_rates.tax;
    let transaction_regulatory_fee = transaction_value * fee_rates.regulatory;
    let transaction_brokerage_fee = transaction_value * fee_rates.brokerage;
    let transaction_transfer_fee = transaction_value * fee_rates.transfer;

    let action_type = ActionType::Open as i32; // 操作类型
    let profit = (current_price - current_cost) * total_amount; //利润
    let profit_rate = profit / (current_cost * total_amount); //利润率

    StockActionHandler::insert_action(
        stock_id as i32,
        current_price,
        current_cost,
        total_amount,
        transaction_price,
        transaction_amount_f64,
        transaction_commission_fee,
        transaction_tax_fee,
        transaction_regulatory_fee,
        transaction_brokerage_fee,
        transaction_transfer_fee,
        action_type,
        profit,
        profit_rate,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// 股票删除
#[tauri::command]
pub fn handle_delete_stock(stock_id: i32) -> Result<(), String> {
    println!("handle_delete_stock:{stock_id}");
    StockHandler::delete_stock(stock_id).map_err(|e| e.to_string())?;
    Ok(())
}
