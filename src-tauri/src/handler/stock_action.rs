use crate::constant::{
    action_type::ActionType, fee_rate::FeeRates, stock_status::StockStatus, stock_type::StockType,
};
use crate::database::stock::StockHandler;
use crate::database::stock_action::{StockActionHandler, StockActionRecord};

fn calculate_safe_profit_rate(profit: f64, cost: f64, position: f64) -> f64 {
    // println!("profit:{profit},cost:{cost},position:{position}");
    let mut denominator = (cost * position).abs();
    // 检查分母是否接近0或为0
    if denominator < f64::EPSILON {
        denominator = profit;
    }
    let rate = profit / denominator;
    // 检查结果是否为无穷大或NaN
    println!(
        "profit:{profit},cost:{cost},position:{position},denominator:{denominator},rate:{rate},{},{}",
        rate.is_infinite(),
        rate.is_nan()
    );
    if rate.is_infinite() || rate.is_nan() {
        return 0.0; // 返回0作为默认值
    }
    rate
}

/// 获取股票操作记录
#[tauri::command]
pub fn handle_get_action_list(stock_id: i32) -> Vec<StockActionRecord> {
    println!("get_action_list: stock_id={}", stock_id);
    let list = StockActionHandler::get_actions_by_stock_id(stock_id).unwrap_or_else(|e| {
        println!("Error getting actions: {}", e);
        Vec::new()
    });
    list
}

