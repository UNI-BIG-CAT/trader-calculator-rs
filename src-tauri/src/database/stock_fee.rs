use crate::database::db_connect::get_db_state;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StockFeeRate {
    pub stock_fee_id: i32,
    pub stock_fee_name: String,
    pub commission_fee_rate: f64, // 佣金
    pub tax_fee_rate: f64,        // 印花税
    pub regulatory_fee_rate: f64, // 证管费
    pub brokerage_fee_rate: f64,  // 经手费
    pub transfer_fee_rate: f64,   // 过户费
    pub created_at: String,
    pub updated_at: String,
}

#[allow(dead_code)]
impl StockFeeRate {
    pub fn get_fee() -> Result<StockFeeRate, rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT stock_fee_id, stock_fee_name,  commission_fee_rate, tax_fee_rate, regulatory_fee_rate, brokerage_fee_rate, transfer_fee_rate,created_at, updated_at FROM tb_stock_fee WHERE stock_fee_id = 1;"
        )?;

        let stock_fee = stmt.query_row([], |row| {
            Ok(StockFeeRate {
                stock_fee_id: row.get(0)?,
                stock_fee_name: row.get(1)?,
                commission_fee_rate: row.get(2)?,
                tax_fee_rate: row.get(3)?,
                regulatory_fee_rate: row.get(4)?,
                brokerage_fee_rate: row.get(5)?,
                transfer_fee_rate: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        Ok(stock_fee)
    }
    pub fn insert(
        stock_fee_name: &str,
        commission_fee_rate: f64,
        tax_fee_rate: f64,
        regulatory_fee_rate: f64,
        brokerage_fee_rate: f64,
        transfer_fee_rate: f64,
    ) -> Result<i64, rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tb_stock_fee (stock_fee_name, commission_fee_rate, tax_fee_rate, regulatory_fee_rate, brokerage_fee_rate, transfer_fee_rate) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                stock_fee_name,
                &commission_fee_rate.to_string(),
                &tax_fee_rate.to_string(),
                &regulatory_fee_rate.to_string(),
                &brokerage_fee_rate.to_string(),
                &transfer_fee_rate.to_string(),
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }
    pub fn update(
        commission_fee_rate: f64,
        tax_fee_rate: f64,
        regulatory_fee_rate: f64,
        brokerage_fee_rate: f64,
        transfer_fee_rate: f64,
    ) -> Result<(), rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "UPDATE tb_stock_fee SET commission_fee_rate = ?1, tax_fee_rate = ?2, regulatory_fee_rate = ?3, brokerage_fee_rate = ?4, transfer_fee_rate = ?5 WHERE stock_fee_id = 1",
            [
                &commission_fee_rate.to_string(),
                &tax_fee_rate.to_string(),
                &regulatory_fee_rate.to_string(),
                &brokerage_fee_rate.to_string(),
                &transfer_fee_rate.to_string(),
            ],
        )?;
        Ok(())
    }
    pub fn delete(stock_fee_id: i32) -> Result<(), rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "DELETE FROM tb_stock_fee WHERE stock_fee_id = ?1",
            [stock_fee_id.to_string()],
        )?;
        Ok(())
    }
}
