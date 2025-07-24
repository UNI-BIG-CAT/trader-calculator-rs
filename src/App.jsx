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
    transaction_price: "",
    transaction_amount: "",
  });

  useEffect(() => {
    getStockList();
  }, []);

  // 获取股票列表
  async function getStockList() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setStockList(await invoke("get_all_stocks"));
    console.log(stockList);
  }

  // 建仓对话框
  const openDialog = () => {
    setShowDialog(true);
  };
  const closeDialog = () => {
    setShowDialog(false);
    setFormData({
      stockName: "",
      currentPrice: "",
      transaction_price: "",
      transaction_amount: "",
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
    // 这里可以添加表单验证
    if (
      !formData.stockName ||
      !formData.currentPrice ||
      !formData.transaction_price ||
      !formData.transaction_amount
    ) {
      alert("请填写所有字段");
      return;
    }
    await invoke("open_stock", {
      stockName: formData.stockName,
      currentPrice: formData.currentPrice,
      transaction_price: formData.transaction_price,
      transaction_amount: formData.transaction_amount,
    });
    getStockList();
    closeDialog();
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
                  name="transaction_price"
                  value={formData.transaction_price}
                  onChange={handleInputChange}
                  placeholder="请输入成本"
                  step="0.01"
                />
              </div>
              <div className="form-group">
                <label>数量:</label>
                <input
                  type="number"
                  name="transaction_amount"
                  value={formData.transaction_amount}
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
