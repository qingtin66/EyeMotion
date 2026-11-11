#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub background: BackgroundStyle,
    pub ball: BallStyle,
    pub ui: UIStyle,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackgroundStyle {
    pub grid_color_dark: Color,
    pub grid_color_light: Color,
    pub grid_size: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BallStyle {
    pub gradient_start: Color,
    pub gradient_end: Color,
    pub outline_color: Color,
    pub radius_ratio: f64,
}

/// 双色线性渐变，用于按钮等需要渐变填充的 UI 元素。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GradientStyle {
    pub start: Color,
    pub end: Color,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UIStyle {
    pub title_color: Color,
    pub subtitle_color: Color,
    pub stats_color: Color,
    pub button_color: Color,
    pub button_hover_color: Color,
    pub background_color: Color,
    /// 主按钮渐变（START / QUIT 等）。
    pub primary_button: GradientStyle,
    /// 次按钮渐变（RESTART 等）。
    pub secondary_button: GradientStyle,
    /// 强调按钮渐变（RESUME / PAUSE / STAGE 标题等）。
    pub accent_button: GradientStyle,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// 品牌主题的唯一来源（Rust 端 SSOT），前端只消费不硬编码。
impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: BackgroundStyle {
                grid_color_dark: Color {
                    r: 11,
                    g: 27,
                    b: 125,
                }, // #0B1B7D
                grid_color_light: Color {
                    r: 21,
                    g: 63,
                    b: 161,
                }, // #153FA1
                grid_size: 80.0,
            },
            ball: BallStyle {
                gradient_start: Color {
                    r: 252,
                    g: 97,
                    b: 112,
                }, // #FC6170
                gradient_end: Color {
                    r: 230,
                    g: 37,
                    b: 56,
                }, // #E62538
                outline_color: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                radius_ratio: 0.05,
            },
            ui: UIStyle {
                title_color: Color {
                    r: 235,
                    g: 191,
                    b: 66,
                }, // #EBBF42
                subtitle_color: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                stats_color: Color {
                    r: 64,
                    g: 197,
                    b: 239,
                }, // #40C5EF
                button_color: Color {
                    r: 252,
                    g: 97,
                    b: 112,
                }, // #FC6170
                button_hover_color: Color {
                    r: 230,
                    g: 37,
                    b: 56,
                }, // #E62538
                background_color: Color {
                    r: 11,
                    g: 27,
                    b: 125,
                }, // #0B1B7D
                primary_button: GradientStyle {
                    start: Color {
                        r: 252,
                        g: 97,
                        b: 112,
                    }, // #FC6170
                    end: Color {
                        r: 230,
                        g: 37,
                        b: 56,
                    }, // #E62538
                },
                secondary_button: GradientStyle {
                    start: Color {
                        r: 235,
                        g: 191,
                        b: 66,
                    }, // #EBBF42
                    end: Color {
                        r: 230,
                        g: 131,
                        b: 37,
                    }, // #E68325
                },
                accent_button: GradientStyle {
                    start: Color {
                        r: 65,
                        g: 198,
                        b: 240,
                    }, // #41C6F0
                    end: Color {
                        r: 16,
                        g: 143,
                        b: 223,
                    }, // #108FDF
                },
            },
        }
    }
}
