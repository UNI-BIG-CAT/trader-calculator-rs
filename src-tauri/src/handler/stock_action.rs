use crate::constant::{
    action_type::ActionType, fee_rate::FeeRates, stock_status::StockStatus, stock_type::StockType,
};
use crate::database::stock::StockHandler;
use crate::database::stock_action::{StockActionHandler, StockActionRecord};

/// 获取股票操作记录
#[tauri::command]
pub fn get_action_list(stock_id: i32) -> Vec<StockActionRecord> {
    println!("get_action_list: stock_id={}", stock_id);
    let list = StockActionHandler::get_actions_by_stock_id(stock_id).unwrap_or_else(|e| {
        println!("Error getting actions: {}", e);
        Vec::new()
    });
    list
}

// 加仓
#[tauri::command]
pub fn handle_add_position(
    stock_id: i32,
    current_price: f64,
    transaction_price: f64,
    transaction_amount: i32,
) -> Result<(), String> {
    println!("add_stock:{stock_id},{current_price},{transaction_price},{transaction_amount}");
    let last_action = StockActionHandler::get_last_action(stock_id).map_err(|e| e.to_string())?;
    //
    let action_type = ActionType::AddPosition as i32;
    // 本次各项费用
    let fee_rate =
        FeeRates::for_stock_type(StockType::from(last_action.action), ActionType::AddPosition);
    let transaction_commission_fee =
        transaction_price * transaction_amount as f64 * fee_rate.commission;
    let transaction_tax_fee = transaction_price * transaction_amount as f64 * fee_rate.tax;
    let transaction_regulatory_fee =
        transaction_price * transaction_amount as f64 * fee_rate.regulatory;
    let transaction_brokerage_fee =
        transaction_price * transaction_amount as f64 * fee_rate.brokerage;
    let transaction_transfer_fee =
        transaction_price * transaction_amount as f64 * fee_rate.transfer;
    let total_fee = last_action.total_fee
        + transaction_commission_fee
        + transaction_tax_fee
        + transaction_regulatory_fee
        + transaction_brokerage_fee
        + transaction_transfer_fee;
    // 新总手数
    let total_amount = last_action.total_amount + transaction_amount as f64;
    // 新成本 = (原总金额+加仓金额) / 新总手数
    let current_cost = (last_action.current_cost + transaction_price) / total_amount;
    // 利润 = (当前价格 - 新成本) * 交易数量
    let profit = (current_price - current_cost) * total_amount;
    // 利润率 = 利润 / 当前成本
    let profit_rate = profit / current_cost;
    // 插入操作记录
    StockActionHandler::insert_action(
        stock_id,
        current_price,
        current_cost,
        total_amount,
        total_fee,
        transaction_price,
        transaction_amount as f64,
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

// 减仓
#[tauri::command]
pub fn handle_reduce_position(
    stock_id: i32,
    current_price: f64,
    transaction_price: f64,
    transaction_amount: i32,
) -> Result<(), String> {
    println!("reduce_stock:{stock_id},{current_price},{transaction_price},{transaction_amount}");
    let last_action = StockActionHandler::get_last_action(stock_id).unwrap();
    //
    let action_type = ActionType::ReducePosition as i32;
    // 本次各项费用
    let fee_rate = FeeRates::for_stock_type(
        StockType::from(last_action.action),
        ActionType::ReducePosition,
    );
    let transaction_commission_fee =
        transaction_price * transaction_amount as f64 * fee_rate.commission;
    let transaction_tax_fee = transaction_price * transaction_amount as f64 * fee_rate.tax;
    let transaction_regulatory_fee =
        transaction_price * transaction_amount as f64 * fee_rate.regulatory;
    let transaction_brokerage_fee =
        transaction_price * transaction_amount as f64 * fee_rate.brokerage;
    let transaction_transfer_fee =
        transaction_price * transaction_amount as f64 * fee_rate.transfer;
    let total_fee = last_action.total_fee
        + transaction_commission_fee
        + transaction_tax_fee
        + transaction_regulatory_fee
        + transaction_brokerage_fee
        + transaction_transfer_fee;
    // 新总手数
    let total_amount = last_action.total_amount - transaction_amount as f64;
    // 新成本 = (原总金额-减仓金额) / 新总手数
    let current_cost = (last_action.current_cost - transaction_price) / total_amount;
    // 利润 = (当前价格 - 新成本) * 交易数量
    let profit = (current_price - current_cost) * total_amount;
    // 利润率 = 利润 / 当前成本
    let profit_rate = profit / current_cost;
    // 插入操作记录
    StockActionHandler::insert_action(
        stock_id,
        current_price,
        current_cost,
        total_amount,
        total_fee,
        transaction_price,
        transaction_amount as f64,
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

// 回退
#[tauri::command]
pub fn handle_back_position(stock_id: i32) -> Result<(), String> {
    StockActionHandler::delete_last_action(stock_id).map_err(|e| e.to_string())?;
    StockHandler::update_stock_status(stock_id, StockStatus::OPEN as i32)
        .map_err(|e| e.to_string())?;
    Ok(())
}

// 平仓
#[tauri::command]
pub fn handle_close_position(stock_id: i32, current_price: f64) -> Result<(), String> {
    println!("close_stock:{stock_id},{current_price}");
    let last_action = StockActionHandler::get_last_action(stock_id).map_err(|e| e.to_string())?;
    //
    let action_type = ActionType::Close as i32;
    // 本次各项费用
    let fee_rate = FeeRates::for_stock_type(StockType::from(last_action.action), ActionType::Close);
    let transaction_commission_fee =
        current_price * last_action.total_amount as f64 * fee_rate.commission;
    let transaction_tax_fee = current_price * last_action.total_amount as f64 * fee_rate.tax;
    let transaction_regulatory_fee =
        current_price * last_action.total_amount as f64 * fee_rate.regulatory;
    let transaction_brokerage_fee =
        current_price * last_action.total_amount as f64 * fee_rate.brokerage;
    let transaction_transfer_fee =
        current_price * last_action.total_amount as f64 * fee_rate.transfer;
    let total_fee = last_action.total_fee
        + transaction_commission_fee
        + transaction_tax_fee
        + transaction_regulatory_fee
        + transaction_brokerage_fee
        + transaction_transfer_fee;
    // 利润
    let profit = (current_price - last_action.current_cost) * last_action.total_amount;
    let profit_rate = profit / (last_action.current_cost * last_action.total_amount);
    // 插入操作记录
    StockActionHandler::insert_action(
        stock_id,
        current_price,
        last_action.current_cost,
        0.0,
        total_fee,
        current_price,
        last_action.total_amount,
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
    StockHandler::update_stock_status(stock_id, StockStatus::CLOSE as i32)
        .map_err(|e| e.to_string())?;
    Ok(())
}
