use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;
use tauri::command;

#[command]
pub fn check_background_image() -> Result<serde_json::Value, String> {
    // 获取当前可执行文件所在目录
    let current_exe =
        std::env::current_exe().map_err(|e| format!("无法获取当前可执行文件路径: {}", e))?;

    let exe_dir = current_exe.parent().ok_or("无法获取可执行文件目录")?;

    // 创建backgrounds文件夹路径（与exe同目录）
    let backgrounds_dir = exe_dir.join("backgrounds");
    println!("{:?}", backgrounds_dir);
    // 确保目录存在
    std::fs::create_dir_all(&backgrounds_dir).map_err(|e| format!("无法创建目录: {}", e))?;

    // 支持的图片格式
    let image_formats = ["jpg", "jpeg", "png", "gif", "webp"];

    // 检查每种格式的图片
    for format in &image_formats {
        let image_path: std::path::PathBuf =
            backgrounds_dir.join(format!("custom-background.{}", format));
        println!("{:?}", image_path);
        if Path::new(&image_path).exists() {
            // 读取图片文件
            let image_data =
                fs::read(&image_path).map_err(|e| format!("无法读取图片文件: {}", e))?;

            // 转换为base64
            let base64_data = general_purpose::STANDARD.encode(&image_data);

            // 根据文件扩展名确定MIME类型
            let mime_type = match format.as_ref() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "webp" => "image/webp",
                _ => "image/jpeg",
            };

            return Ok(serde_json::json!({
                "exists": true,
                "data": format!("data:{};base64,{}", mime_type, base64_data)
            }));
        }
    }

    // 如果没有找到图片，返回false
    Ok(serde_json::json!({
        "exists": false,
        "data": ""
    }))
}
