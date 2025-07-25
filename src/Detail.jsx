import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./css/App.css";
import "./css/detail.css";

function Detail({ stockId, stockName, onBack }) {
  /*******************参数*********************/
  const [stockStatus, setStockStatus] = useState(1);
  const [actionList, setActionList] = useState([]);
  const [showAddOrReduceDialog, setShowAddOrReduceDialog] = useState(false);
  const [dialogType, setDialogType] = useState(""); // "add" 或 "reduce"
  const [addOrReduceFormData, setAddOrReduceFormData] = useState({
    currentPrice: "",
    transactionPrice: "",
    transactionAmount: "",
  });
  const [showCloseDialog, setShowCloseDialog] = useState(false);
  const [closeFormData, setCloseFormData] = useState({
    transactionPrice: "",
  });

  /*******************生命周期*********************/
  useEffect(() => {
    if (stockId) {
      getActionList();
    }
  }, [stockId]);

  /*******************函数*********************/
  /*************获取列表***************/
  // 获取详情
  const getActionList = async () => {
    try {
      const stock = await invoke("get_stock_info", { stockId });
      setStockStatus(stock.status);
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
    return num.toFixed(2) * 100 + "%";
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
    let totalExpenditure = marketValue + totalFees;
    if (action.action == 2 || action.action == 4) {
      totalExpenditure = -totalExpenditure;
    }
    return totalExpenditure;
  };
  // 总支出
  const getTotalExpenditure = (action) => {
    const marketValue = getTransactionValue(action);
    const totalFees = action.total_fee;
    let totalExpenditure = marketValue + totalFees;
    return totalExpenditure;
  };
  // 计算当前价清仓手续费
  const getCurrentClosingFees = (action) => {
    const currentValue = action.current_price * action.total_amount;
    // 估算清仓时的手续费（佣金+印花税+其他费用）
    return currentValue * (0.0001 + 0.001 + 0.00002 + 0.0000487 + 0.00001);
  };

  // 计算当前价清仓后纯收益
  const getNetProfit = (action) => {
    if (action.total_amount > 0) {
      return action.profit - getCurrentClosingFees(action);
    } else {
      return 0;
    }
  };

  /*************加仓减仓***************/
  // 加仓
  const addStock = () => {
    setDialogType("add");
    setAddOrReduceFormData({
      currentPrice: actionList[0]?.current_price || "",
      transactionPrice: "",
      transactionAmount: "",
    });
    setShowAddOrReduceDialog(true);
  };

  // 减仓
  const reduceStock = () => {
    setDialogType("reduce");
    setAddOrReduceFormData({
      currentPrice: actionList[0]?.current_price || "",
      transactionPrice: "",
      transactionAmount: "",
    });
    setShowAddOrReduceDialog(true);
  };

  // 处理加减仓输入变化
  const handleAddOrReduceInputChange = (e) => {
    const { name, value } = e.target;
    setAddOrReduceFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  // 处理平仓输入变化
  const handleCloseInputChange = (e) => {
    const { name, value } = e.target;
    setCloseFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  // 关闭加减仓对话框
  const closeAddOrReduceDialog = () => {
    setShowAddOrReduceDialog(false);
    setDialogType("");
    setAddOrReduceFormData({
      currentPrice: "",
      transactionPrice: "",
      transactionAmount: "",
    });
  };

  // 关闭平仓对话框
  const closeCloseDialog = () => {
    setShowCloseDialog(false);
    setCloseFormData({
      transactionPrice: "",
    });
  };

  // 确认加减仓
  const handleAddOrReduceConfirm = async () => {
    try {
      if (
        !addOrReduceFormData.transactionPrice ||
        !addOrReduceFormData.transactionAmount
      ) {
        alert("请填写所有字段");
        return;
      }
      if (dialogType == "add") {
        await invoke("handle_add_position", {
          stockId,
          currentPrice: parseFloat(addOrReduceFormData.currentPrice),
          transactionPrice: parseFloat(addOrReduceFormData.transactionPrice),
          transactionAmount: parseInt(addOrReduceFormData.transactionAmount),
        });
      } else {
        await invoke("handle_reduce_position", {
          stockId,
          currentPrice: parseFloat(addOrReduceFormData.currentPrice),
          transactionPrice: parseFloat(addOrReduceFormData.transactionPrice),
          transactionAmount: parseInt(addOrReduceFormData.transactionAmount),
        });
      }
      // 重新获取数据
      getActionList();
      closeAddOrReduceDialog();
    } catch (error) {
      console.error("Error processing add/reduce:", error);
      alert("操作失败: " + error);
    }
  };

  /*************平仓***************/
  // 平仓
  const closeStock = () => {
    setCloseFormData({
      transactionPrice: actionList[0]?.current_price || "",
    });
    setShowCloseDialog(true);
  };

  // 确认平仓
  const handleCloseConfirm = async () => {
    try {
      if (!closeFormData.transactionPrice) {
        alert("请填写平仓价格");
        return;
      }
      await invoke("handle_close_position", {
        stockId,
        actionType: 2,
        currentPrice: parseFloat(closeFormData.transactionPrice),
      });
      // 重新获取数据
      getActionList();
      closeCloseDialog();
    } catch (error) {
      console.error("Error processing close:", error);
      alert("平仓失败: " + error);
    }
  };
  /*************回退***************/
  const backStock = async () => {
    console.log("回退");
    await invoke("handle_back_position", { stockId });
    getActionList();
  };

  /********************************************************/
  return (
    <main className="container">
      {/* header */}
      <div className="header">
        <div className="header-title">{stockName}-控仓</div>
        <div></div>
        <div></div>
        <button className="back-btn" onClick={onBack}>
          返回
        </button>
      </div>

      {/* 操作列表 */}
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
                  {formatNumber(getTotalExpenditure(action))}
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
                  {formatNumber(action.profit)}
                </span>
                <span className="detail-label">当前价清仓手续费</span>
                <span className="detail-value">
                  {formatNumber(getCurrentClosingFees(action)) > 0
                    ? formatNumber(getCurrentClosingFees(action))
                    : "-"}
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
                  {getNetProfit(action) > 0
                    ? formatNumber(getNetProfit(action))
                    : "-"}
                </span>
              </div>
            </div>
          </div>
        ))}

        {actionList.length === 0 && (
          <div className="empty-message">暂无操作记录</div>
        )}

        <div className="detail-action-btn-container">
          <button
            className="detail-action-btn"
            disabled={stockStatus == 2}
            onClick={addStock}
          >
            加仓
          </button>
          <button
            className="detail-action-btn"
            disabled={stockStatus == 2}
            onClick={reduceStock}
          >
            减仓
          </button>
          <button
            className="detail-action-btn"
            disabled={stockStatus == 2}
            onClick={closeStock}
          >
            平仓
          </button>
          <button
            className="detail-action-btn"
            disabled={actionList.length <= 1}
            onClick={backStock}
          >
            回退
          </button>
        </div>
      </div>

      {/* 加减仓对话框 */}
      {showAddOrReduceDialog && (
        <div className="dialog-overlay">
          <div className="dialog">
            <div className="dialog-header">
              <h3>{dialogType === "add" ? "加仓" : "减仓"}</h3>
            </div>
            <div className="dialog-content">
              <div className="form-group">
                <label>当前价格:</label>
                <input
                  type="number"
                  name="currentPrice"
                  value={addOrReduceFormData.currentPrice}
                  onChange={handleAddOrReduceInputChange}
                  placeholder="请输入当前价格"
                  step="0.01"
                />
              </div>
              <div className="form-group">
                <label>{dialogType === "add" ? "加仓" : "减仓"}价格:</label>
                <input
                  type="number"
                  name="transactionPrice"
                  value={addOrReduceFormData.transactionPrice}
                  onChange={handleAddOrReduceInputChange}
                  placeholder={`请输入${
                    dialogType === "add" ? "加仓" : "减仓"
                  }价格`}
                  step="0.01"
                />
              </div>
              <div className="form-group">
                <label>数量:</label>
                <input
                  type="number"
                  name="transactionAmount"
                  value={addOrReduceFormData.transactionAmount}
                  onChange={handleAddOrReduceInputChange}
                  placeholder="请输入数量"
                  min="1"
                />
              </div>
            </div>
            <div className="dialog-actions">
              <button className="btn-cancel" onClick={closeAddOrReduceDialog}>
                取消
              </button>
              <button
                className="btn-confirm"
                onClick={handleAddOrReduceConfirm}
              >
                确定
              </button>
            </div>
          </div>
        </div>
      )}

      {/* 平仓对话框 */}
      {showCloseDialog && (
        <div className="dialog-overlay">
          <div className="dialog">
            <div className="dialog-header">
              <h3>平仓</h3>
            </div>
            <div className="dialog-content">
              <div className="form-group">
                <label>平仓价格:</label>
                <input
                  type="number"
                  name="transactionPrice"
                  value={closeFormData.transactionPrice}
                  onChange={handleCloseInputChange}
                  placeholder="请输入平仓价格"
                  step="0.01"
                />
              </div>
            </div>
            <div className="dialog-actions">
              <button className="btn-cancel" onClick={closeCloseDialog}>
                取消
              </button>
              <button className="btn-confirm" onClick={handleCloseConfirm}>
                确定
              </button>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

export default Detail;
