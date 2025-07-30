import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import dayjs from "dayjs";
import { DatePicker } from "antd";
import "./css/App.css";
import "./css/actionInfo.css";
import { useToast, ToastContainer } from "./components/Toast.jsx";
import DetailCard from "./components/DetailCard.jsx";

function ActionInfo({ stockId, stockName, onBack }) {
  /*******************参数*********************/
  const [actionList, setActionList] = useState([]);
  const [selectedAction, setSelectedAction] = useState(null);
  const [showModal, setShowModal] = useState(false);
  const [showActionInfoDialog, setShowActionInfoDialog] = useState(false);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const [actionForm, setActionForm] = useState({
    stockActionId: "",
    actionTime: "",
    actionInfo: "",
  });
  const [deleteActionId, setDeleteActionId] = useState(null);
  const { showError, showSuccess } = useToast();

  /*******************生命周期*********************/
  useEffect(() => {
    getActionList();
  }, []);

  // 删除确认对话框显示时自动获得焦点
  useEffect(() => {
    if (showDeleteDialog) {
      // 使用setTimeout确保DOM已经渲染
      setTimeout(() => {
        const dialogElement = document.querySelector(".dialog-delete");
        if (dialogElement) {
          dialogElement.focus();
        }
      }, 0);
    }
  }, [showDeleteDialog]);
  /*******************函数*********************/
  // 获取详情
  const getActionList = async () => {
    try {
      const result = await invoke("handle_get_action_list", { stockId });
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

  // 查看笔记详情
  const handleViewNote = (action) => {
    setSelectedAction(action);
    setShowModal(true);
    setActionForm({
      stockActionId: action.stock_action_id,
      actionTime: action.action_time,
      actionInfo: action.action_info,
    });
  };

  // 删除操作
  const handleDeleteAction = async () => {
    try {
      await invoke("handle_save_action_info", {
        stockActionId: deleteActionId,
        actionTime: "",
        actionInfo: "",
      });
      getActionList(); // 重新获取列表
      setShowDeleteDialog(false);
    } catch (error) {
      showError("删除失败");
    }
  };

  // 处理操作信息变化
  const handleActionInfoInputChange = (e) => {
    const { name, value } = e.target;
    setActionForm((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  // 保存新的笔记
  const handleSaveAction = async () => {
    await invoke("handle_save_action_info", {
      stockActionId: actionForm.stockActionId,
      actionTime: actionForm.actionTime,
      actionInfo: actionForm.actionInfo,
    });
    await getActionList();
    closeModal();
  };
  // 关闭对话框
  const closeModal = () => {
    setShowModal(false);
    setShowDeleteDialog(false);
    setSelectedAction(null);
    setDeleteActionId(null);
  };

  // 格式化数字
  const formatNumber = (num, decimals = 2) => {
    if (num === null || num === undefined || isNaN(num)) {
      return "-";
    }
    return num.toFixed(decimals);
  };

  // 格式化百分比
  const formatPercent = (num) => {
    return (num * 100).toFixed(2) + "%";
  };

  // 计算交易市值
  const getTransactionValue = (action) => {
    if (!action || !action.transaction_price || !action.transaction_position) {
      return 0;
    }
    return action.transaction_price * action.transaction_position;
  };

  // 计算此次支出
  const getCurrentExpenditure = (action) => {
    if (!action) return 0;

    const marketValue = getTransactionValue(action);
    const totalFees =
      (action.transaction_commission_fee || 0) +
      (action.transaction_tax_fee || 0) +
      (action.transaction_regulatory_fee || 0) +
      (action.transaction_brokerage_fee || 0) +
      (action.transaction_transfer_fee || 0);
    let totalExpenditure = marketValue + totalFees;
    if (action.action == 2 || action.action == 4) {
      totalExpenditure = -totalExpenditure;
    }
    return totalExpenditure;
  };

  // 计算总计支出
  const getTotalExpenditure = (action) => {
    return action?.total_expenditure || 0;
  };

  // 计算当前价清仓手续费
  const getCurrentClosingFees = (action) => {
    return action?.current_closing_fees || 0;
  };

  // 计算净收益
  const getNetProfit = (action) => {
    return action?.net_profit || 0;
  };

  // 格式化日期时间用于tooltip
  const formatDateTimeForTooltip = (dateTimeString) => {
    return dayjs(dateTimeString).format("YYYY-MM-DD HH:mm:ss");
  };

  /*******************渲染*********************/
  return (
    <main className="container">
      {/* header */}
      <div className="header">
        <div className="header-title">{stockName}-笔记</div>
        <div></div>
        <div>
          <button
            className="back-btn"
            onClick={() => onBack("detail", stockId, stockName)}
          >
            返回
          </button>
        </div>
      </div>

      {/* 笔记表格 */}
      <div className="action-list">
        <table className="action-table">
          <thead>
            <tr>
              <th>类型</th>
              <th>数量</th>
              <th>价格</th>
              <th>成本</th>
              <th>利润</th>
              <th>笔记</th>
              <th>操作时间</th>
              <th>操作</th>
            </tr>
          </thead>
          <tbody>
            {actionList.length === 0 ? (
              <tr>
                <td colSpan="7" className="empty-message">
                  暂时没有笔记
                </td>
              </tr>
            ) : (
              actionList
                .filter((action) => action.action_info)
                .map((action, index) => (
                  <tr key={index}>
                    <td>{getActionTypeText(action.action)}</td>
                    <td>{action.transaction_position}</td>
                    <td>{action.transaction_price}</td>
                    <td>{action.transaction_cost}</td>
                    <td
                      className={
                        action.profit < 0
                          ? "profit-negative"
                          : action.profit >= 0
                          ? "profit-positive"
                          : ""
                      }
                    >
                      {action.profit.toFixed(2) || "-"}
                    </td>
                    <td>
                      <span className="note-content" title={action.action_info}>
                        {action.action_info}
                      </span>
                    </td>
                    <td>
                      {dayjs(action.action_time).format("YYYY-MM-DD HH:mm")}
                    </td>
                    <td>
                      <div className="action-buttons">
                        <button
                          className="action-btn view-btn"
                          onClick={() => handleViewNote(action)}
                        >
                          查看
                        </button>
                        <button
                          className="action-btn delete-btn"
                          onClick={() => {
                            setShowDeleteDialog(true);
                            setDeleteActionId(action.stock_action_id);
                          }}
                        >
                          删除
                        </button>
                      </div>
                    </td>
                  </tr>
                ))
            )}
          </tbody>
        </table>
      </div>

      {/* 笔记详情对话框 */}
      {showModal && selectedAction && (
        <div className="dialog-overlay" onClick={closeModal}>
          <div
            className="dialog-detail-container"
            onClick={(e) => e.stopPropagation()}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                handleSaveAction();
              }
            }}
          >
            <div className="dialog-header">
              <h3>笔记详情</h3>
            </div>
            <DetailCard
              showIndicator={false}
              isDialog={true}
              action={selectedAction}
              onClick={() => {}}
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
            <div className="dialog-content">
              <div className="time-input-container">
                <DatePicker
                  showTime
                  format="YYYY年MM月DD日 HH:mm"
                  placeholder="请选择日期和时间"
                  value={
                    actionForm.actionTime
                      ? dayjs(actionForm.actionTime)
                      : dayjs(actionForm.actionTime)
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
            <div className="detail-btn-container">
              <button
                className="btn-confirm"
                onClick={() => handleSaveAction()}
              >
                保存
              </button>
              <button className="btn-cancel" onClick={() => closeModal()}>
                取消
              </button>
            </div>
          </div>
        </div>
      )}

      {/* 删除确认对话框 */}
      {showDeleteDialog && (
        <div className="dialog-delete-overlay" onClick={closeModal}>
          <div
            className="dialog-delete"
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                handleDeleteAction();
              }
            }}
            tabIndex={0}
          >
            <div className="dialog-delete-header">
              <h3>你确定要删除这个笔记吗？</h3>
            </div>
            <div className="delete-btn-container">
              <button
                className="btn-confirm"
                onClick={() => handleDeleteAction()}
              >
                确定
              </button>
              <button className="btn-cancel" onClick={() => closeModal()}>
                取消
              </button>
            </div>
          </div>
        </div>
      )}

      {/* 操作详情对话框 */}
      {showActionInfoDialog && (
        <div className="dialog-overlay">
          <div
            className="dialog"
            onKeyDown={(e) => {
              if (e.key === "Enter" && actionForm.actionInfo.trim()) {
                saveActionInfo();
              }
            }}
            tabIndex={0}
          >
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

      {/* <ToastContainer /> */}
    </main>
  );
}

export default ActionInfo;
