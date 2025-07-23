use crate::constant::{ActionType, StockType};
use crate::database::stock_action::StockActionHandler as DbStockActionHandler;
use crate::{MainWindow, StockActionItem};
use rusqlite::Connection;
use rusqlite::Result;
use slint::ComponentHandle;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn handle_add_position(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow) {}

pub fn handle_reduce_position(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow) {}

pub fn handle_back_position(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow, stock_id: i32) {
    match DbStockActionHandler::delete_last_action(db_conn, stock_id) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("删除最后一条操作记录失败: {}", e);
        }
    }
}

pub fn handle_close_position(db_conn: &Arc<Mutex<Connection>>, ui: &MainWindow) {}
