// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod constant;
mod database;
mod handler;

fn main() {
    // 初始化数据库
    let db_conn = database::db_connect::init_db();
    // 启动Tauri应用
    tauri_app_lib::run()
}
