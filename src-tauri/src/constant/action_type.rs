// 操作类型
pub enum ActionType {
    Open = 1,           // 建仓
    Close = 2,          // 平仓
    AddPosition = 3,    // 加仓
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
