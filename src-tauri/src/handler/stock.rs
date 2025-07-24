use crate::constant::ActionType;
use crate::database::stock_action::StockActionHandler as DbStockActionHandler;
use rusqlite::Connection;
use rusqlite::Result;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// Tauri专用的简化函数
/// 获取所有股票 - 适配Tauri
pub fn get_all_stocks() {}

/// 创建股票 - 适配Tauri
pub fn open_stock() {}

/// 负责股票列表显示
pub fn load_stock_list() {}

/// 股票查看
pub fn handle_view_stock(stock_id: i32) {}

/// 股票删除
pub fn handle_delete_stock(stock_id: i32) {}
