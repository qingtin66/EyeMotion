use eyemotion_core::{GameState, UserConfig};
use std::sync::Mutex;

/// 应用全局状态：持有游戏状态与用户配置，均由互斥锁保护。
pub struct AppState {
    pub game_state: Mutex<GameState>,
    pub user_config: Mutex<UserConfig>,
}
