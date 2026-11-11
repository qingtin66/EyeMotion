//! 输入参数校验与规范化工具。
//!
//! 在命令边界处统一校验来自前端的尺寸与时间步长，避免非法数值进入物理逻辑。

use crate::{CoreError, Result};

/// 校验屏幕尺寸：宽高必须为正的有限数值，否则返回错误。
pub fn validate_dimensions(width: f64, height: f64) -> Result<()> {
    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
        return Err(CoreError::Config(format!(
            "invalid dimensions: width={width}, height={height}"
        )));
    }
    Ok(())
}

/// 规范化单帧时间步长：拒绝非有限或负值，并把结果夹取到 [0, max_dt]。
pub fn sanitize_dt(dt: f64, max_dt: f64) -> Result<f64> {
    if !dt.is_finite() || dt < 0.0 {
        return Err(CoreError::Physics(format!("invalid dt: {dt}")));
    }
    if !max_dt.is_finite() || max_dt <= 0.0 {
        return Err(CoreError::Config(format!("invalid max_dt: {max_dt}")));
    }
    Ok(dt.min(max_dt))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_dimensions_ok() {
        assert!(validate_dimensions(800.0, 600.0).is_ok());
    }

    #[test]
    fn zero_width_rejected() {
        assert!(validate_dimensions(0.0, 600.0).is_err());
    }

    #[test]
    fn nan_dimension_rejected() {
        assert!(validate_dimensions(f64::NAN, 600.0).is_err());
    }

    #[test]
    fn negative_height_rejected() {
        assert!(validate_dimensions(800.0, -1.0).is_err());
    }

    #[test]
    fn dt_zero_ok() {
        assert_eq!(sanitize_dt(0.0, 0.1).unwrap(), 0.0);
    }

    #[test]
    fn dt_is_clamped() {
        assert_eq!(sanitize_dt(1.0, 0.1).unwrap(), 0.1);
    }

    #[test]
    fn negative_dt_rejected() {
        assert!(sanitize_dt(-0.1, 0.1).is_err());
    }

    #[test]
    fn nan_dt_rejected() {
        assert!(sanitize_dt(f64::NAN, 0.1).is_err());
    }
}
