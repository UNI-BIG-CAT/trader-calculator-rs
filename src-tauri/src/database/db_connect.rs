use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex, OnceLock};

/*************************************数据库状态管理器**************************************/
// 应用状态管理器，用于共享数据库连接
pub struct DatabaseState {
    pub db: Arc<Mutex<Connection>>,
}

impl DatabaseState {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?; // 打开数据库

        // 设置时区为中国时区 (UTC+8)
        conn.execute("PRAGMA timezone = '+08:00'", [])?;

        create_tables(&conn)?; // 创建表
        Ok(DatabaseState {
            db: Arc::new(Mutex::new(conn)),
        })
    }
    // 获取数据库连接
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.db)
    }
}

/*************************************数据库表创建**************************************/
// 独立的表格创建函数，可以被DatabaseState
fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "
         CREATE TABLE IF NOT EXISTS tb_stock_fee (
            stock_fee_id INTEGER PRIMARY KEY AUTOINCREMENT,       -- ID
            stock_fee_name TEXT NOT NULL,                         -- 费用名称
            commission_fee_rate REAL NOT NULL DEFAULT 0.0003, -- 佣金费率 万0.1~万3(最低5元)
            tax_fee_rate REAL NOT NULL DEFAULT 0.0001,        -- 印花税 0.1% 仅卖出收取
            regulatory_fee_rate REAL NOT NULL DEFAULT 0.00002, -- 证管费 0.002%
            brokerage_fee_rate REAL NOT NULL DEFAULT 0.0000487,  -- 经手费 沪市为0.00487% 深市为0.0341‰
            transfer_fee_rate REAL NOT NULL DEFAULT 0,         -- 过户费 沪市为0.001%(万0.1) 深市为0 
            created_at DATETIME DEFAULT (datetime('now', 'localtime')),    -- 创建时间
            updated_at DATETIME DEFAULT (datetime('now', 'localtime'))     -- 更新时间（无法自动更新）
        );
        ",
        [],
    )?;

    conn.execute(
        "
         insert into tb_stock_fee (stock_fee_id, stock_fee_name, commission_fee_rate, tax_fee_rate, regulatory_fee_rate, brokerage_fee_rate, transfer_fee_rate) values (1,'默认费率', 0.00025, 0.001, 0.00002, 0.0000341, 0);
        ",
        [],
    ).ok(); // 忽略错误，可能已经存在

    conn.execute(
        "
         CREATE TABLE IF NOT EXISTS tb_stock (
            stock_id INTEGER PRIMARY KEY AUTOINCREMENT,       -- ID
            stock_name TEXT NOT NULL,                         -- 股票名称
            type INTEGER NOT NULL DEFAULT 1,                  -- 股票类型 1-沪 2-深 3-创业板 4-科创板
            commission_fee_rate REAL NOT NULL DEFAULT 0.0003, -- 佣金费率 万0.1~万3(最低5元)
            tax_fee_rate REAL NOT NULL DEFAULT 0.0001,        -- 印花税 0.1% 仅卖出收取
            regulatory_fee_rate REAL NOT NULL DEFAULT 0.00002, -- 证管费 0.002%
            brokerage_fee_rate REAL NOT NULL DEFAULT 0.0000487,  -- 经手费 沪市为0.00487% 深市为0.0341‰
            transfer_fee_rate REAL NOT NULL DEFAULT 0,         -- 过户费 沪市为0.001%(万0.1) 深市为0 
            status INTEGER NOT NULL DEFAULT 1,                 -- 状态 1-正常买卖中 2-已经平仓
            sort INTEGER NOT NULL DEFAULT 0,                 -- 排序
            created_at DATETIME DEFAULT (datetime('now', 'localtime')),    -- 创建时间
            updated_at DATETIME DEFAULT (datetime('now', 'localtime'))     -- 更新时间（无法自动更新）
        );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS tb_stock_action (
            stock_action_id INTEGER PRIMARY KEY AUTOINCREMENT,     -- ID
            stock_id INTEGER NOT NULL,                             -- 股票ID
            current_price REAL NOT NULL,                           -- 当前价格
            current_cost REAL NOT NULL DEFAULT 0,                  -- 持仓成本
            total_position REAL NOT NULL,                            -- 总数量
            total_fee REAL NOT NULL DEFAULT 0,                     -- 到目前总手续费
            transaction_price REAL NOT NULL,                       -- 交易价格
            transaction_position REAL NOT NULL,                      -- 交易数量
            transaction_commission_fee REAL NOT NULL DEFAULT 0,    -- 交易佣金费
            transaction_tax_fee REAL NOT NULL DEFAULT 0,           -- 交易印花税
            transaction_regulatory_fee REAL NOT NULL DEFAULT 0,    -- 交易证管费
            transaction_brokerage_fee REAL NOT NULL DEFAULT 0,     -- 交易经手费
            transaction_transfer_fee REAL NOT NULL DEFAULT 0,      -- 交易过户费
            action INTEGER NOT NULL DEFAULT 1,                     -- 操作类型 1-建仓 2-平仓 3-加仓 4-减仓
            profit REAL NOT NULL DEFAULT 0,                        -- 盈亏金额(忽略清仓手续费)
            profit_rate REAL NOT NULL DEFAULT 0,                   -- 盈亏比例(忽略清仓手续费)
            action_time DATETIME NOT NULL DEFAULT '', -- 操作时间
            action_info TEXT NOT NULL DEFAULT '',            -- 操作信息
            created_at DATETIME DEFAULT (datetime('now', 'localtime')),         -- 创建时间
            updated_at DATETIME DEFAULT (datetime('now', 'localtime'))          -- 更新时间（需应用层更新）
        );
        ",
        [],
    )?;
    // 创建索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tb_stock_action_stock_id ON tb_stock_action(stock_id);",
        [],
    )?;

    // 创建触发器来自动更新修改时间
    conn.execute(
                 "CREATE TRIGGER IF NOT EXISTS update_tb_stock_timestamp 
         AFTER UPDATE ON tb_stock_fee
         BEGIN
            UPDATE tb_stock SET updated_at = datetime('now', 'localtime') WHERE stock_fee_id = NEW.stock_fee_id;
         END",
        [],
    )?;

    // 创建触发器来自动更新修改时间
    conn.execute(
                 "CREATE TRIGGER IF NOT EXISTS update_tb_stock_timestamp 
         AFTER UPDATE ON tb_stock
         BEGIN
            UPDATE tb_stock SET updated_at = datetime('now', 'localtime') WHERE stock_id = NEW.stock_id;
         END",
        [],
    )?;

    conn.execute(
                 "CREATE TRIGGER IF NOT EXISTS update_tb_stock_action_timestamp 
         AFTER UPDATE ON tb_stock_action
         BEGIN
            UPDATE tb_stock_action SET updated_at = datetime('now', 'localtime') WHERE stock_action_id = NEW.stock_action_id;
         END",
        [],
    )?;

    Ok(())
}

/*************************************全局数据库实例**************************************/
static DB_INSTANCE: OnceLock<Arc<Mutex<Connection>>> = OnceLock::new();

// 获取公用的数据库状态 - 单例模式
#[allow(dead_code)]
pub fn get_db_state() -> Arc<Mutex<Connection>> {
    DB_INSTANCE
        .get_or_init(|| {
            let db_path = "calculator-db.db";
            let db_state = DatabaseState::new(db_path).unwrap();
            db_state.get_connection()
        })
        .clone()
}
