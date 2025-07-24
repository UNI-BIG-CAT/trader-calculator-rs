use crate::database::db_connect::get_db_state;
use rusqlite::Result;
use serde::{Deserialize, Serialize};

// Stock结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockRecord {
    pub stock_id: i32,
    pub stock_name: String,
    pub stock_type: i32,
    pub commission_fee_rate: f64,
    pub created_at: String,
    pub updated_at: String,
}

// 操作记录处理
pub struct StockHandler;
impl StockHandler {
    /// 插入股票数据
    pub fn insert_stock(
        stock_name: &str,
        stock_type: i32,
        commission_fee_rate: f64,
    ) -> Result<i64, rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tb_stock (stock_name, type, commission_fee_rate) VALUES (?1, ?2, ?3)",
            [
                stock_name,
                &stock_type.to_string(),
                &commission_fee_rate.to_string(),
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// 查询所有股票数据
    pub fn get_all_stocks() -> Result<Vec<StockRecord>> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT stock_id, stock_name, type, commission_fee_rate, created_at, updated_at FROM tb_stock ORDER BY stock_id DESC"
        )?;

        let stock_iter = stmt.query_map([], |row| {
            Ok(StockRecord {
                stock_id: row.get(0)?,
                stock_name: row.get(1)?,
                stock_type: row.get(2)?,
                commission_fee_rate: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut stocks = Vec::new();
        for stock in stock_iter {
            stocks.push(stock?);
        }
        Ok(stocks)
    }

    /// 根据ID查询股票
    pub fn get_stock_by_id(stock_id: i32) -> Result<Option<StockRecord>> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT stock_id, stock_name, type, commission_fee_rate, created_at, updated_at FROM tb_stock WHERE stock_id = ?"
        )?;

        let mut rows = stmt.query_map([stock_id], |row| {
            Ok(StockRecord {
                stock_id: row.get(0)?,
                stock_name: row.get(1)?,
                stock_type: row.get(2)?,
                commission_fee_rate: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        match rows.next() {
            Some(stock) => Ok(Some(stock?)),
            None => Ok(None),
        }
    }

    /// 删除股票数据
    pub fn delete_stock(stock_id: i32) -> Result<()> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute("DELETE FROM tb_stock WHERE stock_id = ?", [stock_id])?;
        conn.execute("DELETE FROM tb_stock_action WHERE stock_id = ?", [stock_id])?;
        Ok(())
    }

    //
}
