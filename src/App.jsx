import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  // 股票列表状态
  const [stockList, setStockList] = useState([]);
  // 对话框显示状态
  const [showDialog, setShowDialog] = useState(false);
  // 表单数据状态
  const [formData, setFormData] = useState({
    stockName: "",
    stockType: 1,
    currentPrice: "",
    transactionPrice: "",
    transactionAmount: "",
  });

  useEffect(() => {
    getStockList();
  }, []);

  // 获取股票列表
  async function getStockList() {
    try {
      const result = await invoke("get_all_stocks");
      setStockList(result);
    } catch (error) {
      console.error("Error getting stock list:", error);
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

  // 格式化时间
  const formatDateTime = (dateString) => {
    if (!dateString) return "暂无";
    try {
      const date = new Date(dateString);
      return date.toLocaleString("zh-CN", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch (error) {
      return dateString;
    }
  };

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
      transactionAmount: "",
    });
  };
  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };
  // 确认建仓
  const handleConfirm = async () => {
    try {
      if (
        !formData.stockName ||
        !formData.currentPrice ||
        !formData.transactionPrice ||
        !formData.transactionAmount
      ) {
        alert("请填写所有字段");
        return;
      }

      await invoke("open_stock", {
        stockName: formData.stockName,
        stockType: formData.stockType,
        currentPrice: parseFloat(formData.currentPrice),
        transactionPrice: parseFloat(formData.transactionPrice),
        transactionAmount: parseInt(formData.transactionAmount),
      });
      getStockList();
      closeDialog();
    } catch (error) {
      console.error("Error opening stock:", error);
      alert("建仓失败: " + error);
    }
  };
  // 删除股票
  const handleDeleteStock = async (stockId) => {
    try {
      await invoke("handle_delete_stock", { stockId });
      getStockList();
    } catch (error) {
      console.error("Error deleting stock:", error);
    }
  };

  return (
    <main className="container">
      <div className="header">
        <div></div>
        <div className="header-title">股票列表</div>
        <button className="open-stock-btn" onClick={openDialog}>
          建仓
        </button>
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
              {/* <th>建仓时间</th> */}
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
                {/* <td>{formatDateTime(stock.created_at)}</td> */}
                <td>
                  <button className="action-btn view-btn">查看</button>
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
          <div className="empty-message">暂无股票数据</div>
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
                />
              </div>
              <div className="form-group">
                <label>数量:</label>
                <input
                  type="number"
                  name="transactionAmount"
                  value={formData.transactionAmount}
                  onChange={handleInputChange}
                  placeholder="请输入数量"
                  min="1"
                />
              </div>
            </div>
            <div className="dialog-actions">
              <button className="btn-confirm" onClick={handleConfirm}>
                确认
              </button>
              <button className="btn-cancel" onClick={closeDialog}>
                取消
              </button>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;
