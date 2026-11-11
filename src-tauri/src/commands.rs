use eyemotion_core::Theme;
use tauri::{State, Window};

/// 单帧允许的最大时间步长（秒），避免帧率抖动导致物理穿透。
const MAX_DT: f64 = 0.1;

/// 推进一帧游戏逻辑；dt 会被校验并夹取到 [0, MAX_DT]。
#[tauri::command]
pub async fn tick(
    state: State<'_, super::state::AppState>,
    dt: f64,
) -> Result<(eyemotion_core::GameState, Vec<eyemotion_core::GameEvent>), String> {
    let dt = eyemotion_core::validation::sanitize_dt(dt, MAX_DT).map_err(|e| e.to_string())?;
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    let update = game_state.update(dt).map_err(|e| e.to_string())?;
    Ok((game_state.clone(), update.events))
}

/// 重置游戏到指定屏幕尺寸；宽高必须为正的有限数值。
#[tauri::command]
pub fn reset_game(
    state: State<'_, super::state::AppState>,
    w: f64,
    h: f64,
) -> Result<eyemotion_core::GameState, String> {
    eyemotion_core::validation::validate_dimensions(w, h).map_err(|e| e.to_string())?;
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    game_state.reset(w, h);
    Ok(game_state.clone())
}

/// 调整屏幕尺寸；宽高必须为正的有限数值。
#[tauri::command]
pub fn resize_game(state: State<'_, super::state::AppState>, w: f64, h: f64) -> Result<(), String> {
    eyemotion_core::validation::validate_dimensions(w, h).map_err(|e| e.to_string())?;
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    game_state.resize(w, h);
    Ok(())
}

/// 切换暂停状态。
#[tauri::command]
pub fn toggle_pause(state: State<'_, super::state::AppState>) {
    match state.game_state.lock() {
        Ok(mut game_state) => game_state.paused = !game_state.paused,
        Err(err) => log::warn!("toggle_pause: 无法锁定 game_state: {err}"),
    }
}

/// 从开始界面进入游戏。
#[tauri::command]
pub fn start_game(state: State<'_, super::state::AppState>) {
    match state.game_state.lock() {
        Ok(mut game_state) => game_state.is_start_screen = false,
        Err(err) => log::warn!("start_game: 无法锁定 game_state: {err}"),
    }
}

/// 进入下一关。
#[tauri::command]
pub fn next_stage(
    state: State<'_, super::state::AppState>,
) -> Result<eyemotion_core::GameState, String> {
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    let mut events = Vec::new();
    game_state.next_stage(&mut events);
    Ok(game_state.clone())
}

/// 返回上一关。
#[tauri::command]
pub fn prev_stage(
    state: State<'_, super::state::AppState>,
) -> Result<eyemotion_core::GameState, String> {
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    let mut events = Vec::new();
    game_state.prev_stage(&mut events);
    Ok(game_state.clone())
}

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

/// 返回默认视觉主题。
#[tauri::command]
pub fn get_theme() -> Theme {
    Theme::default()
}

/// 设置并持久化语言。
#[tauri::command]
pub fn set_language(state: State<'_, super::state::AppState>, language: String) {
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
pub fn get_language(state: State<'_, super::state::AppState>) -> String {
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
pub fn get_config(state: State<'_, super::state::AppState>) -> eyemotion_core::UserConfig {
    match state.user_config.lock() {
        Ok(config) => config.clone(),
        Err(err) => {
            log::warn!("get_config: 无法锁定 user_config: {err}");
            eyemotion_core::UserConfig::default()
        }
    }
}
