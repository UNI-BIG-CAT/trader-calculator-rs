import { useState, useEffect } from "react";

function BackgroundManager({ onBackgroundChange }) {
  const [backgroundImage, setBackgroundImage] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  // 背景状态
  const [hasCustomBackground, setHasCustomBackground] = useState(false);

  useEffect(() => {
    checkAndLoadBackground();
  }, []);

  const checkAndLoadBackground = () => {
    setIsLoading(true);

    // 支持的图片格式
    const imageFormats = ["jpg", "jpeg", "png", "gif", "webp"];
    let currentFormatIndex = 0;

    const tryLoadImage = () => {
      if (currentFormatIndex >= imageFormats.length) {
        // 没有找到图片
        setBackgroundImage(null);
        setIsLoading(false);
        onBackgroundChange && onBackgroundChange(false);
        return;
      }

      const format = imageFormats[currentFormatIndex];
      const backgroundPath = `/backgrounds/custom-background.${format}`;

      const img = new Image();
      img.onload = () => {
        setBackgroundImage(img.src);
        setIsLoading(false);
        onBackgroundChange && onBackgroundChange(true);
      };
      img.onerror = () => {
        // 尝试下一个格式
        currentFormatIndex++;
        tryLoadImage();
      };

      img.src = backgroundPath;
    };

    tryLoadImage();
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
