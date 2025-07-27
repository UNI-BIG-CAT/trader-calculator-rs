// 费率结构体
use crate::constant::action_type::ActionType;
use crate::constant::stock_type::StockType;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FeeRates {
    pub tax: f64,        // 印花税费率
    pub regulatory: f64, // 证管费费率
    pub brokerage: f64,  // 经手费费率
    pub transfer: f64,   // 过户费费率
}

impl FeeRates {
    // 为不同股票类型创建费率
    #[allow(dead_code)]
    pub fn for_stock_type(stock_type: StockType, action_type: ActionType) -> Self {
        let tax_rate = match action_type {
            ActionType::AddPosition => 0.0, // 买入时不收印花税
            _ => 0.001, // 卖出时收印花税(开仓需要记录税率，先返回，后续判断不计算)
        };

        match stock_type {
            StockType::SH => Self {
                tax: tax_rate,
                regulatory: 0.00002,
                brokerage: 0.0000487,
                transfer: 0.00001,
            },
            StockType::SZ => Self {
                tax: tax_rate,
                regulatory: 0.00002,
                brokerage: 0.0000341,
                transfer: 0.0,
            },
            StockType::CYB => Self {
                tax: tax_rate,
                regulatory: 0.00002,
                brokerage: 0.0000487,
                transfer: 0.00001,
            },
            StockType::KCB => Self {
                tax: tax_rate,
                regulatory: 0.00002,
                brokerage: 0.0000487,
                transfer: 0.00001,
            },
        }
    }
}
