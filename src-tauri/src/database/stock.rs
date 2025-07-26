use crate::database::db_connect::get_db_state;
use rusqlite::Result;
use serde::{Deserialize, Serialize};

// Stock结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockRecord {
    pub stock_id: i32,
    pub stock_name: String,
    pub stock_type: i32,
    pub commission_fee_rate: f64, // 佣金
    pub tax_fee_rate: f64,        // 印花税
    pub regulatory_fee_rate: f64, // 证管费
    pub brokerage_fee_rate: f64,  // 经手费
    pub transfer_fee_rate: f64,   // 过户费
    pub status: i32,              // 状态 1-正常买卖中 2-已经平仓
    pub created_at: String,
    pub updated_at: String,
}

// 操作记录处理
#[allow(dead_code)]
pub struct StockHandler;
impl StockHandler {
    /// 插入股票数据
    #[allow(dead_code)]
    pub fn insert_stock(
        stock_name: &str,
        stock_type: i32,
        commission_fee_rate: f64,
        tax_fee_rate: f64,
        regulatory_fee_rate: f64,
        brokerage_fee_rate: f64,
        transfer_fee_rate: f64,
    ) -> Result<i64, rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tb_stock (stock_name, type, commission_fee_rate, tax_fee_rate, regulatory_fee_rate, brokerage_fee_rate, transfer_fee_rate) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            [
                stock_name,
                &stock_type.to_string(),
                &commission_fee_rate.to_string(),
                &tax_fee_rate.to_string(),
                &regulatory_fee_rate.to_string(),
                &brokerage_fee_rate.to_string(),
                &transfer_fee_rate.to_string(),
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// 查询所有股票数据
    #[allow(dead_code)]
    pub fn get_all_stocks() -> Result<Vec<StockRecord>> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT stock_id, stock_name, type, commission_fee_rate, tax_fee_rate, regulatory_fee_rate, brokerage_fee_rate, transfer_fee_rate, status, created_at, updated_at FROM tb_stock ORDER BY stock_id DESC"
        )?;

        let stock_iter = stmt.query_map([], |row| {
            Ok(StockRecord {
                stock_id: row.get(0)?,
                stock_name: row.get(1)?,
                stock_type: row.get(2)?,
                commission_fee_rate: row.get(3)?,
                tax_fee_rate: row.get(4)?,
                regulatory_fee_rate: row.get(5)?,
                brokerage_fee_rate: row.get(6)?,
                transfer_fee_rate: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;

        let mut stocks = Vec::new();
        for stock in stock_iter {
            stocks.push(stock?);
        }
        Ok(stocks)
    }

    /// 根据ID查询股票
    #[allow(dead_code)]
    pub fn get_stock_by_id(stock_id: i32) -> Result<Option<StockRecord>> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT stock_id, stock_name, type, commission_fee_rate, tax_fee_rate, regulatory_fee_rate, brokerage_fee_rate, transfer_fee_rate, status, created_at, updated_at FROM tb_stock WHERE stock_id = ?"
        )?;

        let mut rows = stmt.query_map([stock_id], |row| {
            Ok(StockRecord {
                stock_id: row.get(0)?,
                stock_name: row.get(1)?,
                stock_type: row.get(2)?,
                commission_fee_rate: row.get(3)?,
                tax_fee_rate: row.get(4)?,
                regulatory_fee_rate: row.get(5)?,
                brokerage_fee_rate: row.get(6)?,
                transfer_fee_rate: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;

        match rows.next() {
            Some(stock) => Ok(Some(stock?)),
            None => Ok(None),
        }
    }

    // 修改股票状态
    #[allow(dead_code)]
    pub fn update_stock_status(stock_id: i32, status: i32) -> Result<()> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "UPDATE tb_stock SET status = ? WHERE stock_id = ?",
            [status, stock_id],
        )?;
        Ok(())
    }

    /// 删除股票数据
    #[allow(dead_code)]
    pub fn delete_stock(stock_id: i32) -> Result<()> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute("DELETE FROM tb_stock WHERE stock_id = ?", [stock_id])?;
        conn.execute("DELETE FROM tb_stock_action WHERE stock_id = ?", [stock_id])?;
        Ok(())
    }

    //
}
