use crate::database::stock::StockHandler as DbStockHandler;
use crate::database::stock_action::StockActionHandler as DbStockActionHandler;
use crate::{StockItem, MainWindow};
use rusqlite::Result;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use rusqlite::Connection;
use slint::ComponentHandle;
use rand::Rng;

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
            println!("  类型: {}", match stock.stock_type {
                1 => "沪市",
                2 => "深市", 
                3 => "创业板",
                4 => "科创板",
                _ => "未知"
            });
            println!("  费率: {}%", stock.commission_fee_rate * 100.0);
            println!("  创建时间: {}", stock.created_at);
            
            // 查询交易记录
            match DbStockActionHandler::get_actions_by_stock_id(&db_conn, stock_id) {
                Ok(actions) => {
                    println!("  交易记录: {} 条", actions.len());
                    for action in actions.iter().take(5) { // 只显示前5条
                        println!("    - 价格: {}, 数量: {}, 操作: {}", 
                            action.price, action.amount, 
                            match action.action {
                                1 => "建仓",
                                2 => "平仓", 
                                3 => "加仓",
                                4 => "减仓",
                                _ => "未知"
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

/// UI处理器 - 处理新建股票操作
pub fn handle_create_stock(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow) {
    println!("创建新股票...");
    
    // 创建一个示例股票
    let mut rng = rand::thread_rng();
    let stock_names = ["招商银行", "中国平安", "五粮液", "贵州茅台", "腾讯控股", "阿里巴巴"];
    let random_name = stock_names[rng.gen_range(0..stock_names.len())];
    let random_type = rng.gen_range(1..=4);
    let random_rate = rng.gen_range(0.0001..0.001); // 0.01% - 0.1%
    
    match DbStockHandler::insert_stock(&db_conn, random_name, random_type, random_rate) {
        Ok(stock_id) => {
            println!("新建股票成功! ID: {}, 名称: {}", stock_id, random_name);
            // 重新加载列表
            if let Err(e) = load_stock_list(&db_conn, ui) {
                eprintln!("重新加载股票列表失败: {}", e);
            }
        }
        Err(e) => {
            eprintln!("新建股票失败: {}", e);
        }
    }
}

