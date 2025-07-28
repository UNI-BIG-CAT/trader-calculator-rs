import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DatePicker } from "antd";
import dayjs from "dayjs";
import "./css/App.css";
import "./css/detail.css";
import "./css/antd-override.css";
import { useToast, ToastContainer } from "./components/Toast.jsx";

function Detail({ stockId, stockName, onBack }) {
  /*******************参数*********************/
  const [stock, setStock] = useState(1);
  const [actionList, setActionList] = useState([]);
  const [showAddOrReduceDialog, setShowAddOrReduceDialog] = useState(false);
  const [showActionInfoDialog, setShowActionInfoDialog] = useState(false);
  const [dialogType, setDialogType] = useState(""); // "add" 或 "reduce"
  const [addOrReduceFormData, setAddOrReduceFormData] = useState({
    currentPrice: "",
    transactionPrice: "",
    transactionPosition: "",
  });
  const [showCloseDialog, setShowCloseDialog] = useState(false);
  const [closeFormData, setCloseFormData] = useState({
    transactionPrice: "",
  });
  // 操作信息
  const [actionForm, setActionForm] = useState({
    actionTime: "",
    actionInfo: "",
  });
  // 保留部分操作
  const [lastActions, setLastActions] = useState(true);
  // Toast Hook
  const { toasts, removeToast, showError, showWarning, showSuccess } =
    useToast();
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
      const stock = await invoke("handle_get_stock_info", { stockId });
      setStock(stock);
      const result = await invoke("handle_get_action_list", { stockId });
      //
      setActionList(result);
    } catch (error) {
      showError("获取详情失败");
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
    return action.transaction_price * action.transaction_position;
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
    let totalExpenditure =
      action.current_cost * action.total_position + action.total_fee;
    return totalExpenditure;
  };
  // 计算当前价清仓手续费
  const getCurrentClosingFees = (action) => {
    const currentValue = action.current_price * action.total_position;
    // 估算清仓时的手续费（佣金+印花税+其他费用）
    return currentValue * (0.0001 + 0.001 + 0.00002 + 0.0000487 + 0.00001);
  };

  // 计算当前价清仓后纯收益
  const getNetProfit = (action) => {
    return (
      action.current_price * action.total_position -
      action.current_cost * action.total_position
    );
  };

  /*************加仓减仓***************/
  // 加仓
  const addStock = () => {
    setDialogType("add");
    setAddOrReduceFormData({
      currentPrice: actionList[0]?.current_price || "",
      transactionPrice: "",
      transactionPosition: "",
    });
    setShowAddOrReduceDialog(true);
  };

  // 减仓
  const reduceStock = () => {
    setDialogType("reduce");
    setAddOrReduceFormData({
      currentPrice: actionList[0]?.current_price || "",
      transactionPrice: "",
      transactionPosition: "",
    });
    setShowAddOrReduceDialog(true);
  };

  // 处理加减仓输入变化
  const handleAddOrReduceInputChange = (e) => {
    const { name, value } = e.target;
    if (name == "transactionPosition" && dialogType == "reduce") {
      if (value >= actionList[actionList.length - 1]?.total_position) {
        showError("减仓数量不能大于持仓数量,或者选择平仓");
        return;
      }
    }
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
      transactionPosition: "",
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
        !addOrReduceFormData.currentPrice ||
        !addOrReduceFormData.transactionPrice ||
        !addOrReduceFormData.transactionPosition
      ) {
        showError("请填写所有字段");
        return;
      }
      if (dialogType == "add") {
        await invoke("handle_add_position", {
          stockId,
          currentPrice: parseFloat(addOrReduceFormData.currentPrice),
          transactionPrice: parseFloat(addOrReduceFormData.transactionPrice),
          transactionPosition: parseInt(
            addOrReduceFormData.transactionPosition
          ),
        });
      } else {
        await invoke("handle_reduce_position", {
          stockId,
          currentPrice: parseFloat(addOrReduceFormData.currentPrice),
          transactionPrice: parseFloat(addOrReduceFormData.transactionPrice),
          transactionPosition: parseInt(
            addOrReduceFormData.transactionPosition
          ),
        });
      }
      // 重新获取数据
      getActionList();
      closeAddOrReduceDialog();
    } catch (error) {
      showError("操作失败");
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
      if (closeFormData.transactionPrice <= 0) {
        showError("请填写平仓价格");
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
      showError("平仓失败");
    }
  };
  /*************回退***************/
  const backStock = async () => {
    console.log("回退");
    await invoke("handle_back_position", { stockId });
    getActionList();
  };

  /*************操作记录***************/
  const showActionInfo = async (action) => {
    try {
      const stock_action_id = action.stock_action_id;
      // 先设置基本信息
      setActionForm({
        stockActionId: stock_action_id,
        actionTime: action.action_time,
        actionInfo: action.action_info,
      });

      setShowActionInfoDialog(true);
    } catch (error) {
      console.error("显示操作信息失败:", error);
    }
  };
  const saveActionInfo = async () => {
    await invoke("handle_save_action_info", {
      stockActionId: actionForm.stockActionId,
      actionTime: actionForm.actionTime,
      actionInfo: actionForm.actionInfo,
    });
    await getActionList();
    setShowActionInfoDialog(false);
  };
  // 格式化日期时间用于 tooltip
  const formatDateTimeForTooltip = (dateTimeString) => {
    if (!dateTimeString) return "";
    const date = new Date(dateTimeString);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");
    return `${year}-${month}-${day} ${hours}:${minutes}`;
  };

  // 移除原来的 handleActionInfoTimeChange 函数，因为现在使用 Ant Design DatePicker
  const handleActionInfoInputChange = (e) => {
    const { name, value } = e.target;
    setActionForm((prev) => ({
      ...prev,
      [name]: value,
    }));
  };
  /********************************************************/
  return (
    <main className="container">
      {/* header */}
      <div className="header">
        <div className="header-title">{stockName}-控仓</div>
        <div></div>
        <div>
          <button
            hidden={actionList.length <= 4}
            className="back-btn"
            onClick={() => setLastActions(!lastActions)}
          >
            {lastActions ? "显示全部" : "隐藏部分"}
          </button>
          <button className="back-btn" onClick={onBack}>
            返回
          </button>
        </div>
      </div>

      {/* 操作列表 */}
      <div className="detail-container">
        <div className="detail-list">
          {(lastActions ? actionList.slice(-4) : actionList).map((action) => (
            <div
              className="action-detail"
              onClick={() => showActionInfo(action)}
              key={action.stock_action_id}
            >
              {/* 操作信息标记 */}
              {action.action_info && action.action_info.trim() !== "" && (
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
                      action.action == 1 || action.action == 3
                        ? "profit"
                        : "loss"
                    }`}
                  >
                    {action.action == 1 || action.action == 3
                      ? formatNumber(action.transaction_position, 0)
                      : formatNumber(-action.transaction_position, 0)}
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
                  <span
                    className={`detail-value ${
                      action.action == 1 || action.action == 3
                        ? "profit"
                        : "loss"
                    }`}
                  >
                    {formatNumber(getCurrentExpenditure(action))}
                  </span>
                </div>

                <div className="detail-row">
                  <span className="detail-label">总计支出</span>
                  <span className="detail-value">
                    {action.total_position > 0
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
                    {action.total_position > 0
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
                      action.profit >= 0 ? "profit" : "loss"
                    }`}
                  >
                    {formatNumber(action.profit)}
                  </span>
                  <span className="detail-label">当前价清仓手续费</span>
                  <span className="detail-value">
                    {action.total_position > 0
                      ? formatNumber(getCurrentClosingFees(action))
                      : "-"}
                  </span>
                </div>

                <div className="detail-row">
                  <span className="detail-label">盈亏比例(忽略清仓手续费)</span>
                  <span
                    className={`detail-value ${
                      action.profit >= 0 ? "profit" : "loss"
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
                    {action.total_position > 0
                      ? formatNumber(getNetProfit(action))
                      : "-"}
                  </span>
                </div>
              </div>
            </div>
          ))}
        </div>
        <div className="detail-action-btn-container">
          <button
            className="detail-action-btn"
            disabled={stock.status == 2}
            onClick={addStock}
          >
            加仓
          </button>
          <button
            className="detail-action-btn"
            disabled={stock.status == 2}
            onClick={reduceStock}
          >
            减仓
          </button>
          <button
            className="detail-action-btn"
            disabled={stock.status == 2}
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
        {actionList.length === 0 && (
          <div className="empty-message">暂无操作记录</div>
        )}
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
                  max="1000"
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
                  max="1000"
                />
              </div>
              <div className="form-group">
                <label>数量:</label>
                <input
                  type="number"
                  name="transactionPosition"
                  value={addOrReduceFormData.transactionPosition}
                  onChange={handleAddOrReduceInputChange}
                  placeholder="请输入数量"
                  min="1"
                  max="1000000"
                />
              </div>
            </div>
            <div className="dialog-actions">
              <button className="btn-cancel" onClick={closeAddOrReduceDialog}>
                取消
              </button>
              <button
                className="btn-confirm"
                disabled={
                  !addOrReduceFormData.transactionPosition ||
                  !addOrReduceFormData.transactionPrice
                }
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
              <button
                className="btn-confirm"
                disabled={!closeFormData.transactionPrice}
                onClick={handleCloseConfirm}
              >
                确定
              </button>
            </div>
          </div>
        </div>
      )}
      {/* 操作详情对话框 */}
      {showActionInfoDialog && (
        <div className="dialog-overlay">
          <div className="dialog">
            <div className="dialog-header">
              <h3>操作笔记</h3>
            </div>
            <div className="dialog-content">
              <div className="time-input-container">
                <DatePicker
                  showTime
                  format="YYYY年MM月DD日 HH:mm"
                  placeholder="请选择日期和时间"
                  value={
                    actionForm.actionTime ? dayjs(actionForm.actionTime) : null
                  }
                  onChange={(date, dateString) => {
                    setActionForm((prev) => ({
                      ...prev,
                      actionTime: date ? date.format("YYYY-MM-DDTHH:mm") : "",
                    }));
                  }}
                  className="antd-datetime-picker"
                />
              </div>
              <textarea
                name="actionInfo"
                value={actionForm.actionInfo}
                onChange={handleActionInfoInputChange}
                className="action-info-textarea"
                rows="6"
                cols="50"
                placeholder="请输入操作信息..."
              ></textarea>
            </div>
            <div className="dialog-actions">
              <button className="btn-confirm" onClick={saveActionInfo}>
                保存
              </button>
              <button
                className="btn-cancel"
                onClick={() => {
                  setActionForm({
                    stockActionId: actionForm.stockActionId,
                    actionTime: "",
                    actionInfo: "",
                  });
                }}
              >
                清空
              </button>
              <button
                className="btn-cancel"
                onClick={() => setShowActionInfoDialog(false)}
              >
                关闭
              </button>
            </div>
          </div>
        </div>
      )}
      {/* Toast */}
      <ToastContainer toasts={toasts} removeToast={removeToast} />
    </main>
  );
}

export default Detail;
