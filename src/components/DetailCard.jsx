function DetailCard({
  showIndicator = true,
  isDialog = false,
  action,
  onClick,
  formatNumber,
  formatPercent,
  getActionTypeText,
  getTransactionValue,
  getCurrentExpenditure,
  getTotalExpenditure,
  getCurrentClosingFees,
  getNetProfit,
  formatDateTimeForTooltip,
}) {
  // 添加空值检查
  if (!action) {
    return null;
  }

  return (
    <div
      className={`${isDialog ? "dialog-detail-content" : "action-detail"}`}
      onClick={() => onClick(action)}
      key={action.stock_action_id}
    >
      {/* 操作信息标记 */}
      {showIndicator &&
        action.action_info &&
        action.action_info.trim() !== "" && (
          <div
            className="action-info-indicator"
            title={formatDateTimeForTooltip(action.action_time)}
          ></div>
        )}
      <div className="detail-grid">
        <div className="detail-row">
          <span className="detail-label">操作类型</span>
          <span className="detail-value">
            {getActionTypeText(action.action)}
          </span>
          <span className="detail-label">交易数量</span>
          <span
            className={`detail-value ${
              action.action == 1 || action.action == 3 ? "profit" : "loss"
            }`}
          >
            {action.action == 1 || action.action == 3
              ? formatNumber(action.transaction_position, 0)
              : formatNumber(-action.transaction_position, 0)}
          </span>
        </div>

        <div className="detail-row">
          <span className="detail-label">交易价格</span>
          <span
            className={`detail-value ${
              action.action == 1 || action.action == 3 ? "profit" : "loss"
            }`}
          >
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
          <span
            className={`detail-value ${
              action.action == 1 || action.action == 3 ? "profit" : "loss"
            }`}
          >
            {formatNumber(getCurrentExpenditure(action))}
          </span>
        </div>

        <div className="detail-row">
          <span className="detail-label">总计支出</span>
          <span className="detail-value">
            {(action.total_position || 0) > 0
              ? formatNumber(getTotalExpenditure(action))
              : "-"}
          </span>
          <span className="detail-label">股票余额</span>
          <span className="detail-value">
            {formatNumber(action.total_position, 0)}
          </span>
        </div>

        <div className="detail-row">
          <span className="detail-label">持仓成本</span>
          <span className="detail-value highlight">
            {(action.total_position || 0) > 0
              ? formatNumber(action.current_cost, 3)
              : "-"}
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
              (action.profit || 0) >= 0 ? "profit" : "loss"
            }`}
          >
            {formatNumber(action.profit)}
          </span>
          <span className="detail-label">当前价清仓手续费</span>
          <span className="detail-value">
            {(action.total_position || 0) > 0
              ? formatNumber(getCurrentClosingFees(action))
              : "-"}
          </span>
        </div>

        <div className="detail-row">
          <span className="detail-label">盈亏比例(忽略清仓手续费)</span>
          <span
            className={`detail-value ${
              (action.profit || 0) >= 0 ? "profit" : "loss"
            }`}
          >
            {formatPercent(action.profit_rate)}
          </span>

          <span className="detail-label">当前价清仓后纯收益</span>
          <span
            className={`detail-value ${
              (getNetProfit(action) || 0) >= 0 ? "profit" : "loss"
            }`}
          >
            {(action.total_position || 0) > 0
              ? formatNumber(getNetProfit(action))
              : "-"}
          </span>
        </div>
      </div>
    </div>
  );
}

export default DetailCard;
