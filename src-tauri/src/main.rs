// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
use crate::database::db_connect::get_db_state;

fn main() {
    // 初始化数据库
    let db_conn = get_db_state();
    // 启动Tauri应用
    tauri_app_lib::run()
}
