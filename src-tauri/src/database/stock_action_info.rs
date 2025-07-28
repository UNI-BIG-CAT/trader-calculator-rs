use crate::database::db_connect::get_db_state;
use rusqlite::Result;
use serde::{Deserialize, Serialize};

// Stock结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockActionInfoRecord {
    pub stock_action_info_id: i32,
    pub stock_action_id: i32,
    pub action_time: String,
    pub action_info: String,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(dead_code)]
impl StockActionInfoRecord {
    pub fn get_action_info_by_action_id(
        action_id: i32,
    ) -> Result<Vec<StockActionInfoRecord>, rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT * FROM tb_stock_action_info WHERE stock_action_id = ?")?;
        let action_infos = stmt.query_map([action_id], |row| {
            Ok(StockActionInfoRecord {
                stock_action_info_id: row.get(0)?,
                stock_action_id: row.get(1)?,
                action_time: row.get(2)?,
                action_info: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        let mut action_infos_vec = Vec::new();
        for action_info in action_infos {
            action_infos_vec.push(action_info?);
        }
        Ok(action_infos_vec)
    }
    //
    pub fn save_stock_action_info(
        stock_action_info: StockActionInfoRecord,
    ) -> Result<i64, rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let action_info =
            StockActionInfoRecord::get_action_info_by_action_id(stock_action_info.stock_action_id)
                .map_err(|e| e.to_string())
                .unwrap();
        if action_info.is_empty() {
            conn.execute(
                "INSERT INTO tb_stock_action_info (stock_action_id, action_time, action_info) VALUES (?, ?, ?)",
                (stock_action_info.stock_action_id, stock_action_info.action_time, stock_action_info.action_info),
            )?;
        } else {
            conn.execute(
                "UPDATE tb_stock_action_info SET action_time = ?, action_info = ? WHERE stock_action_id = ?",
                (stock_action_info.action_time, stock_action_info.action_info, stock_action_info.stock_action_id),
            )?;
        }
        Ok(conn.last_insert_rowid())
    }
}
