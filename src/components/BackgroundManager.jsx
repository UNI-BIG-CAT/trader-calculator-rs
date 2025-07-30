import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

function BackgroundManager({ onBackgroundChange }) {
  const [backgroundImage, setBackgroundImage] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  // 背景状态
  const [hasCustomBackground, setHasCustomBackground] = useState(false);

  useEffect(() => {
    checkAndLoadBackground();
  }, []);

  const checkAndLoadBackground = async () => {
    setIsLoading(true);

    try {
      // 使用Tauri的文件系统API检查背景图片
      const result = await invoke("check_background_image");
      if (result.exists) {
        // 如果图片存在，使用base64数据
        setBackgroundImage(result.data);
        setIsLoading(false);
        onBackgroundChange && onBackgroundChange(true);
      } else {
        // 没有找到图片
        setBackgroundImage(null);
        setIsLoading(false);
        onBackgroundChange && onBackgroundChange(false);
      }
    } catch (error) {
      console.error("Error checking background image:", error);
      setBackgroundImage(null);
      setIsLoading(false);
      onBackgroundChange && onBackgroundChange(false);
    }
  };

  // 如果正在加载，显示加载状态
  if (isLoading) {
    return null;
  }

  // 如果没有背景图片，返回null
  if (!backgroundImage) {
    return null;
  }

  // 应用背景样式
  const backgroundStyle = {
    position: "fixed",
    top: 0,
    left: 0,
    width: "100%",
    height: "100%",
    backgroundImage: `url(${backgroundImage})`,
    backgroundSize: "cover",
    backgroundPosition: "center",
    backgroundRepeat: "no-repeat",
    zIndex: -1,
    opacity: 0.3, // 设置透明度，让内容仍然清晰可见
  };

  return <div style={backgroundStyle} />;
}

export default BackgroundManager;