/// 开仓
#[tauri::command]
pub fn handle_open_position(
    stock_name: String,
    stock_type: i32,
    current_price: f64,
    transaction_price: f64,
    transaction_position: i32,
    commission_fee_rate: f64,
) -> Result<(), String> {
    // 插入股票及其费率
    let fee_rates = FeeRates::for_stock_type(StockType::from(stock_type), ActionType::Open);
    let stock_id = StockHandler::insert_stock(
        &stock_name,
        stock_type,
        commission_fee_rate,
        fee_rates.tax,
        fee_rates.regulatory,
        fee_rates.brokerage,
        fee_rates.transfer,
    )
    .map_err(|e| e.to_string())?;

    // 记录开仓价格数据
    let current_cost = transaction_price; // 开仓:成本 = 交易价格
    let total_position = transaction_position as f64; // 开仓:总数 = 交易数量
    let transaction_position_f64 = transaction_position as f64;
    let transaction_value = transaction_price * transaction_position_f64;

    // 计算各种费用
    let mut transaction_commission_fee = transaction_value * commission_fee_rate;
    if transaction_commission_fee < 5.0 {
        // 佣金最少5元
        transaction_commission_fee = 5.0;
    }
    let transaction_tax_fee = transaction_value * 0.0; // 买入时不收印花税
    let transaction_regulatory_fee = transaction_value * fee_rates.regulatory;
    let transaction_brokerage_fee = transaction_value * fee_rates.brokerage;
    let transaction_transfer_fee = transaction_value * fee_rates.transfer;
    let total_fee = transaction_commission_fee
        + transaction_tax_fee
        + transaction_regulatory_fee
        + transaction_brokerage_fee
        + transaction_transfer_fee;
    //
    let action_type = ActionType::Open as i32; // 操作类型
                                               //利润
    let profit = (current_price - current_cost) * total_position;
    // 利润率
    let profit_rate = calculate_safe_profit_rate(profit, current_cost, total_position);

    StockActionHandler::insert_action(
        stock_id as i32,
        current_price,
        current_cost,
        total_position,
        total_fee,
        transaction_price,
        transaction_position_f64,
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

// 加仓
#[tauri::command]
pub fn handle_add_position(
    stock_id: i32,
    current_price: f64,
    transaction_price: f64,
    transaction_position: i32,
) -> Result<(), String> {
    println!("add_stock:{stock_id},{current_price},{transaction_price},{transaction_position}");
    let stock = StockHandler::get_stock_by_id(stock_id)
        .map_err(|e| e.to_string())?
        .ok_or("Stock not found")?;
    let last_action = StockActionHandler::get_last_action(stock_id).map_err(|e| e.to_string())?;
    //
    let action_type = ActionType::AddPosition as i32;
    // 本次各项费用
    let fee_rate =
        FeeRates::for_stock_type(StockType::from(last_action.action), ActionType::AddPosition);
    let mut transaction_commission_fee =
        transaction_price * transaction_position as f64 * stock.commission_fee_rate;
    if transaction_commission_fee < 5.0 {
        // 佣金最少5元
        transaction_commission_fee = 5.0;
    }
    let transaction_tax_fee = transaction_price * transaction_position as f64 * fee_rate.tax;
    let transaction_regulatory_fee =
        transaction_price * transaction_position as f64 * fee_rate.regulatory;
    let transaction_brokerage_fee =
        transaction_price * transaction_position as f64 * fee_rate.brokerage;
    let transaction_transfer_fee =
        transaction_price * transaction_position as f64 * fee_rate.transfer;
    let total_fee = last_action.total_fee
        + transaction_commission_fee
        + transaction_tax_fee
        + transaction_regulatory_fee
        + transaction_brokerage_fee
        + transaction_transfer_fee;
    // 总仓位
    let total_position = last_action.total_position + transaction_position as f64;
    // 新成本价（加权平均）
    let current_cost = (last_action.current_cost * last_action.total_position
        + transaction_price * transaction_position as f64)
        / total_position;
    // 当前整体浮动利润
    let profit = (current_price - current_cost) * total_position;
    // 利润率：以当前总成本为基准
    let profit_rate = calculate_safe_profit_rate(profit, current_cost, total_position);
    // 插入操作记录
    StockActionHandler::insert_action(
        stock_id,
        current_price,
        current_cost,
        total_position,
        total_fee,
        transaction_price,
        transaction_position as f64,
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
    transaction_position: i32,
) -> Result<(), String> {
    println!("reduce_stock:{stock_id},{current_price},{transaction_price},{transaction_position}");
    let stock = StockHandler::get_stock_by_id(stock_id)
        .map_err(|e| e.to_string())?
        .ok_or("Stock not found")?;
    let last_action = StockActionHandler::get_last_action(stock_id).unwrap();
    if transaction_position >= last_action.total_position as i32 {
        return Err("请选择平仓".to_string());
    }
    //
    let action_type = ActionType::ReducePosition as i32;
    // 本次各项费用
    let fee_rate = FeeRates::for_stock_type(
        StockType::from(last_action.action),
        ActionType::ReducePosition,
    );
    let transaction_commission_fee =
        transaction_price * transaction_position as f64 * stock.commission_fee_rate;
    let transaction_tax_fee = transaction_price * transaction_position as f64 * fee_rate.tax;
    let transaction_regulatory_fee =
        transaction_price * transaction_position as f64 * fee_rate.regulatory;
    let transaction_brokerage_fee =
        transaction_price * transaction_position as f64 * fee_rate.brokerage;
    let transaction_transfer_fee =
        transaction_price * transaction_position as f64 * fee_rate.transfer;
    let total_fee = last_action.total_fee
        + transaction_commission_fee
        + transaction_tax_fee
        + transaction_regulatory_fee
        + transaction_brokerage_fee
        + transaction_transfer_fee;
    // 采用利润反向摊薄计算剩余成本的方式（券商常见写法之一）
    // 新总手数
    let total_position = last_action.total_position - transaction_position as f64;
    // 摊薄成本价 = (原成本*原总手数-卖出价格*卖出手数) / 新总手数
    let current_cost = (last_action.current_cost * last_action.total_position
        - transaction_price * transaction_position as f64)
        / total_position;
    // 利润 = (当前价格 - 持仓成本) * 当前持仓数量
    let profit = (current_price - current_cost) * total_position;
    // 利润率 = 利润 / 当前成本
    let profit_rate = calculate_safe_profit_rate(profit, current_cost, total_position);
    // 插入操作记录
    StockActionHandler::insert_action(
        stock_id,
        current_price,
        current_cost,
        total_position,
        total_fee,
        transaction_price,
        transaction_position as f64,
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

// 平仓
#[tauri::command]
pub fn handle_close_position(stock_id: i32, current_price: f64) -> Result<(), String> {
    println!("close_stock:{stock_id},{current_price}");
    let stock = StockHandler::get_stock_by_id(stock_id)
        .map_err(|e| e.to_string())?
        .ok_or("Stock not found")?;
    let last_action = StockActionHandler::get_last_action(stock_id).map_err(|e| e.to_string())?;
    //
    let action_type = ActionType::Close as i32;
    // 本次各项费用
    let fee_rate = FeeRates::for_stock_type(StockType::from(last_action.action), ActionType::Close);
    let transaction_commission_fee =
        current_price * last_action.total_position as f64 * stock.commission_fee_rate;
    let transaction_tax_fee = current_price * last_action.total_position as f64 * fee_rate.tax;
    let transaction_regulatory_fee =
        current_price * last_action.total_position as f64 * fee_rate.regulatory;
    let transaction_brokerage_fee =
        current_price * last_action.total_position as f64 * fee_rate.brokerage;
    let transaction_transfer_fee =
        current_price * last_action.total_position as f64 * fee_rate.transfer;
    let total_fee = last_action.total_fee
        + transaction_commission_fee
        + transaction_tax_fee
        + transaction_regulatory_fee
        + transaction_brokerage_fee
        + transaction_transfer_fee;
    // 成本价
    let current_cost = 0.0;
    // 总持仓变成0
    let total_position = 0.0;
    // 利润
    let profit = (current_price - last_action.current_cost) * last_action.total_position;
    let profit_rate =
        calculate_safe_profit_rate(profit, last_action.current_cost, last_action.total_position);

    // 插入操作记录
    StockActionHandler::insert_action(
        stock_id,
        current_price,
        current_cost,
        total_position,
        total_fee,
        current_price,
        last_action.total_position,
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

// 回退
#[tauri::command]
pub fn handle_back_position(stock_id: i32) -> Result<(), String> {
    StockActionHandler::delete_last_action(stock_id).map_err(|e| e.to_string())?;
    StockHandler::update_stock_status(stock_id, StockStatus::OPEN as i32)
        .map_err(|e| e.to_string())?;
    Ok(())
}
