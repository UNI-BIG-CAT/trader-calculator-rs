use crate::database::db_connect::get_db_state;
use serde::Serialize;

// 数据结构定义
#[derive(Debug, Clone, Serialize)]
pub struct StockActionRecord {
    pub stock_action_id: i32,
    pub stock_id: i32,
    pub current_price: f64,
    pub current_cost: f64,
    pub total_position: f64,
    pub total_fee:f64,
    pub transaction_price: f64,
    pub transaction_position: f64,
    pub transaction_commission_fee: f64, // 佣金
    pub transaction_tax_fee:f64, // 印花税
    pub transaction_regulatory_fee:f64, // 证管费
    pub transaction_brokerage_fee:f64, // 经手费
    pub transaction_transfer_fee:f64, // 过户费
    pub action: i32,
    pub profit: f64,
    pub profit_rate: f64,
    pub action_time: String,
    pub action_info: String,
    pub created_at: String,
    pub updated_at: String,
}

// 操作记录处理
    #[allow(dead_code)]
    impl StockActionRecord {
    /// 插入股票操作数据
    pub fn insert_action(
        stock_id: i32, 
        current_price: f64, 
        current_cost: f64, 
        total_position: f64, 
        total_fee:f64,
        transaction_price: f64, 
        transaction_position: f64, 
        transaction_commission_fee: f64, 
        transaction_tax_fee: f64,
        transaction_regulatory_fee: f64,
        transaction_brokerage_fee: f64,
        transaction_transfer_fee: f64,
        action: i32, 
        profit: f64, 
        profit_rate: f64, 
    ) -> Result<i64, rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tb_stock_action (stock_id, current_price, current_cost, total_position, total_fee, transaction_price, transaction_position, transaction_commission_fee, transaction_tax_fee, transaction_regulatory_fee, transaction_brokerage_fee, transaction_transfer_fee, action, profit, profit_rate) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            [
                &stock_id.to_string(), 
                &current_price.to_string(), 
                &current_cost.to_string(), 
                &total_position.to_string(), 
                &total_fee.to_string(),
                &transaction_price.to_string(), 
                &transaction_position.to_string(), 
                &transaction_commission_fee.to_string(), 
                &transaction_tax_fee.to_string(),
                &transaction_regulatory_fee.to_string(),
                &transaction_brokerage_fee.to_string(),
                &transaction_transfer_fee.to_string(),
                &action.to_string(), 
                &profit.to_string(), 
                &profit_rate.to_string()
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }
    /// 根据股票ID查询操作记录
    pub fn get_actions_by_stock_id( stock_id: i32) -> Result<Vec<StockActionRecord>,rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT stock_action_id, stock_id, current_price, current_cost, total_position, total_fee, transaction_price, transaction_position, transaction_commission_fee, transaction_tax_fee, transaction_regulatory_fee, transaction_brokerage_fee, transaction_transfer_fee, action, profit, profit_rate,action_time,action_info, created_at, updated_at FROM tb_stock_action WHERE stock_id = ? ORDER BY stock_action_id ASC"
        )?;

        let stock_action_iter = stmt.query_map([stock_id], |row| {
            Ok(StockActionRecord {
                stock_action_id: row.get(0)?,
                stock_id: row.get(1)?,
                current_price: row.get(2)?,
                current_cost: row.get(3)?,
                total_position: row.get(4)?,
                total_fee: row.get(5)?,
                transaction_price: row.get(6)?,
                transaction_position: row.get(7)?,
                transaction_commission_fee: row.get(8)?,
                transaction_tax_fee: row.get(9)?,
                transaction_regulatory_fee: row.get(10)?,
                transaction_brokerage_fee: row.get(11)?,
                transaction_transfer_fee: row.get(12)?,
                action: row.get(13)?,
                profit: row.get(14)?,
                profit_rate: row.get(15)?,
                action_time: row.get(16)?,
                action_info: row.get(17)?,
                created_at: row.get(18)?,
                updated_at: row.get(19)?,
            })
        })?;

        let mut stock_actions = Vec::new();
        for stock_action in stock_action_iter {
            stock_actions.push(stock_action?);
        }
        Ok(stock_actions)
    }
    
    // 获取最后一次操作
    pub fn get_last_action(stock_id:i32) -> Result<StockActionRecord,rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT stock_action_id, stock_id, current_price, current_cost, total_position,total_fee, transaction_price, transaction_position, transaction_commission_fee, transaction_tax_fee, transaction_regulatory_fee, transaction_brokerage_fee, transaction_transfer_fee, action, profit, profit_rate,action_time,action_info, created_at, updated_at FROM tb_stock_action WHERE stock_id = ? ORDER BY stock_action_id DESC LIMIT 1")?;
        let stock_action = stmt.query_row([stock_id], |row| {
            Ok(StockActionRecord {
                stock_action_id: row.get(0)?,   
                stock_id: row.get(1)?,
                current_price: row.get(2)?,
                current_cost: row.get(3)?,
                total_position: row.get(4)?,
                total_fee: row.get(5)?,
                transaction_price: row.get(6)?,
                transaction_position: row.get(7)?,
                transaction_commission_fee: row.get(8)?,
                transaction_tax_fee: row.get(9)?,
                transaction_regulatory_fee: row.get(10)?,
                transaction_brokerage_fee: row.get(11)?,
                transaction_transfer_fee: row.get(12)?,
                action: row.get(13)?,
                profit: row.get(14)?,
                profit_rate: row.get(15)?,
                action_time: row.get(16)?,
                action_info: row.get(17)?,
                created_at: row.get(18)?,
                updated_at: row.get(19)?,
            })
        })?;
        Ok(stock_action)
    }   

    /// 保存操作信息
    pub fn save_stock_action_info(stock_action_id:i32,action_time:String,action_info:String) -> Result<(),rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute("UPDATE tb_stock_action SET action_time = ?, action_info = ? WHERE stock_action_id = ?", [action_time,action_info,stock_action_id.to_string()])?;
        Ok(())
    }
    
    /// 删除最后一条操作记录
    pub fn delete_last_action(stock_id:i32) -> Result<(),rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute("DELETE FROM tb_stock_action WHERE stock_action_id = (SELECT MAX(stock_action_id) FROM tb_stock_action WHERE stock_id = ?)", [stock_id])?;
        Ok(())
    }
}
