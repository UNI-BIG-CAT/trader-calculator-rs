import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./css/App.css";
import Detail from "./Detail.jsx";
import LimitCal from "./LimitCal.jsx";
import { useToast, ToastContainer } from "./components/Toast.jsx";

function App() {
  /*******************参数*********************/
  // 页面状态管理
  const [currentPage, setCurrentPage] = useState("list"); // "list" 或 "detail" 或者 "limitCal"
  const [selectedStockId, setSelectedStockId] = useState(null);
  const [selectedStockName, setSelectedStockName] = useState(null);

  // 股票列表状态
  const [stockList, setStockList] = useState([]);
  // 对话框显示状态
  const [showDialog, setShowDialog] = useState(false);

  // Toast Hook
  const { toasts, removeToast, showError, showSuccess } = useToast();

  // 从localStorage获取上次保存的手续费率，如果没有则使用默认值
  const getSavedCommissionFeeRate = () => {
    const saved = localStorage.getItem("commissionFeeRate");
    return saved ? parseFloat(saved) : 0.00025;
  };

  // 表单数据状态
  const [formData, setFormData] = useState({
    stockName: "",
    stockType: 1,
    currentPrice: "",
    transactionPrice: "",
    transactionPosition: "",
    commissionFeeRate: getSavedCommissionFeeRate(),
  });

  /*******************生命周期*********************/
  useEffect(() => {
    getStockList();
  }, []);

  /*******************函数*********************/
  // 获取股票列表
  async function getStockList() {
    try {
      const result = await invoke("handle_get_all_stocks");
      setStockList(result);
    } catch (error) {
      console.error("Error getting stock list:", error);
      showError("获取股票列表失败", 1000);
    }
  }

  // 股票类型转换函数
  const getStockTypeText = (stockType) => {
    switch (stockType) {
      case 1:
        return "上海";
      case 2:
        return "深圳";
      case 3:
        return "创业板";
      case 4:
        return "科创板";
      default:
        return "未知";
    }
  };

  // 格式化手续费率
  const formatCommissionFee = (rate) => {
    return (rate * 100).toFixed(4) + "%";
  };

  /*************建仓**************/
  // 建仓对话框
  const openDialog = () => {
    setShowDialog(true);
  };
  const closeDialog = () => {
    setShowDialog(false);
    setFormData({
      stockName: "",
      stockType: 1,
      currentPrice: "",
      transactionPrice: "",
      transactionPosition: "",
      commissionFeeRate: getSavedCommissionFeeRate(),
    });
  };
  const handleInputChange = (e) => {
    const { name, value } = e.target;
    if (name == "transactionPosition") {
      if (value >= 1000000) {
        showError("数量不能大于1000000", 1000);
        return;
      }
    }
    setFormData((prev) => {
      const newFormData = {
        ...prev,
        [name]: value,
      };

      // 如果修改的是手续费率，保存到localStorage
      if (name === "commissionFeeRate") {
        localStorage.setItem("commissionFeeRate", value);
      }

      return newFormData;
    });
  };
  // 确认建仓
  const handleConfirm = async () => {
    try {
      if (
        !formData.stockName ||
        !formData.currentPrice ||
        !formData.transactionPrice ||
        !formData.transactionPosition
      ) {
        showError("请填写所有字段", 1000);
        return;
      }

      await invoke("handle_open_position", {
        stockName: formData.stockName,
        stockType: formData.stockType,
        currentPrice: parseFloat(formData.currentPrice),
        transactionPrice: parseFloat(formData.transactionPrice),
        transactionPosition: parseInt(formData.transactionPosition),
        commissionFeeRate: parseFloat(formData.commissionFeeRate),
      });
      getStockList();
      closeDialog();
      // showSuccess("建仓成功！", 500);
    } catch (error) {
      console.error("Error opening stock:", error);
      showError("建仓失败", 1000);
    }
  };
  /*************查看详情**************/
  const handleViewStock = async (stockId, stockName) => {
    setSelectedStockId(stockId);
    setSelectedStockName(stockName);
    setCurrentPage("detail");
  };

  /*************删除股票**************/
  const handleDeleteStock = async (stockId) => {
    try {
      await invoke("handle_delete_stock", { stockId });
      getStockList();
      // showSuccess("删除成功！", 500);
    } catch (error) {
      console.error("Error deleting stock:", error);
      showError("删除失败", 1000);
    }
  };

  /*************连板**************/
  const handleOpenLimitCal = () => {
    setCurrentPage("LimitCal");
  };

  /*************页面控制**************/
  // 返回列表页
  const handleBackToList = () => {
    setCurrentPage("list");
    setSelectedStockId(null);
    setSelectedStockName(null);
  };
  // 根据当前页面状态渲染不同内容
  if (currentPage === "detail") {
    return (
      <Detail
        stockId={selectedStockId}
        stockName={selectedStockName}
        onBack={handleBackToList}
      />
    );
  } else if (currentPage === "LimitCal") {
    return <LimitCal onBack={handleBackToList} />;
  }

  return (
    <main className="container">
      <div className="header">
        <div className="header-title">盈亏计算器</div>
        <div>
          <button className="open-stock-btn" onClick={openDialog}>
            建仓
          </button>
          <button className="open-stock-btn" onClick={handleOpenLimitCal}>
            连板
          </button>
        </div>
      </div>
      {/* 股票列表 */}
      <div className="stock-list">
        <table className="stock-table">
          <thead>
            <tr>
              <th>股票名称</th>
              <th>类型</th>
              <th>手续费率</th>
              <th>印花税率</th>
              <th>证管费率</th>
              <th>经手费率</th>
              <th>过户费率</th>
              <th>操作</th>
            </tr>
          </thead>
          <tbody>
            {stockList.map((stock) => (
              <tr key={stock.stock_id}>
                <td>{stock.stock_name}</td>
                <td>{getStockTypeText(stock.stock_type)}</td>
                <td>{formatCommissionFee(stock.commission_fee_rate)}</td>
                <td>{formatCommissionFee(stock.tax_fee_rate)}</td>
                <td>{formatCommissionFee(stock.regulatory_fee_rate)}</td>
                <td>{formatCommissionFee(stock.brokerage_fee_rate)}</td>
                <td>{formatCommissionFee(stock.transfer_fee_rate)}</td>
                <td>
                  <button
                    className="action-btn view-btn"
                    onClick={() =>
                      handleViewStock(stock.stock_id, stock.stock_name)
                    }
                  >
                    查看
                  </button>
                  <button
                    className="action-btn delete-btn"
                    onClick={() => handleDeleteStock(stock.stock_id)}
                  >
                    删除
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {stockList.length === 0 && (
          <div className="empty-message">请先建仓股票</div>
        )}
      </div>
      {/* 对话框遮罩 */}
      {showDialog && (
        <div className="dialog-overlay">
          <div className="dialog">
            <div className="dialog-header">
              <h3>股票建仓</h3>
            </div>
            <div className="dialog-content">
              <div className="form-group">
                <label>股票名称:</label>
                <input
                  type="text"
                  name="stockName"
                  value={formData.stockName}
                  onChange={handleInputChange}
                  placeholder="请输入股票名称"
                />
              </div>
              <div className="form-group">
                <div className="stock-type-buttons">
                  <label>股票类型:</label>
                  <button
                    className={
                      formData.stockType === 1
                        ? "stock-type-button-active"
                        : "stock-type-button"
                    }
                    onClick={() => setFormData({ ...formData, stockType: 1 })}
                  >
                    上海股(60开头)
                  </button>
                  <button
                    className={
                      formData.stockType === 2
                        ? "stock-type-button-active"
                        : "stock-type-button"
                    }
                    onClick={() => setFormData({ ...formData, stockType: 2 })}
                  >
                    深圳(00或30开头)
                  </button>
                </div>
              </div>
              <div className="form-group">
                <label>当前价格:</label>
                <input
                  type="number"
                  name="currentPrice"
                  value={formData.currentPrice}
                  onChange={handleInputChange}
                  placeholder="请输入当前价格"
                  step="0.01"
                  max="1000"
                />
              </div>
              <div className="form-group">
                <label>成本:</label>
                <input
                  type="number"
                  name="transactionPrice"
                  value={formData.transactionPrice}
                  onChange={handleInputChange}
                  placeholder="请输入成本"
                  step="0.01"
                  max="1000"
                />
              </div>
              <div className="form-group">
                <label>数量:</label>
                <input
                  type="number"
                  name="transactionPosition"
                  value={formData.transactionPosition}
                  onChange={handleInputChange}
                  placeholder="请输入数量"
                  min="1"
                  max="1000000"
                />
              </div>
              <div className="form-group">
                <label>佣金比例:</label>
                <input
                  type="number"
                  name="commissionFeeRate"
                  value={formData.commissionFeeRate}
                  onChange={handleInputChange}
                  placeholder="请输入佣金比例"
                  max="1"
                />
              </div>
            </div>
            <div className="dialog-actions">
              <button
                className="btn-confirm"
                disabled={
                  !formData.stockName ||
                  !formData.currentPrice ||
                  !formData.transactionPrice ||
                  !formData.transactionPosition
                }
                onClick={handleConfirm}
              >
                确认
              </button>
              <button className="btn-cancel" onClick={closeDialog}>
                取消
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Toast容器 */}
      <ToastContainer toasts={toasts} removeToast={removeToast} />
    </main>
  );
}

export default App;
