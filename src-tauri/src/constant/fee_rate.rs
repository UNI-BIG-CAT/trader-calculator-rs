// 费率结构体
use crate::constant::action_type::ActionType;
use crate::constant::stock_type::StockType;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FeeRates {
    pub commission: f64, // 佣金费率
    pub tax: f64,        // 印花税费率
    pub regulatory: f64, // 证管费费率
    pub brokerage: f64,  // 经手费费率
    pub transfer: f64,   // 过户费费率
}

impl FeeRates {
    // 为不同股票类型创建费率
    #[allow(dead_code)]
    pub fn for_stock_type(stock_type: StockType, action_type: ActionType) -> Self {
        // 方法1：简洁的match判断
        let tax_rate = match action_type {
            ActionType::Close | ActionType::ReducePosition => 0.001, // 卖出时收印花税
            _ => 0.0,                                                // 买入时不收印花税
        };

        match stock_type {
            StockType::SH => Self {
                commission: 0.0001,
                tax: tax_rate,
                regulatory: 0.00002,
                brokerage: 0.0000487,
                transfer: 0.00001,
            },
            StockType::SZ => Self {
                commission: 0.0001,
                tax: tax_rate,
                regulatory: 0.00002,
                brokerage: 0.0000487,
                transfer: 0.00001,
            },
            StockType::CYB => Self {
                commission: 0.0001,
                tax: tax_rate,
                regulatory: 0.00002,
                brokerage: 0.0000487,
                transfer: 0.00001,
            },
            StockType::KCB => Self {
                commission: 0.0001,
                tax: tax_rate,
                regulatory: 0.00002,
                brokerage: 0.0000487,
                transfer: 0.00001,
            },
        }
    }
}
