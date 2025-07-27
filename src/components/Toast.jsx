import { useState, useEffect } from "react";
import "./Toast.css";

const Toast = ({ message, type = "info", duration = 1000, onClose }) => {
  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsVisible(false);
      setTimeout(() => {
        onClose && onClose();
      }, 300); // 等待动画完成
    }, duration);

    return () => clearTimeout(timer);
  }, [duration, onClose]);

  return (
    <div className={`toast ${type} ${isVisible ? "show" : "hide"}`}>
      <div className="toast-message">{message}</div>
    </div>
  );
};

// Toast容器组件，用于管理多个Toast
export const ToastContainer = ({ toasts, removeToast }) => {
  return (
    <div className="toast-container">
      {toasts.map((toast) => (
        <Toast
          key={toast.id}
          message={toast.message}
          type={toast.type}
          duration={toast.duration}
          onClose={() => removeToast(toast.id)}
        />
      ))}
    </div>
  );
};

// Toast Hook，用于在组件中使用Toast
export const useToast = () => {
  const [toasts, setToasts] = useState([]);

  const addToast = (message, type = "info", duration = 3000) => {
    const id = Date.now() + Math.random();
    const newToast = { id, message, type, duration };
    setToasts((prev) => [...prev, newToast]);
  };

  const removeToast = (id) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  };

  const showSuccess = (message, duration) =>
    addToast(message, "success", duration);
  const showError = (message, duration) => addToast(message, "error", duration);
  const showWarning = (message, duration) =>
    addToast(message, "warning", duration);
  const showInfo = (message, duration) => addToast(message, "info", duration);

  return {
    toasts,
    addToast,
    removeToast,
    showSuccess,
    showError,
    showWarning,
    showInfo,
  };
};

export default Toast;
