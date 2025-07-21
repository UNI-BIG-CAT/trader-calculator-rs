slint::include_modules!();

mod database;
mod handler;

use database::DatabaseState;
use handler::stock::StockHandler;
use handler::stock_action::StockActionHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用状态（包含共享的数据库连接）
    let db_state = DatabaseState::new("trader_calculator.db")?;
    println!("数据库初始化成功！");

    // 获取共享的数据库连接
    let db_conn = db_state.get_connection();
    // // 演示如何使用共享的数据库连接
    // // 1. 使用StockHandler插入一些测试数据
    // let stock_id = StockHandler::insert_stock(&db_conn, "平安银行", 1, 0.0003)?;
    // println!("插入股票记录，ID: {}", stock_id);
    
    // let stock_id2 = StockHandler::insert_stock(&db_conn, "腾讯控股", 2, 0.0005)?;
    // println!("插入股票记录，ID: {}", stock_id2);
    
    // // 2. 使用StockActionHandler插入交易记录（使用相同的数据库连接）
    // let action_id = StockActionHandler::insert_action(&db_conn, stock_id as i32, 10.50, 1000.0, 3.15, 1, 10503.15)?;
    // println!("插入交易记录，ID: {}", action_id);
    
    // // 3. 查询数据（验证共享连接工作正常）
    // let stocks = StockHandler::get_all_stocks(&db_conn)?;
    // println!("当前股票列表：");
    // for stock in stocks {
    //     println!("  - {} (ID: {}, 类型: {}, 费率: {})", 
    //         stock.stock_name, stock.stock_id, stock.stock_type, stock.commission_fee_rate);
    // }
    // let actions = StockActionHandler::get_actions_by_stock_id(&db_conn, stock_id as i32)?;
    // println!("股票ID {} 的交易记录：", stock_id);
    // for action in actions {
    //     println!("  - 价格: {}, 数量: {}, 操作: {}, 成本: {}", 
    //         action.price, action.amount, action.action, action.current_cost);
    // }
    
    // 创建UI
    let ui = MainWindow::new()?;
    // 启动UI
    ui.run()?;
    Ok(())
}
