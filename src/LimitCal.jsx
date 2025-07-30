import { useState, useEffect } from "react";
import { useToast, ToastContainer } from "./components/Toast.jsx";
import "./css/limitCal.css";

function LimitCal({ onBack }) {
  const [startPrice, setStartPrice] = useState("");
  const [stockQuantity, setStockQuantity] = useState("");
  const [dailyChange, setDailyChange] = useState(10);
  const [boardCount, setBoardCount] = useState("");
  const [limitResult, setLimitResult] = useState([]);

  // Toast Hook
  const { toasts, removeToast, showError, showSuccess } = useToast();

  // 组件加载时自动获得焦点
  useEffect(() => {
    const inputSection = document.querySelector(".input-section");
    if (inputSection) {
      inputSection.focus();
    }
  }, []);

  const handleSubmit = () => {
    if (startPrice <= 0 || startPrice > 1000) {
      showError("起始价格不能小于等于0或大于1000", 1000);
      return;
    }
    if (stockQuantity <= 0 || stockQuantity > 1000000) {
      showError("股票数量不能小于等于0", 1000);
      return;
    }
    if (boardCount > 30) {
      showError("板数不能大于30", 1000);
      return;
    }
    const result = [];
    for (let i = 0; i < boardCount; i++) {
      const currentPrice = startPrice * Math.pow(1 + dailyChange / 100, i + 1);
      const previousPrice =
        i === 0 ? startPrice : startPrice * Math.pow(1 + dailyChange / 100, i);
      const priceChange = currentPrice - previousPrice;
      const priceChangePercent =
        ((currentPrice - previousPrice) / previousPrice) * 100;
      const currentMarketValue = currentPrice * stockQuantity;

      result.push({
        day: i + 1,
        currentPrice: currentPrice,
        priceChange: priceChange,
        priceChangePercent: priceChangePercent,
        currentMarketValue: currentMarketValue,
      });
    }
    setLimitResult(result);
  };

  const handleClear = () => {
    setStartPrice("");
    setStockQuantity("");
    setDailyChange(10);
    setBoardCount("");
    setLimitResult([]);
  };

  return (
    <main className="container">
      {/* header */}
      <div className="header">
        <div className="header-title">连板计算器</div>
        <div></div>
        <div></div>
        <button className="back-btn" onClick={onBack}>
          返回
        </button>
      </div>

      {/* 输入区域 */}
      <div
        className="input-section"
        onKeyDown={(e) => {
          if (e.key === "Enter" && startPrice && stockQuantity && boardCount) {
            handleSubmit();
          }
        }}
        tabIndex={0}
      >
        <div className="input-row">
          <label>起始价格:</label>
          <input
            type="number"
            value={startPrice}
            onChange={(e) => setStartPrice(e.target.value)}
            placeholder="请输入起始价格"
            className="input-field"
            max="1000"
          />
        </div>

        <div className="input-row">
          <label>股票数量:</label>
          <input
            type="number"
            value={stockQuantity}
            onChange={(e) => setStockQuantity(e.target.value)}
            placeholder="请输入股票数量"
            className="input-field"
            step={100}
            max="1000000"
          />
        </div>

        <div className="input-row">
          <label>日涨幅:</label>
          <div className="button-group">
            <button
              className={`change-btn ${dailyChange === 10 ? "active" : ""}`}
              onClick={() => setDailyChange(10)}
            >
              10%
            </button>
            <button
              className={`change-btn ${dailyChange === 5 ? "active" : ""}`}
              onClick={() => setDailyChange(5)}
            >
              5%
            </button>
            <button
              className={`change-btn ${dailyChange === -5 ? "active" : ""}`}
              onClick={() => setDailyChange(-5)}
            >
              -5%
            </button>
            <button
              className={`change-btn ${dailyChange === -10 ? "active" : ""}`}
              onClick={() => setDailyChange(-10)}
            >
              -10%
            </button>
          </div>
        </div>

        <div className="input-row">
          <label>板数:</label>
          <input
            type="number"
            value={boardCount}
            onChange={(e) => setBoardCount(e.target.value)}
            placeholder="请输入板数"
            className="input-field"
            max="30"
          />
        </div>
        <div className="limit-btn-container">
          <button
            className="limit-ok-btn"
            disabled={!startPrice || !stockQuantity || !boardCount}
            onClick={handleSubmit}
          >
            确定
          </button>
          <button className="limit-clear-btn" onClick={handleClear}>
            清空
          </button>
        </div>
      </div>
      {/* 展示连板区域 */}
      <div className="limit-result-container">
        {limitResult.length > 0 && (
          <div className="limit-result-table">
            <table>
              <thead>
                <tr>
                  <th>板数</th>
                  <th>当前价格</th>
                  <th>涨幅</th>
                  <th>上涨额度</th>
                  <th>当前市值</th>
                </tr>
              </thead>
              <tbody>
                {limitResult.map((item, index) => (
                  <tr key={index}>
                    <td>第{item.day}板</td>
                    <td
                      className={
                        item.priceChange >= 0 ? "positive" : "negative"
                      }
                    >
                      {item.currentPrice.toFixed(2)}
                    </td>
                    <td
                      className={
                        item.priceChangePercent >= 0 ? "positive" : "negative"
                      }
                    >
                      {item.priceChangePercent.toFixed(2)}%
                    </td>
                    <td
                      className={
                        item.priceChange >= 0 ? "positive" : "negative"
                      }
                    >
                      {item.priceChange.toFixed(2)}
                    </td>
                    <td
                      className={
                        item.priceChange >= 0 ? "positive" : "negative"
                      }
                    >
                      {item.currentMarketValue.toFixed(2)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
      {/* Toast */}
      <ToastContainer toasts={toasts} removeToast={removeToast} />
    </main>
  );
}

export default LimitCal;
