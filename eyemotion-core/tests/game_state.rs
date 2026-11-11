//! eyemotion-core 游戏状态机集成测试。
//!
//! 覆盖初始状态、暂停/游戏结束/起始屏的门控、过渡倒计时、关卡推进与回退、
//! 重置/缩放，以及 Stage 2/3/4/5 的分支行为。涉及随机位置处只断言不变量。

use eyemotion_core::{stage_speed, GameEvent, GameState};

fn speed_of(state: &GameState) -> f64 {
    (state.ball.vx.powi(2) + state.ball.vy.powi(2)).sqrt()
}

fn assert_approx(actual: f64, expected: f64, eps: f64) {
    assert!(
        (actual - expected).abs() <= eps,
        "expected {expected} ± {eps}, got {actual}"
    );
}

/// 构造一个已跳过起始屏与过渡、可以直接推进的状态。
fn playable() -> GameState {
    let mut s = GameState::new(1000.0, 800.0);
    s.is_start_screen = false;
    s.is_transitioning = false;
    s
}

#[test]
fn new_state_defaults() {
    let s = GameState::new(800.0, 600.0);
    assert_eq!(s.stage, 1);
    assert!(s.is_start_screen);
    assert!(s.is_transitioning);
    assert!(!s.is_game_over);
    assert!(!s.paused);
    assert_eq!(s.stage_elapsed, 0.0);
    assert_approx(s.transition_timer, 3.0, 1e-9);
    assert_approx(speed_of(&s), stage_speed(1), 1e-6);
}

#[test]
fn update_is_noop_on_start_screen() {
    let mut s = GameState::new(800.0, 600.0);
    let before = (s.ball.x, s.ball.y);
    let update = s.update(0.1).unwrap();
    assert!(update.events.is_empty());
    assert_eq!((s.ball.x, s.ball.y), before);
    assert_eq!(s.stage_elapsed, 0.0);
}

#[test]
fn update_is_noop_when_paused() {
    let mut s = playable();
    s.paused = true;
    let before = (s.ball.x, s.ball.y);
    let update = s.update(0.1).unwrap();
    assert!(update.events.is_empty());
    assert_eq!((s.ball.x, s.ball.y), before);
}

#[test]
fn update_is_noop_when_game_over() {
    let mut s = playable();
    s.is_game_over = true;
    let update = s.update(0.1).unwrap();
    assert!(update.events.is_empty());
    assert_eq!(s.stage_elapsed, 0.0);
}

#[test]
fn transition_countdown_blocks_progress_then_clears() {
    let mut s = GameState::new(800.0, 600.0);
    s.is_start_screen = false; // 仍处于过渡中
    s.update(1.0).unwrap();
    assert!(s.is_transitioning);
    assert_approx(s.transition_timer, 2.0, 1e-9);
    assert_eq!(s.stage_elapsed, 0.0);

    // 让倒计时结束
    s.transition_timer = 0.05;
    s.update(0.1).unwrap();
    assert!(!s.is_transitioning);
    assert_eq!(s.stage_elapsed, 0.0);
}

#[test]
fn stage1_advances_stage_elapsed() {
    let mut s = playable();
    let before = (s.ball.x, s.ball.y);
    let update = s.update(0.5).unwrap();
    assert_approx(update.time_elapsed, 0.5, 1e-9);
    assert_approx(s.stage_elapsed, 0.5, 1e-9);
    // 物理推进：小球位置应发生变化（可能发生边界反弹，故不假设 events 为空）
    assert_ne!((s.ball.x, s.ball.y), before);
}

#[test]
fn stage_advances_and_resets_ball_when_time_exceeded() {
    let mut s = playable();
    s.stage_elapsed = 45.9;
    let update = s.update(0.2).unwrap(); // 46.1s > 45s
    assert_eq!(s.stage, 2);
    assert!(update
        .events
        .contains(&GameEvent::StageChanged { from: 1, to: 2 }));
    assert!(s.is_transitioning);
    assert_approx(s.transition_timer, 3.0, 1e-9);
    assert_approx(speed_of(&s), stage_speed(2), 1e-6);
}

#[test]
fn next_stage_from_five_sets_game_over() {
    let mut s = GameState::new(800.0, 600.0);
    s.stage = 5;
    let mut events = Vec::new();
    s.next_stage(&mut events);
    assert!(s.is_game_over);
    assert_eq!(s.stage, 5);
    assert!(events.contains(&GameEvent::GameOver));
}

