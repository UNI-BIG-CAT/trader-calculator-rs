slint::include_modules!();

mod database;
mod handler;
mod constant;

use database::DatabaseState;
use handler::stock::{load_stock_list, handle_view_stock, handle_delete_stock, handle_create_stock};
use std::rc::Rc;
use slint::ComponentHandle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /**********************数据库**************************/
    // 初始化数据库
    let db_state = Rc::new(DatabaseState::new("trader_calculator.db")?);
    println!("数据库初始化成功！");
    // 获取共享的数据库连接
    let db_conn = db_state.get_connection();

    /**********************UI**************************/
    let ui = MainWindow::new()?;
    // 初始化加载股票列表
    if let Err(e) = load_stock_list(&db_conn, &ui) {
        eprintln!("初始化加载股票列表失败: {}", e);
    }
    
    /**********************回调**************************/
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
    
    // 设置建仓对话框回调
    {
        let db_conn = db_conn.clone();
        let ui_weak = ui.as_weak();
        ui.on_create_stock_with_data(move |data| {
            if let Some(ui) = ui_weak.upgrade() {
                handle_create_stock(&db_conn, &ui, data);
            }
        });
    }
    
    // 启动UI
    ui.run()?;
    Ok(())
}
