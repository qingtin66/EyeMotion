//! 游戏流程相关命令：帧推进、重置、尺寸调整、暂停、开始、切关。

use crate::state::AppState;
use eyemotion_core::{GameEvent, GameState};
use tauri::State;

/// 单帧允许的最大时间步长（秒），避免帧率抖动导致物理穿透。
const MAX_DT: f64 = 0.1;

/// 推进一帧游戏逻辑；dt 会被校验并夹取到 [0, MAX_DT]。
#[tauri::command]
pub async fn tick(
    state: State<'_, AppState>,
    dt: f64,
) -> Result<(GameState, Vec<GameEvent>), String> {
    let dt = eyemotion_core::validation::sanitize_dt(dt, MAX_DT).map_err(|e| e.to_string())?;
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    let update = game_state.update(dt).map_err(|e| e.to_string())?;
    Ok((game_state.clone(), update.events))
}

/// 重置游戏到指定屏幕尺寸；宽高必须为正的有限数值。
#[tauri::command]
pub fn reset_game(state: State<'_, AppState>, w: f64, h: f64) -> Result<GameState, String> {
    eyemotion_core::validation::validate_dimensions(w, h).map_err(|e| e.to_string())?;
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    game_state.reset(w, h);
    Ok(game_state.clone())
}

/// 调整屏幕尺寸；宽高必须为正的有限数值。
#[tauri::command]
pub fn resize_game(state: State<'_, AppState>, w: f64, h: f64) -> Result<(), String> {
    eyemotion_core::validation::validate_dimensions(w, h).map_err(|e| e.to_string())?;
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    game_state.resize(w, h);
    Ok(())
}

/// 切换暂停状态。
#[tauri::command]
pub fn toggle_pause(state: State<'_, AppState>) {
    match state.game_state.lock() {
        Ok(mut game_state) => game_state.paused = !game_state.paused,
        Err(err) => log::warn!("toggle_pause: 无法锁定 game_state: {err}"),
    }
}

/// 从开始界面进入游戏。
#[tauri::command]
pub fn start_game(state: State<'_, AppState>) {
    match state.game_state.lock() {
        Ok(mut game_state) => {
            game_state.is_start_screen = false;
            game_state.is_game_over = false;
            game_state.is_transitioning = true;
            game_state.transition_timer = 3.0;
        }
        Err(err) => log::warn!("start_game: 无法锁定 game_state: {err}"),
    }
}

/// 返回开始界面（取消暂停与结束态），由前端在退出对局时调用。
#[tauri::command]
pub fn return_to_start_screen(state: State<'_, AppState>) {
    match state.game_state.lock() {
        Ok(mut game_state) => {
            game_state.is_start_screen = true;
            game_state.paused = false;
            game_state.is_game_over = false;
        }
        Err(err) => log::warn!("return_to_start_screen: 无法锁定 game_state: {err}"),
    }
}

/// 进入下一关。
#[tauri::command]
pub fn next_stage(state: State<'_, AppState>) -> Result<GameState, String> {
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    let mut events = Vec::new();
    game_state.next_stage(&mut events);
    Ok(game_state.clone())
}

/// 返回上一关。
#[tauri::command]
pub fn prev_stage(state: State<'_, AppState>) -> Result<GameState, String> {
    let mut game_state = state.game_state.lock().map_err(|e| e.to_string())?;
    let mut events = Vec::new();
    game_state.prev_stage(&mut events);
    Ok(game_state.clone())
}
