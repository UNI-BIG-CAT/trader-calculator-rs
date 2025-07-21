// 股票类型
pub enum StockType {
    SH = 1, // 沪
    SZ = 2, // 深
    CYB = 3, // 创业板
    KCB = 4, // 科创板
}

impl From<i32> for StockType {
    fn from(value: i32) -> Self {
        match value {
            1 => StockType::SH,
            2 => StockType::SZ,
            3 => StockType::CYB,
            4 => StockType::KCB,
            _ => StockType::SH, // 默认值
        }
    }
}

// 操作类型
pub enum ActionType {
    Open = 1, // 建仓
    Close = 2, // 平仓
    AddPosition = 3, // 加仓
    ReducePosition = 4, // 减仓
}

impl From<i32> for ActionType {
    fn from(value: i32) -> Self {
        match value {
            1 => ActionType::Open,
            2 => ActionType::Close,
            3 => ActionType::AddPosition,
            4 => ActionType::ReducePosition,
            _ => ActionType::Open, // 默认值
        }
    }
}