//! 视觉主题命令。

use eyemotion_core::Theme;

/// 返回默认视觉主题（当前为品牌主题的唯一来源）。
#[tauri::command]
pub fn get_theme() -> Theme {
    Theme::default()
}
