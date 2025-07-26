// 股票类型
#[allow(dead_code)]
pub enum StockStatus {
    OPEN = 1,  // 正常买卖中
    CLOSE = 2, // 已经平仓
}

impl From<i32> for StockStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => StockStatus::OPEN,
            2 => StockStatus::CLOSE,
            _ => StockStatus::OPEN, // 默认值
        }
    }
}
