//! eyemotion-core 配置/常量/错误契约集成测试。
//!
//! 覆盖关卡速度/方向常量、UserConfig 默认值与单调更新、StageConfig 默认值、
//! Theme 默认配色、GameUpdate 默认值，以及 CoreError 的错误文案与校验边界。

use eyemotion_core::{
    sanitize_dt, stage_direction, stage_speed, validate_dimensions, Axis, CoreError, GameUpdate,
    MotionType, StageConfig, Theme, UserConfig,
};

#[test]
fn stage_speed_matches_specification() {
    assert_eq!(stage_speed(1), 1000.0);
    assert_eq!(stage_speed(2), 1100.0);
    assert_eq!(stage_speed(3), 1625.0);
    assert_eq!(stage_speed(4), 1500.0);
    assert_eq!(stage_speed(5), 0.0);
    // 未知关卡回退到默认速度
    assert_eq!(stage_speed(99), 1000.0);
}

#[test]
fn stage_direction_is_unit_length_for_random_stages() {
    for stage in [1, 2, 4, 5] {
        for _ in 0..100 {
            let (x, y) = stage_direction(stage);
            let mag = (x * x + y * y).sqrt();
            assert!((mag - 1.0).abs() < 1e-9, "stage {stage} magnitude {mag}");
        }
    }
}

#[test]
fn stage_direction_stage3_is_axis_aligned() {
    for _ in 0..50 {
        let (x, y) = stage_direction(3);
        assert_eq!(y, 0.0);
        assert!(x == 1.0 || x == -1.0, "x={x}");
    }
}

#[test]
fn stage_direction_unknown_falls_back() {
    assert_eq!(stage_direction(99), (1.0, 1.0));
}

#[test]
fn stage_direction_angle_ranges() {
    for _ in 0..100 {
        let (x, y) = stage_direction(1);
        let angle = y.atan2(x.abs()).to_degrees();
        assert!(
            (10.0 - 1e-6..=20.0 + 1e-6).contains(&angle),
            "stage1 angle {angle}"
        );
    }
    for _ in 0..100 {
        let (x, y) = stage_direction(2);
        let angle = x.abs().atan2(y.abs()).to_degrees();
        assert!(
            (7.0 - 1e-6..=17.0 + 1e-6).contains(&angle),
            "stage2 angle {angle}"
        );
    }
    for _ in 0..100 {
        let (x, y) = stage_direction(4);
        let angle = x.abs().atan2(y.abs()).to_degrees();
        assert!(
            (20.0 - 1e-6..=70.0 + 1e-6).contains(&angle),
            "stage4 angle {angle}"
        );
    }
}

#[test]
fn user_config_defaults() {
    let c = UserConfig::default();
    assert_eq!(c.language, "en");
    assert_eq!(c.last_played_stage, 1);
}

#[test]
fn update_last_stage_only_moves_forward() {
    let mut c = UserConfig::default();
    c.update_last_stage(3);
    assert_eq!(c.last_played_stage, 3);
    c.update_last_stage(2); // 回退不覆盖
    assert_eq!(c.last_played_stage, 3);
    c.update_last_stage(5);
    assert_eq!(c.last_played_stage, 5);
}

#[test]
fn stage_config_default() {
    let sc = StageConfig::default();
    assert_eq!(sc.speed, 1000.0);
    assert_eq!(sc.angle_range, (10.0, 20.0));
    match sc.motion_type {
        MotionType::Linear { primary_axis } => {
            assert!(matches!(primary_axis, Axis::Horizontal));
        }
        _ => panic!("默认运动类型应为水平线性"),
    }
}

#[test]
fn theme_defaults() {
    let t = Theme::default();
    // 品牌主题（#0B1B7D / #153FA1）为唯一来源。
    assert_eq!(
        (
            t.background.grid_color_dark.r,
            t.background.grid_color_dark.g,
            t.background.grid_color_dark.b
        ),
        (11, 27, 125)
    );
    assert_eq!(
        (
            t.background.grid_color_light.r,
            t.background.grid_color_light.g,
            t.background.grid_color_light.b
        ),
        (21, 63, 161)
    );
    assert_eq!(t.background.grid_size, 80.0);
    assert!((t.ball.radius_ratio - 0.05).abs() < 1e-12);
    assert_eq!(
        (
            t.ui.button_color.r,
            t.ui.button_color.g,
            t.ui.button_color.b
        ),
        (252, 97, 112)
    );
    // 按钮渐变随主题统一下发
    assert_eq!(
        (
            t.ui.primary_button.start.r,
            t.ui.primary_button.start.g,
            t.ui.primary_button.start.b
        ),
        (252, 97, 112)
    );
    assert_eq!(
        (
            t.ui.primary_button.end.r,
            t.ui.primary_button.end.g,
            t.ui.primary_button.end.b
        ),
        (230, 37, 56)
    );
}

#[test]
fn game_update_default_is_empty() {
    let u = GameUpdate::default();
    assert_eq!(u.time_elapsed, 0.0);
    assert!(u.events.is_empty());
}

#[test]
fn core_error_display_messages() {
    assert_eq!(
        CoreError::Physics("boom".to_string()).to_string(),
        "Physics error: boom"
    );
    assert_eq!(
        CoreError::Config("bad".to_string()).to_string(),
        "Configuration error: bad"
    );
    assert_eq!(CoreError::InvalidStage(7).to_string(), "Invalid stage: 7");
    assert_eq!(
        CoreError::InvalidTransition.to_string(),
        "Invalid state transition"
    );
}

#[test]
fn validation_boundaries() {
    assert!(validate_dimensions(1.0, 1.0).is_ok());
    assert!(validate_dimensions(f64::INFINITY, 600.0).is_err());
    assert!(validate_dimensions(800.0, f64::NAN).is_err());
    assert_eq!(sanitize_dt(0.5, 1.0).unwrap(), 0.5);
    assert_eq!(sanitize_dt(1.0, 0.1).unwrap(), 0.1);
    assert!(sanitize_dt(1.0, 0.0).is_err()); // 非法 max_dt
    assert!(sanitize_dt(f64::NAN, 0.1).is_err());
    assert!(sanitize_dt(-0.01, 0.1).is_err());
}