#[test]
fn prev_stage_moves_back_and_randomizes() {
    let mut s = GameState::new(1000.0, 800.0);
    s.stage = 3;
    let mut events = Vec::new();
    s.prev_stage(&mut events);
    assert_eq!(s.stage, 2);
    assert!(s.is_transitioning);
    assert!(events.contains(&GameEvent::StageChanged { from: 3, to: 2 }));
    assert_approx(speed_of(&s), stage_speed(2), 1e-6);
}

#[test]
fn prev_stage_from_one_is_noop() {
    let mut s = GameState::new(800.0, 600.0);
    s.stage = 1;
    let mut events = Vec::new();
    s.prev_stage(&mut events);
    assert_eq!(s.stage, 1);
    assert!(events.is_empty());
}

#[test]
fn reset_restores_initial_state() {
    let mut s = GameState::new(800.0, 600.0);
    s.stage = 5;
    s.stage_elapsed = 100.0;
    s.is_game_over = true;
    s.paused = true;
    s.is_transitioning = false;
    s.reset(1280.0, 720.0);
    assert_eq!(s.stage, 1);
    assert_eq!(s.stage_elapsed, 0.0);
    assert!(!s.is_game_over);
    assert!(!s.paused);
    assert!(s.is_transitioning);
    assert_approx(s.ball.screen_w, 1280.0, 1e-9);
    assert_approx(s.ball.radius, 720.0 / 22.5, 1e-9);
    assert_approx(speed_of(&s), stage_speed(1), 1e-6);
}

#[test]
fn resize_updates_ball_screen_size() {
    let mut s = GameState::new(800.0, 600.0);
    s.resize(1600.0, 1200.0);
    assert_approx(s.ball.screen_w, 1600.0, 1e-9);
    assert_approx(s.ball.screen_h, 1200.0, 1e-9);
    assert_approx(s.ball.radius, 1200.0 / 22.5, 1e-9);
}

#[test]
fn stage2_update_runs() {
    let mut s = playable();
    s.stage = 2;
    s.ball.set_speed(stage_speed(2), Some((0.0, 1.0)));
    let update = s.update(0.1).unwrap();
    assert_approx(s.stage_elapsed, 0.1, 1e-9);
    assert!(update.events.is_empty());
}

#[test]
fn stage3_forces_horizontal_then_vertical() {
    let mut s = playable();
    s.stage = 3;
    s.stage_elapsed = 1.0; // 前半程：水平
    s.ball.vx = stage_speed(3);
    s.ball.vy = 0.0;
    s.update(0.1).unwrap();
    assert_approx(s.ball.y, 400.0, 1e-9);
    assert_eq!(s.ball.vy, 0.0);
    assert_approx(s.ball.vx.abs(), stage_speed(3), 1e-6);

    // 后半程：垂直，速度 x0.7，x 强制居中
    s.stage_elapsed = 30.0;
    s.ball.vy = 1.0;
    s.update(0.1).unwrap();
    assert_approx(s.ball.x, 500.0, 1e-9);
    assert_eq!(s.ball.vx, 0.0);
    assert_approx(s.ball.vy.abs(), stage_speed(3) * 0.7, 1e-6);
}

#[test]
fn stage4_bounce_preserves_speed() {
    let mut s = playable();
    s.stage = 4;
    s.ball.set_speed(1500.0, Some((1.0, 0.0)));
    s.ball.x = 1000.0 - s.ball.radius - 0.1; // 贴近右壁，确保本帧反弹
    let before = speed_of(&s);
    let update = s.update(0.01).unwrap();
    assert!(update.events.contains(&GameEvent::BallBounced));
    assert_approx(speed_of(&s), before, 1e-6); // 随机偏移只改角度，不改速率
}

#[test]
fn stage5_orbit_stays_on_screen_and_pause_window_works() {
    let mut s = playable();
    s.stage = 5;
    s.update(0.1).unwrap();
    assert!(s.ball.x.is_finite() && s.ball.y.is_finite());
    assert!(s.ball.x >= s.ball.radius && s.ball.x <= s.ball.screen_w - s.ball.radius);
    assert!(s.ball.y >= s.ball.radius && s.ball.y <= s.ball.screen_h - s.ball.radius);

    // 进入 22.5s（<22.6s）暂停窗口
    s.stage_elapsed = 22.55;
    s.stage5_paused = false;
    s.update(0.0).unwrap();
    assert!(s.stage5_paused);

    // 暂停累计 >=0.3s 后自动恢复运动
    s.stage5_pause_elapsed = 0.0;
    s.update(0.3).unwrap();
    assert!(!s.stage5_paused);
}
