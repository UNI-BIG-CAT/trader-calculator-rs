slint::include_modules!();

mod database;
mod handler;

use database::DatabaseState;
use handler::stock::{load_stock_list, handle_view_stock, handle_delete_stock, handle_create_stock};
use std::rc::Rc;
use slint::ComponentHandle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用状态（包含共享的数据库连接）
    let db_state = Rc::new(DatabaseState::new("trader_calculator.db")?);
    println!("数据库初始化成功！");

    // 获取共享的数据库连接
    let db_conn = db_state.get_connection();
    
    // 创建UI
    let ui = MainWindow::new()?;
    
    // 初始化加载股票列表
    if let Err(e) = load_stock_list(&db_conn, &ui) {
        eprintln!("初始化加载股票列表失败: {}", e);
    }
    
    // 设置UI回调函数
    {
        let db_conn = db_conn.clone();
        let ui_weak = ui.as_weak();
        ui.on_refresh_stock_list(move || {
            if let Some(ui) = ui_weak.upgrade() {
                if let Err(e) = load_stock_list(&db_conn, &ui) {
                    eprintln!("刷新股票列表失败: {}", e);
                }
            }
        });
    }
    
    // 设置查看股票详情回调
    {
        let db_conn = db_conn.clone();
        ui.on_view_stock_details(move |stock_id| {
            handle_view_stock(&db_conn, stock_id);
        });
    }
    
    // 设置删除股票回调
    {
        let db_conn = db_conn.clone();
        let ui_weak = ui.as_weak();
        ui.on_delete_stock_item(move |stock_id| {
            if let Some(ui) = ui_weak.upgrade() {
                handle_delete_stock(&db_conn, &ui, stock_id);
            }
        });
    }
    
    // 设置新建股票回调
    {
        let db_conn = db_conn.clone();
        let ui_weak = ui.as_weak();
        ui.on_create_new_stock(move || {
            if let Some(ui) = ui_weak.upgrade() {
                handle_create_stock(&db_conn, &ui);
            }
        });
    }
    
    // 启动UI
    ui.run()?;
    Ok(())
}
