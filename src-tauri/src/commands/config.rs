//! 用户配置相关命令：语言读写与配置读取。

use crate::state::AppState;
use eyemotion_core::UserConfig;
use tauri::State;

/// 设置并持久化语言。
#[tauri::command]
pub fn set_language(state: State<'_, AppState>, language: String) {
    match state.user_config.lock() {
        Ok(mut config) => {
            config.language = language;
            if let Err(err) = config.save() {
                log::warn!("set_language: 保存配置失败: {err}");
            }
        }
        Err(err) => log::warn!("set_language: 无法锁定 user_config: {err}"),
    }
}

/// 读取当前语言。
#[tauri::command]
pub fn get_language(state: State<'_, AppState>) -> String {
    match state.user_config.lock() {
        Ok(config) => config.language.clone(),
        Err(err) => {
            log::warn!("get_language: 无法锁定 user_config: {err}");
            "en".to_string()
        }
    }
}

/// 读取用户配置。
#[tauri::command]
pub fn get_config(state: State<'_, AppState>) -> UserConfig {
    match state.user_config.lock() {
        Ok(config) => config.clone(),
        Err(err) => {
            log::warn!("get_config: 无法锁定 user_config: {err}");
            UserConfig::default()
        }
    }
}
