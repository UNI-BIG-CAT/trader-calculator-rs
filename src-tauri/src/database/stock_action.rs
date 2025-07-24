use crate::database::db_connect::get_db_state;

// 数据结构定义
#[derive(Debug, Clone)]
pub struct StockActionRecord {
    pub stock_action_id: i32,
    pub stock_id: i32,
    pub current_price: f64,
    pub current_cost: f64,
    pub total_amount: f64,
    pub transaction_price: f64,
    pub transaction_amount: f64,
    pub transaction_commission_fee: f64,
    pub action: i32,
    pub profit: f64,
    pub profit_rate: f64,
    pub created_at: String,
    pub updated_at: String,
}

// 操作记录处理
pub struct StockActionHandler;
impl StockActionHandler {
    /// 插入股票操作数据
    pub fn insert_action(
        stock_id: i32, 
        current_price: f64, 
        current_cost: f64, 
        total_amount: f64, 
        transaction_price: f64, 
        transaction_amount: f64, 
        transaction_commission_fee: f64, 
        action: i32, 
        profit: f64, 
        profit_rate: f64, 
    ) -> Result<i64, rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tb_stock_action (stock_id, current_price, current_cost, total_amount, transaction_price, transaction_amount, transaction_commission_fee, action, profit, profit_rate) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            [
                &stock_id.to_string(), 
                &current_price.to_string(), 
                &current_cost.to_string(), 
                &total_amount.to_string(), 
                &transaction_price.to_string(), 
                &transaction_amount.to_string(), 
                &transaction_commission_fee.to_string(), 
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
            "SELECT stock_action_id, stock_id, current_price, current_cost, total_amount, transaction_price, transaction_amount, transaction_commission_fee, action, profit, profit_rate, created_at, updated_at FROM tb_stock_action WHERE stock_id = ? ORDER BY stock_action_id DESC"
        )?;

        let stock_action_iter = stmt.query_map([stock_id], |row| {
            Ok(StockActionRecord {
                stock_action_id: row.get(0)?,
                stock_id: row.get(1)?,
                current_price: row.get(2)?,
                current_cost: row.get(3)?,
                total_amount: row.get(4)?,
                transaction_price: row.get(5)?,
                transaction_amount: row.get(6)?,
                transaction_commission_fee: row.get(7)?,
                action: row.get(8)?,
                profit: row.get(9)?,
                profit_rate: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;

        let mut stock_actions = Vec::new();
        for stock_action in stock_action_iter {
            stock_actions.push(stock_action?);
        }
        Ok(stock_actions)
    }
    /// 删除最后一条操作记录
    pub fn delete_last_action(stock_id:i32) -> Result<(),rusqlite::Error> {
        let db_conn = get_db_state();
        let conn = db_conn.lock().unwrap();
        conn.execute("DELETE FROM tb_stock_action WHERE stock_action_id = (SELECT MAX(stock_action_id) FROM tb_stock_action WHERE stock_id = ?)", [stock_id])?;
        Ok(())
    }
}