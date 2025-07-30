import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DatePicker } from "antd";
import dayjs from "dayjs";
import "./css/App.css";
import "./css/detail.css";
import "./css/antd-override.css";
import { useToast, ToastContainer } from "./components/Toast.jsx";
import DetailCard from "./components/DetailCard.jsx";
import ActionInfoDialog from "./components/ActionInfoDialog.jsx";
import BackgroundManager from "./components/BackgroundManager.jsx";

function Detail({ stockId, stockName, onBack, handleViewActionInfo }) {
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
  const [showBackConfirmDialog, setShowBackConfirmDialog] = useState(false);
  const [closeFormData, setCloseFormData] = useState({
    transactionPrice: "",
  });
  // 操作信息
  const [actionForm, setActionForm] = useState({
    actionTime: "",
    actionInfo: "",
  });
  // 保留部分操作
  const [lastActions, setLastActions] = useState(false);
  // 背景状态
  const [hasCustomBackground, setHasCustomBackground] = useState(false);
  // Toast Hook
  const { toasts, removeToast, showError, showWarning, showSuccess } =
    useToast();
  /*******************生命周期*********************/
  useEffect(() => {
    if (stockId) {
      getActionList();
    }
  }, [stockId]);

  // 对话框显示时自动获得焦点
  useEffect(() => {
    if (showAddOrReduceDialog) {
      setTimeout(() => {
        const dialogElement = document.querySelector(".dialog");
        if (dialogElement) {
          dialogElement.focus();
        }
      }, 0);
    }
  }, [showAddOrReduceDialog]);

  useEffect(() => {
    if (showCloseDialog) {
      setTimeout(() => {
        const dialogElement = document.querySelector(".dialog");
        if (dialogElement) {
          dialogElement.focus();
        }
      }, 0);
    }
  }, [showCloseDialog]);

  useEffect(() => {
    if (showBackConfirmDialog) {
      setTimeout(() => {
        const dialogElement = document.querySelector(".dialog-delete");
        if (dialogElement) {
          dialogElement.focus();
        }
      }, 0);
    }
  }, [showBackConfirmDialog]);

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
      currentPrice: actionList[actionList.length - 1]?.current_price || "",
      transactionPrice: "",
      transactionPosition: "",
    });
    setShowAddOrReduceDialog(true);
  };

  // 减仓
  const reduceStock = () => {
    setDialogType("reduce");
    setAddOrReduceFormData({
      currentPrice: actionList[actionList.length - 1]?.current_price || "",
      transactionPrice: "",
      transactionPosition: "",
    });
    setShowAddOrReduceDialog(true);
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

  // 处理加减仓输入变化
  const handleAddOrReduceInputChange = (e) => {
    const { name, value } = e.target;
    // 限制买卖数量
    if (name == "transactionPosition" && dialogType == "reduce") {
      if (value >= actionList[actionList.length - 1]?.total_position) {
        showError("减仓数量不能大于持仓数量,或者选择平仓");
        return;
      }
    }
    // 如果交易价格小于当前价格,则提示
    if (name == "currentPrice" && value > 3000) {
      showError("当前价格不能大于3000");
      return;
    }
    // 如果交易价格大于当前价格,则提示
    if (name == "transactionPrice" && value > 3000) {
      showError("交易价格不能大于3000");
      return;
    }
    if (name == "transactionPosition" && value > 1000000) {
      showError("交易数量不能大于1000000");
      return;
    }
    // 更新当前价格的时候,更新交易价格
    if (name === "currentPrice") {
      setAddOrReduceFormData((prev) => ({
        ...prev,
        transactionPrice: value,
      }));
    }
    // 设置其他字段
    setAddOrReduceFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  // 处理平仓输入变化
  const handleCloseInputChange = (e) => {
    const { name, value } = e.target;
    if (name === "transactionPrice" && value > 3000) {
      showError("当前价格不能大于3000");
      return;
    }
    setCloseFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
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
      transactionPrice: actionList[actionList.length - 1]?.current_price || "",
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
    await invoke("handle_back_position", { stockId });
    getActionList();
    setShowBackConfirmDialog(false);
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

  // 处理操作信息变化
  const handleActionInfoInputChange = (e) => {
    const { name, value } = e.target;
    setActionForm((prev) => ({
      ...prev,
      [name]: value,
    }));
  };
  /*******************渲染*********************/
  return (
    <main className="container">
      <BackgroundManager onBackgroundChange={setHasCustomBackground} />
      {/* header */}
      <div
        className="header"
        style={{
          backgroundColor: hasCustomBackground
            ? "rgba(255, 255, 255, 0.2)"
            : undefined,
          backdropFilter: hasCustomBackground ? "blur(10px)" : undefined,
        }}
      >
        <div className="header-title">{stockName}-控仓</div>
        <div></div>
        <div>
          <button
            className="back-btn"
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.2)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(10px)" : undefined,
            }}
            onClick={() => handleViewActionInfo(stockId, stockName)}
          >
            笔记管理
          </button>
          <button
            hidden={actionList.length <= 4}
            className="back-btn"
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.2)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(10px)" : undefined,
            }}
            onClick={() => setLastActions(!lastActions)}
          >
            {lastActions ? "显示全部" : "隐藏部分"}
          </button>
          <button
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.2)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(10px)" : undefined,
            }}
            className="back-btn"
            onClick={onBack}
          >
            返回
          </button>
        </div>
      </div>

      {/* 操作列表 */}
      <div className="detail-container">
        <div className="detail-list">
          {(lastActions ? actionList.slice(-4) : actionList).map((action) => (
            <DetailCard
              key={action.stock_action_id}
              action={action}
              onClick={showActionInfo}
              hasCustomBackground={hasCustomBackground}
              formatNumber={formatNumber}
              formatPercent={formatPercent}
              getActionTypeText={getActionTypeText}
              getTransactionValue={getTransactionValue}
              getCurrentExpenditure={getCurrentExpenditure}
              getTotalExpenditure={getTotalExpenditure}
              getCurrentClosingFees={getCurrentClosingFees}
              getNetProfit={getNetProfit}
              formatDateTimeForTooltip={formatDateTimeForTooltip}
            />
          ))}
        </div>
        <div
          className="detail-action-btn-container"
          style={{
            backgroundColor: hasCustomBackground
              ? "rgba(255, 255, 255, 0.2)"
              : undefined,
            backdropFilter: hasCustomBackground ? "blur(15px)" : undefined,
          }}
        >
          <button
            className="detail-action-btn"
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.2)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(15px)" : undefined,
            }}
            disabled={stock.status == 2}
            onClick={addStock}
          >
            加仓
          </button>
          <button
            className="detail-action-btn"
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.2)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(15px)" : undefined,
            }}
            disabled={stock.status == 2}
            onClick={reduceStock}
          >
            减仓
          </button>
          <button
            className="detail-action-btn"
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.2)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(15px)" : undefined,
            }}
            disabled={stock.status == 2}
            onClick={closeStock}
          >
            平仓
          </button>
          <button
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.2)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(15px)" : undefined,
            }}
            className="detail-action-btn"
            disabled={actionList.length <= 1}
            onClick={() => {
              if (actionList[actionList.length - 1].action_info) {
                setShowBackConfirmDialog(true);
                return;
              }
              backStock();
            }}
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
        <div className="dialog-overlay" onClick={closeAddOrReduceDialog}>
          <div
            className="dialog"
            onClick={(e) => e.stopPropagation()}
            onKeyDown={(e) => {
              if (
                e.key === "Enter" &&
                addOrReduceFormData.transactionPosition &&
                addOrReduceFormData.transactionPrice
              ) {
                handleAddOrReduceConfirm();
              }
            }}
            tabIndex={0}
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.95)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(15px)" : undefined,
            }}
          >
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
                  max="3001"
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
                  max="3001"
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
                  min="100"
                  max="1000100"
                  step={100}
                />
              </div>
            </div>
            <div className="dialog-actions">
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
              <button className="btn-cancel" onClick={closeAddOrReduceDialog}>
                取消
              </button>
            </div>
          </div>
        </div>
      )}

      {/* 平仓对话框 */}
      {showCloseDialog && (
        <div className="dialog-overlay" onClick={closeCloseDialog}>
          <div
            className="dialog"
            onClick={(e) => e.stopPropagation()}
            onKeyDown={(e) => {
              if (e.key === "Enter" && closeFormData.transactionPrice) {
                handleCloseConfirm();
              }
            }}
            tabIndex={0}
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.95)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(15px)" : undefined,
            }}
          >
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
                  max="3001"
                  step="0.01"
                />
              </div>
            </div>
            <div className="dialog-actions">
              <button
                className="btn-confirm"
                disabled={!closeFormData.transactionPrice}
                onClick={handleCloseConfirm}
              >
                确定
              </button>
              <button className="btn-cancel" onClick={closeCloseDialog}>
                取消
              </button>
            </div>
          </div>
        </div>
      )}

      {/* 操作详情对话框 */}
      {showActionInfoDialog && (
        <ActionInfoDialog
          showActionInfoDialog={showActionInfoDialog}
          hasCustomBackground={hasCustomBackground}
          actionForm={actionForm}
          setActionForm={setActionForm}
          handleActionInfoInputChange={handleActionInfoInputChange}
          saveActionInfo={saveActionInfo}
          setShowActionInfoDialog={setShowActionInfoDialog}
        />
      )}

      {/* 回退确认框 */}
      {showBackConfirmDialog && (
        <div
          className="dialog-delete-overlay"
          onClick={() => setShowBackConfirmDialog(false)}
        >
          <div
            className="dialog-delete"
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                backStock();
              }
            }}
            tabIndex={0}
            style={{
              backgroundColor: hasCustomBackground
                ? "rgba(255, 255, 255, 0.95)"
                : undefined,
              backdropFilter: hasCustomBackground ? "blur(15px)" : undefined,
            }}
          >
            <div className="dialog-delete-header">
              <h3>你确定要回退这个操作吗?</h3>
            </div>
            <div className="delete-btn-container">
              <button className="btn-confirm" onClick={() => backStock()}>
                确定
              </button>
              <button
                className="btn-cancel"
                onClick={() => setShowBackConfirmDialog(false)}
              >
                取消
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
