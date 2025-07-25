import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./css/App.css";
import "./css/detail.css";

function Detail({ stockId, onBack }) {
  const [actionList, setActionList] = useState([]);

  useEffect(() => {
    if (stockId) {
      getActionList();
    }
  }, [stockId]);

  const getActionList = async () => {
    try {
      const result = await invoke("get_action_list", { stockId });
      setActionList(result);
    } catch (error) {
      console.error("Error getting action list:", error);
    }
  };

  // 操作类型转换
  const getActionTypeText = (actionType) => {
    switch (actionType) {
      case 1:
        return "建仓";
      case 2:
        return "平仓";
      case 3:
        return "加仓";
      case 4:
        return "减仓";
      default:
        return "未知";
    }
  };

  // 格式化数字
  const formatNumber = (num, decimals = 2) => {
    return num.toFixed(decimals);
  };

  // 格式化百分比
  const formatPercent = (num) => {
    return (num * 100).toFixed(2) + "%";
  };

  // 计算交易市值
  const getTransactionValue = (action) => {
    return action.transaction_price * action.transaction_amount;
  };

  // 计算此次支出
  const getCurrentExpenditure = (action) => {
    const marketValue = getTransactionValue(action);
    const totalFees =
      action.transaction_commission_fee +
      action.transaction_tax_fee +
      action.transaction_regulatory_fee +
      action.transaction_brokerage_fee +
      action.transaction_transfer_fee;
    return marketValue + totalFees;
  };

  // 计算当前价清仓手续费
  const getCurrentClosingFees = (action) => {
    const currentValue = action.current_price * action.total_amount;
    // 估算清仓时的手续费（佣金+印花税+其他费用）
    return currentValue * (0.0001 + 0.001 + 0.00002 + 0.0000487 + 0.00001);
  };

  // 计算当前价清仓后纯收益
  const getNetProfit = (action) => {
    return action.profit - getCurrentClosingFees(action);
  };

  return (
    <main className="container">
      <div className="header">
        <button className="back-btn" onClick={onBack}>
          返回
        </button>
        <div className="header-title">股票详情</div>
        <div></div>
        <div></div>
      </div>

      <div className="detail-container">
        {actionList.map((action) => (
          <div className="action-detail" key={action.stock_action_id}>
            <div className="detail-grid">
              <div className="detail-row">
                <span className="detail-label">操作类型</span>
                <span className="detail-value">
                  {getActionTypeText(action.action)}
                </span>
                <span className="detail-label">交易数量</span>
                <span className="detail-value highlight">
                  {formatNumber(action.transaction_amount, 0)}
                </span>
              </div>

              <div className="detail-row">
                <span className="detail-label">交易价格</span>
                <span className="detail-value highlight">
                  {formatNumber(action.transaction_price)}
                </span>
                <span className="detail-label">交易市值</span>
                <span className="detail-value">
                  {formatNumber(getTransactionValue(action))}
                </span>
              </div>

              <div className="detail-row">
                <span className="detail-label">手续费</span>
                <span className="detail-value">
                  {formatNumber(action.transaction_commission_fee)}
                </span>
                <span className="detail-label">此次支出</span>
                <span className="detail-value">
                  {formatNumber(getCurrentExpenditure(action))}
                </span>
              </div>

              <div className="detail-row">
                <span className="detail-label">总计支出</span>
                <span className="detail-value">
                  {formatNumber(getCurrentExpenditure(action))}
                </span>
                <span className="detail-label">股票余额</span>
                <span className="detail-value">
                  {formatNumber(action.total_amount, 0)}
                </span>
              </div>

              <div className="detail-row">
                <span className="detail-label">持仓成本</span>
                <span className="detail-value highlight">
                  {formatNumber(action.current_cost, 3)}
                </span>
                <span className="detail-label">当前价格(参考)</span>
                <span className="detail-value highlight">
                  {formatNumber(action.current_price)}
                </span>
              </div>

              <div className="detail-row">
                <span className="detail-label">盈亏金额(忽略清仓手续费)</span>
                <span
                  className={`detail-value ${
                    action.profit >= 0 ? "profit" : "loss"
                  }`}
                >
                  {action.profit >= 0 ? "" : ""}
                  {formatNumber(action.profit)}
                </span>
                <span className="detail-label">当前价清仓手续费</span>
                <span className="detail-value">
                  {formatNumber(getCurrentClosingFees(action))}
                </span>
              </div>

              <div className="detail-row">
                <span className="detail-label">盈亏比例(忽略清仓手续费)</span>
                <span
                  className={`detail-value ${
                    action.profit_rate >= 0 ? "profit" : "loss"
                  }`}
                >
                  {formatPercent(action.profit_rate)}
                </span>

                <span className="detail-label">当前价清仓后纯收益</span>
                <span
                  className={`detail-value ${
                    getNetProfit(action) >= 0 ? "profit" : "loss"
                  }`}
                >
                  {formatNumber(getNetProfit(action))}
                </span>
              </div>
            </div>
          </div>
        ))}

        {actionList.length === 0 && (
          <div className="empty-message">暂无操作记录</div>
        )}
      </div>
    </main>
  );
}

export default Detail;
