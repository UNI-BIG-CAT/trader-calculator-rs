use crate::database::{StockRecord};
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub struct StockHandler;

impl StockHandler {
    // pub fn new() -> Self {
    //     Self
    // }

    /// 插入股票数据
    pub fn insert_stock(
        db_conn: &Arc<Mutex<Connection>>, 
        stock_name: &str, 
        stock_type: i32,
        commission_fee_rate: f64
    ) -> Result<i64> {
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tb_stock (stock_name, type, commission_fee_rate) VALUES (?1, ?2, ?3)",
            [stock_name, &stock_type.to_string(), &commission_fee_rate.to_string()],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// 查询所有股票数据
    pub fn get_all_stocks(db_conn: &Arc<Mutex<Connection>>) -> Result<Vec<StockRecord>> {
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
    pub fn get_stock_by_id(db_conn: &Arc<Mutex<Connection>>, stock_id: i32) -> Result<Option<StockRecord>> {
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
    pub fn delete_stock(db_conn: &Arc<Mutex<Connection>>, stock_id: i32) -> Result<()> {
        let conn = db_conn.lock().unwrap();
        conn.execute("DELETE FROM tb_stock WHERE stock_id = ?", [stock_id])?;
        conn.execute("DELETE FROM tb_stock_action WHERE stock_id = ?", [stock_id])?;
        Ok(())
    }

    //
}