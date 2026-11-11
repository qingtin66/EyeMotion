//! 窗口与应用生命周期相关命令。

use tauri::Window;

/// 退出应用。
#[tauri::command]
pub fn exit_app() {
    std::process::exit(0);
}

/// 最小化窗口（移动端为空实现）。
#[tauri::command]
pub fn minimize_window(window: Window) {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    if let Err(err) = window.minimize() {
        log::warn!("minimize_window 失败: {err}");
    }
}

/// 切换窗口全屏状态（移动端为空实现）。
#[tauri::command]
pub fn toggle_fullscreen(window: Window) {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let is_fullscreen = match window.is_fullscreen() {
            Ok(value) => value,
            Err(err) => {
                log::warn!("toggle_fullscreen: 读取全屏状态失败: {err}");
                false
            }
        };
        if let Err(err) = window.set_fullscreen(!is_fullscreen) {
            log::warn!("toggle_fullscreen: 设置全屏失败: {err}");
        }
    }
}

/// 显示并聚焦主窗口，同时强制全屏（移动端为空实现）。
#[tauri::command]
pub fn show_main_window(window: Window) {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        if let Err(err) = window.set_fullscreen(true) {
            log::warn!("show_main_window: 设置全屏失败: {err}");
        }
        if let Err(err) = window.show() {
            log::warn!("show_main_window: 显示窗口失败: {err}");
        }
        if let Err(err) = window.set_focus() {
            log::warn!("show_main_window: 聚焦窗口失败: {err}");
        }
    }
}
