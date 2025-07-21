use crate::database::StockActionRecord;
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub struct StockActionHandler;

impl StockActionHandler {
    // pub fn new() -> Self {
    //     Self
    // }

    /// 插入股票操作数据
    pub fn insert_action(
        db_conn: &Arc<Mutex<Connection>>, 
        stock_id: i32, 
        price: f64, 
        amount: f64, 
        commission_fee: f64, 
        action: i32, 
        current_cost: f64
    ) -> Result<i64> {
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tb_stock_action (stock_id, price, amount, commission_fee, action, current_cost) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &stock_id.to_string(), 
                &price.to_string(), 
                &amount.to_string(), 
                &commission_fee.to_string(), 
                &action.to_string(), 
                &current_cost.to_string()
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }
    /// 根据股票ID查询操作记录
    pub fn get_actions_by_stock_id(db_conn: &Arc<Mutex<Connection>>, stock_id: i32) -> Result<Vec<StockActionRecord>> {
        let conn = db_conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT stock_action_id, stock_id, price, amount, commission_fee, action, current_cost, created_at, updated_at FROM tb_stock_action WHERE stock_id = ? ORDER BY stock_action_id DESC"
        )?;

        let stock_action_iter = stmt.query_map([stock_id], |row| {
            Ok(StockActionRecord {
                stock_action_id: row.get(0)?,
                stock_id: row.get(1)?,
                price: row.get(2)?,
                amount: row.get(3)?,
                commission_fee: row.get(4)?,
                action: row.get(5)?,
                current_cost: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        let mut stock_actions = Vec::new();
        for stock_action in stock_action_iter {
            stock_actions.push(stock_action?);
        }
        Ok(stock_actions)
    }

    /// 删除最后一条操作记录
    pub fn delete_last_action(db_conn: &Arc<Mutex<Connection>>) -> Result<()> {
        let conn = db_conn.lock().unwrap();
        conn.execute("DELETE FROM tb_stock_action WHERE stock_action_id = (SELECT MAX(stock_action_id) FROM tb_stock_action)", [])?;
        Ok(())
    }
}