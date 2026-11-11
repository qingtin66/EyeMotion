//! Tauri 命令集合，按职责拆分为游戏、窗口、配置、主题四个子模块。

pub mod config;
pub mod game;
pub mod theme;
pub mod window;

pub use config::*;
pub use game::*;
pub use theme::*;
pub use window::*;
