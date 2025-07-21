use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

// 应用状态管理器，用于共享数据库连接
pub struct DatabaseState {
    pub db: Arc<Mutex<Connection>>,
}

impl DatabaseState {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        create_tables(&conn)?;
        Ok(DatabaseState {
            db: Arc::new(Mutex::new(conn)),
        })
    }
    // 获取数据库连接
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.db)
    }
}

// 独立的表格创建函数，可以被DatabaseState
fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "
         CREATE TABLE IF NOT EXISTS tb_stock (
            stock_id INTEGER PRIMARY KEY AUTOINCREMENT,       -- ID
            stock_name TEXT NOT NULL,                         -- 股票名称
            type INTEGER NOT NULL DEFAULT 1,                  -- 股票类型 1-沪 2-深 3-创业板 4-科创板
            commission_fee_rate REAL NOT NULL DEFAULT 0.0003, -- 佣金费率
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,    -- 创建时间
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP     -- 更新时间（无法自动更新）
        );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS tb_stock_action (
            stock_action_id INTEGER PRIMARY KEY AUTOINCREMENT,     -- ID
            stock_id INTEGER NOT NULL,                             -- 股票ID
            price REAL NOT NULL,                                   -- 交易价格
            amount REAL NOT NULL,                                  -- 交易数量
            commission_fee REAL NOT NULL DEFAULT 0,                -- 佣金费
            action INTEGER NOT NULL DEFAULT 1,                     -- 操作类型：1-建仓 2-平仓 3-加仓 4-减仓
            current_cost REAL NOT NULL DEFAULT 0,                  -- 当前成本
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,         -- 创建时间
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP          -- 更新时间（需应用层更新）
        );
        ",
        [],
    )?;

    // 创建触发器来自动更新修改时间
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_tb_stock_timestamp 
         AFTER UPDATE ON tb_stock
         BEGIN
            UPDATE tb_stock SET updated_at = CURRENT_TIMESTAMP WHERE stock_id = NEW.stock_id;
         END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_tb_stock_action_timestamp 
         AFTER UPDATE ON tb_stock_action
         BEGIN
            UPDATE tb_stock_action SET updated_at = CURRENT_TIMESTAMP WHERE stock_action_id = NEW.stock_action_id;
         END",
        [],
    )?;

    Ok(())
}

// 数据结构定义
#[derive(Debug, Clone)]
pub struct StockRecord {
    pub stock_id: i32,
    pub stock_name: String,
    pub stock_type: i32,
    pub commission_fee_rate: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct StockActionRecord {
    pub stock_action_id: i32,
    pub stock_id: i32,
    pub price: f64,
    pub amount: f64,
    pub commission_fee: f64,
    pub action: i32,
    pub current_cost: f64,
    pub created_at: String,
    pub updated_at: String,
}