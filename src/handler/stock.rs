use crate::constant::ActionType;
use crate::database::stock::StockHandler as DbStockHandler;
use crate::database::stock_action::StockActionHandler as DbStockActionHandler;
use crate::{MainWindow, OpenStockData, StockActionItem, StockItem};
use rusqlite::Connection;
use rusqlite::Result;
use slint::ComponentHandle;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// UI处理器 - 负责股票列表显示
pub fn load_stock_list(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow) -> Result<()> {
    let ui_weak = ui.as_weak();
    if let Some(ui) = ui_weak.upgrade() {
        match DbStockHandler::get_all_stocks(&db_conn) {
            Ok(stocks) => {
                let stock_items: Vec<StockItem> = stocks
                    .into_iter()
                    .map(|stock| StockItem {
                        stock_id: stock.stock_id,
                        stock_name: stock.stock_name.into(),
                        stock_type: stock.stock_type,
                        commission_fee_rate: stock.commission_fee_rate as f32,
                        created_at: stock.created_at.into(),
                    })
                    .collect();

                let model_rc = Rc::new(slint::VecModel::from(stock_items));
                ui.set_stock_list(model_rc.into());
            }
            Err(e) => {
                eprintln!("加载股票列表失败: {}", e);
            }
        }
    }
    Ok(())
}

/// UI处理器 - 处理股票查看操作
pub fn handle_view_stock(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow, stock_id: i32) {
    // 查询股票信息
    match DbStockHandler::get_stock_by_id(&db_conn, stock_id) {
        Ok(Some(stock)) => {
            // 设置选中的股票信息
            let stock_item = StockItem {
                stock_id: stock.stock_id,
                stock_name: stock.stock_name.clone().into(),
                stock_type: stock.stock_type,
                commission_fee_rate: stock.commission_fee_rate as f32,
                created_at: stock.created_at.clone().into(),
            };
            ui.set_selected_stock(stock_item);

            // 查询交易记录
            match DbStockActionHandler::get_actions_by_stock_id(&db_conn, stock_id) {
                Ok(actions) => {
                    // 转换为UI数据
                    let action_items: Vec<StockActionItem> = actions
                        .into_iter()
                        .map(|action| StockActionItem {
                            stock_action_id: action.stock_action_id,
                            stock_id: action.stock_id,
                            current_price: action.current_price as f32,
                            current_cost: action.current_cost as f32,
                            total_amount: action.total_amount as f32,
                            transaction_price: action.transaction_price as f32,
                            transaction_amount: action.transaction_amount as f32,
                            transaction_commission_fee: action.transaction_commission_fee as f32,
                            action: action.action,
                            profit: action.profit as f32,
                            profit_rate: action.profit_rate as f32,
                            created_at: action.created_at.into(),
                        })
                        .collect();

                    let model_rc = Rc::new(slint::VecModel::from(action_items));
                    ui.set_stock_actions(model_rc.into());

                    // 切换到详情页面
                    ui.set_show_detail(true);
                }
                Err(e) => eprintln!("查询交易记录失败: {}", e),
            }
        }
        Ok(None) => {
            eprintln!("未找到股票 ID: {}", stock_id);
        }
        Err(e) => {
            eprintln!("查询股票失败: {}", e);
        }
    }
}

/// UI处理器 - 处理股票删除操作
pub fn handle_delete_stock(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow, stock_id: i32) {
    match DbStockHandler::delete_stock(&db_conn, stock_id) {
        Ok(_) => {
            // 重新加载列表
            if let Err(e) = load_stock_list(&db_conn, ui) {
                eprintln!("重新加载股票列表失败: {}", e);
            }
        }
        Err(e) => {
            eprintln!("删除股票失败: {}", e);
        }
    }
}
/// UI处理器 - 处理建仓对话框数据
pub fn handle_create_stock(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow, data: OpenStockData) {
    // 计算默认佣金费率
    let commission_fee_rate = match data.stock_type {
        1 => 0.0003, // 沪市默认万3
        2 => 0.0003, // 深市默认万3
        _ => 0.0003,
    };

    // 1. 先创建股票记录
    match DbStockHandler::insert_stock(
        &db_conn,
        &data.stock_name,
        data.stock_type,
        commission_fee_rate,
    ) {
        Ok(stock_id) => {
            // 2. 计算建仓成本和佣金
            let total_amount = (data.transaction_price * data.transaction_amount) as f64; // 总金额
            let commission_fee = total_amount * commission_fee_rate; // 佣金
            let total_cost = total_amount + commission_fee; // 总成本
            let cost_per_share = total_cost / data.transaction_amount as f64; // 每股成本

            // 3. 创建建仓交易记录
            match DbStockActionHandler::insert_action(
                &db_conn,
                stock_id as i32,
                data.current_price as f64,
                total_cost,
                data.transaction_amount as f64,
                data.transaction_price as f64,
                data.transaction_amount as f64,
                commission_fee,
                ActionType::Open as i32, // action = 1 表示建仓
                cost_per_share,
                cost_per_share,
            ) {
                Ok(_action_id) => {
                    // 重新加载列表
                    if let Err(e) = load_stock_list(&db_conn, ui) {
                        eprintln!("重新加载股票列表失败: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("创建建仓记录失败: {}", e);
                    // 建仓记录创建失败，删除刚才创建的股票记录
                    if let Err(delete_err) = DbStockHandler::delete_stock(&db_conn, stock_id as i32)
                    {
                        eprintln!("回滚删除股票记录也失败: {}", delete_err);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("创建股票记录失败: {}", e);
        }
    }
}
