use crate::database::stock::StockHandler as DbStockHandler;
use crate::database::stock_action::StockActionHandler as DbStockActionHandler;
use crate::{StockItem, MainWindow, OpenStockData};
use rusqlite::Result;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use rusqlite::Connection;
use slint::ComponentHandle;
use crate::constant::{StockType, ActionType};


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
                
                let count = stock_items.len();
                let model_rc = Rc::new(slint::VecModel::from(stock_items));
                ui.set_stock_list(model_rc.into());
                println!("已加载 {} 只股票", count);
            }
            Err(e) => {
                eprintln!("加载股票列表失败: {}", e);
            }
        }
    }
    Ok(())
}

/// UI处理器 - 处理股票查看操作
pub fn handle_view_stock(db_conn: &Arc<Mutex<Connection>>, stock_id: i32) {
    println!("查看股票详情: ID = {}", stock_id);
    
    // 查询股票信息
    match DbStockHandler::get_stock_by_id(&db_conn, stock_id) {
        Ok(Some(stock)) => {
            println!("股票信息:");
            println!("  ID: {}", stock.stock_id);
            println!("  名称: {}", stock.stock_name);
            println!("  类型: {}", match StockType::from(stock.stock_type) {
                StockType::SH => "沪市",
                StockType::SZ => "深市", 
                StockType::CYB => "创业板",
                StockType::KCB => "科创板",
            });
            println!("  费率: {}%", stock.commission_fee_rate * 100.0);
            println!("  创建时间: {}", stock.created_at);
            
            // 查询交易记录
            match DbStockActionHandler::get_actions_by_stock_id(&db_conn, stock_id) {
                Ok(actions) => {
                    println!("  交易记录: {} 条", actions.len());
                    for action in actions.iter().take(5) { // 只显示前5条
                        println!("    - 价格: {}, 数量: {}, 操作: {}", 
                            action.current_price, action.total_amount, 
                            match ActionType::from(action.action) {
                                ActionType::Open => "建仓",
                                ActionType::Close => "平仓", 
                                ActionType::AddPosition => "加仓",
                                ActionType::ReducePosition => "减仓",
                            });
                    }
                }
                Err(e) => eprintln!("查询交易记录失败: {}", e),
            }
        }
        Ok(None) => {
            println!("未找到股票 ID: {}", stock_id);
        }
        Err(e) => {
            eprintln!("查询股票失败: {}", e);
        }
    }
}

/// UI处理器 - 处理股票删除操作
pub fn handle_delete_stock(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow, stock_id: i32) {
    println!("删除股票: ID = {}", stock_id);
    
    match DbStockHandler::delete_stock(&db_conn, stock_id) {
        Ok(_) => {
            println!("股票删除成功!");
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
pub fn handle_create_stock(
    db_conn: &Arc<Mutex<Connection>>, 
    ui: &MainWindow, 
    data: OpenStockData
) {
    println!("建仓股票: {}", data.stock_name);
    println!("  类型: {}", if data.stock_type == 1 { "沪市" } else { "深市" });
    println!("  当前价格: {} 元", data.current_price);
    println!("  建仓价格: {} 元", data.transaction_price);
    println!("  建仓数量: {} 股", data.transaction_amount);
    
    // 计算默认佣金费率
    let commission_fee_rate = match data.stock_type {
        1 => 0.0003, // 沪市默认万3
        2 => 0.0003, // 深市默认万3
        _ => 0.0003,
    };
    
    // 1. 先创建股票记录
    match DbStockHandler::insert_stock(&db_conn, &data.stock_name, data.stock_type, commission_fee_rate) {
        Ok(stock_id) => {
            println!("创建股票记录成功，ID: {}", stock_id);
            
            // 2. 计算建仓成本和佣金
            let total_amount = (data.transaction_price * data.transaction_amount) as f64;  // 总金额
            let commission_fee = total_amount * commission_fee_rate;  // 佣金
            let total_cost = total_amount + commission_fee;  // 总成本
            let cost_per_share = total_cost / data.transaction_amount as f64;  // 每股成本
            
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
                cost_per_share
            ) {
                Ok(action_id) => {
                    println!("创建建仓记录成功,ID: {}", action_id);
                    println!("建仓详情:");
                    println!("  总金额: {:.2} 元", total_amount);
                    println!("  佣金费: {:.2} 元", commission_fee);
                    println!("  总成本: {:.2} 元", total_cost);
                    println!("  成本价: {:.4} 元/股", cost_per_share);
                    
                    // 重新加载列表
                    if let Err(e) = load_stock_list(&db_conn, ui) {
                        eprintln!("重新加载股票列表失败: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("创建建仓记录失败: {}", e);
                    // 建仓记录创建失败，删除刚才创建的股票记录
                    if let Err(delete_err) = DbStockHandler::delete_stock(&db_conn, stock_id as i32) {
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

