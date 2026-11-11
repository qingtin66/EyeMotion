//! eyemotion-core 物理层集成测试。
//!
//! 覆盖 `Ball` 的构造、速度归一化、四向边界反弹、屏幕缩放、复位与随机重置不变量，
//! 以及圆周运动的轨道半径守恒。随机重置只断言不变量（位置落在安全边距内、速度清零），
//! 因此对任意随机输出都成立，不依赖固定种子也不会 flaky。

use eyemotion_core::Ball;

fn assert_approx(actual: f64, expected: f64, eps: f64) {
    assert!(
        (actual - expected).abs() <= eps,
        "expected {expected} ± {eps}, got {actual}"
    );
}

#[test]
fn new_uses_height_based_radius_and_centers() {
    let ball = Ball::new(1000.0, 800.0);
    // 半径以屏幕高度为基准：800 / 22.5
    assert_approx(ball.radius, 800.0 / 22.5, 1e-9);
    assert_approx(ball.x, 500.0, 1e-9);
    assert_approx(ball.y, 400.0, 1e-9);
    assert_eq!((ball.vx, ball.vy), (0.0, 0.0));
}

#[test]
fn set_speed_normalizes_direction() {
    let mut ball = Ball::new(1000.0, 800.0);
    ball.set_speed(100.0, Some((3.0, 4.0))); // 3-4-5 直角三角形
    assert_approx(ball.vx, 60.0, 1e-9);
    assert_approx(ball.vy, 80.0, 1e-9);
    assert_approx((ball.vx * ball.vx + ball.vy * ball.vy).sqrt(), 100.0, 1e-9);

    // 保持方向重新设定速度大小
    ball.set_speed(200.0, None);
    assert_approx((ball.vx * ball.vx + ball.vy * ball.vy).sqrt(), 200.0, 1e-9);
    assert_approx(ball.vx / ball.vy, 60.0 / 80.0, 1e-9);
}

#[test]
fn set_speed_ignores_zero_direction() {
    let mut ball = Ball::new(1000.0, 800.0);
    ball.set_speed(100.0, Some((0.0, 0.0)));
    assert_eq!((ball.vx, ball.vy), (0.0, 0.0));
}

#[test]
fn update_bounces_on_all_four_walls() {
    // 右壁：反弹后 vx 变负
    let mut right = Ball::new(100.0, 100.0);
    right.x = 99.0;
    right.y = 50.0;
    right.vx = 50.0;
    right.vy = 0.0;
    assert!(right.update(1.0).unwrap());
    assert!(right.vx < 0.0);

    // 左壁：反弹后 vx 变正
    let mut left = Ball::new(100.0, 100.0);
    left.x = 1.0;
    left.y = 50.0;
    left.vx = -50.0;
    left.vy = 0.0;
    assert!(left.update(1.0).unwrap());
    assert!(left.vx > 0.0);

    // 底壁：反弹后 vy 变负
    let mut bottom = Ball::new(100.0, 100.0);
    bottom.x = 50.0;
    bottom.y = 99.0;
    bottom.vx = 0.0;
    bottom.vy = 50.0;
    assert!(bottom.update(1.0).unwrap());
    assert!(bottom.vy < 0.0);

    // 顶壁：反弹后 vy 变正
    let mut top = Ball::new(100.0, 100.0);
    top.x = 50.0;
    top.y = 1.0;
    top.vx = 0.0;
    top.vy = -50.0;
    assert!(top.update(1.0).unwrap());
    assert!(top.vy > 0.0);
}

#[test]
fn update_without_collision_returns_false() {
    let mut ball = Ball::new(1000.0, 800.0);
    ball.set_speed(10.0, Some((1.0, 0.0)));
    assert!(!ball.update(0.001).unwrap());
}

#[test]
fn update_screen_size_scales_position_and_radius() {
    let mut ball = Ball::new(1000.0, 800.0); // 中心 500,400
    ball.update_screen_size(2000.0, 1600.0); // 两轴均 x2
    assert_approx(ball.radius, 1600.0 / 22.5, 1e-9);
    assert_approx(ball.x, 1000.0, 1e-9);
    assert_approx(ball.y, 800.0, 1e-9);
}

#[test]
fn update_screen_size_clamps_position_inside_bounds() {
    let mut ball = Ball::new(1000.0, 800.0);
    ball.x = 990.0;
    ball.y = 790.0;
    ball.update_screen_size(400.0, 300.0); // 缩小后必须夹回边界内
    assert!(ball.x <= 400.0 - ball.radius + 1e-9);
    assert!(ball.x >= ball.radius - 1e-9);
    assert!(ball.y <= 300.0 - ball.radius + 1e-9);
    assert!(ball.y >= ball.radius - 1e-9);
}

#[test]
fn reset_centers_ball_and_clears_velocity() {
    let mut ball = Ball::new(1000.0, 800.0);
    ball.x = 10.0;
    ball.y = 10.0;
    ball.vx = 5.0;
    ball.vy = 5.0;
    ball.reset(800.0, 600.0);
    assert_approx(ball.x, 400.0, 1e-9);
    assert_approx(ball.y, 300.0, 1e-9);
    assert_eq!((ball.vx, ball.vy), (0.0, 0.0));
    assert_approx(ball.radius, 600.0 / 22.5, 1e-9);
}

#[test]
fn random_reset_stays_inside_safe_margins() {
    let mut ball = Ball::new(1920.0, 1080.0);
    let r = 1080.0 / 22.5;
    let (w, h) = (1920.0, 1080.0);
    for _ in 0..50 {
        ball.reset_to_random_pos(w, h);
        // gen_range 区间 [r+50, w-r-50)：任意随机结果都必须落在安全边距内
        assert!(
            ball.x >= r + 50.0 - 1e-9 && ball.x < w - r - 50.0 + 1e-9,
            "x={} out of safe range",
            ball.x
        );
        assert!(
            ball.y >= r + 50.0 - 1e-9 && ball.y < h - r - 50.0 + 1e-9,
            "y={} out of safe range",
            ball.y
        );
        assert_eq!((ball.vx, ball.vy), (0.0, 0.0));
    }
}

#[test]
fn circular_update_with_radius_preserves_orbit_radius() {
    let mut ball = Ball::new(800.0, 600.0);
    ball.update_circular_with_radius(0.1, 1.2, 200.0);
    let dist = ((ball.x - 400.0f64).powi(2) + (ball.y - 300.0f64).powi(2)).sqrt();
    assert_approx(dist, 200.0, 1e-6);
}

#[test]
fn circular_update_keeps_existing_radius() {
    let mut ball = Ball::new(800.0, 600.0);
    ball.x = 600.0;
    ball.y = 300.0; // 距中心 200
    ball.update_circular(0.5, 1.0);
    let dist = ((ball.x - 400.0f64).powi(2) + (ball.y - 300.0f64).powi(2)).sqrt();
    assert_approx(dist, 200.0, 1e-6);
}
