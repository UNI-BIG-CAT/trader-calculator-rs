slint::include_modules!();

mod constant;
mod database;
mod handler;

use database::DatabaseState;
use handler::stock::{
    handle_create_stock, handle_delete_stock, handle_view_stock, load_stock_list,
};
use handler::stock_action::{
    handle_add_position, handle_back_position, handle_close_position, handle_reduce_position,
};
use slint::ComponentHandle;
use std::rc::Rc;

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
        let ui_weak = ui.as_weak();
        ui.on_view_stock_details(move |stock_id| {
            if let Some(ui) = ui_weak.upgrade() {
                handle_view_stock(&db_conn, &ui, stock_id);
            }
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

    {
        let ui_weak = ui.as_weak();
        ui.on_add_position(move || {
            // if let Some(ui) = ui_weak.upgrade() {
            //     handle_add_position(&db_conn, &ui);
            // }
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_reduce_position(move || {
            // if let Some(ui) = ui_weak.upgrade() {
            //     handle_add_position(&db_conn, &ui);
            // }
        });
    }

    {
        let db_conn = db_conn.clone();
        let ui_weak = ui.as_weak();
        ui.on_back_position(move |stock_id| {
            if let Some(ui) = ui_weak.upgrade() {
                handle_back_position(&db_conn, &ui, stock_id);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_close_position(move || {
            // if let Some(ui) = ui_weak.upgrade() {
            //     handle_add_position(&db_conn, &ui);
            // }
        });
    }
    // 启动UI
    ui.run()?;
    Ok(())
}
