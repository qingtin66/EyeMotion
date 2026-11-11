# EyeMotion 产品背景（Product Context）

> 本文件是 AI Agent 与开发者进入项目时的第一上下文入口，说明“为什么做、做什么、边界在哪”。
> 与 `AGENTS.md`（行为规范）、`docs/plans/modernization-plan.md`（改造计划）配套阅读。

## 1. 产品定位
EyeMotion 是一款面向眼健康的高性能**动态视觉追踪训练**桌面应用：通过线性/圆周等轨迹运动的球体，训练用户的平滑追视（smooth pursuit）、注视稳定性与动态视觉捕捉能力，缓解长时间屏幕使用带来的视疲劳。

- 非医疗器械：仅作视觉辅助训练工具，不替代专业诊疗（README 已含免责声明）。
- 目标平台：Windows 10/11 为主，macOS / Linux 桌面次之；**桌面-only**（移动端已按 2026-11-11 决策移除，见 `docs/decisions/ADR-0001-desktop-only-platform.md`）。
- 交互极简：仅鼠标；全屏沉浸；高刷新率（240–540Hz）友好。

## 2. 核心玩法与状态机
- 训练分 5 个 stage（阶段），每阶段限时约 45 秒，到期自动进入下一阶段；第 5 阶段结束后进入 Game Over。
- 阶段差异体现在球速（stage_speed）与运动方向（stage_direction，随机角度），配置在 `eyemotion-core` 的 `stage_config` / `game_state`。
- 游戏状态机字段（`GameState`）：`is_start_screen`（开始屏）、`is_transitioning` + `transition_timer`（阶段切换倒计时）、`paused`、`is_game_over`、`stage`、`stage_elapsed`、`ball`。
- **状态权威归后端**：所有状态迁移由 Rust 命令完成（tick/reset_game/start_game/toggle_pause/next_stage/prev_stage/return_to_start_screen）；前端只渲染 tick 返回的快照，不直接写权威字段。

## 3. 架构分层（Phase 2 后）
- `eyemotion-core`：纯逻辑库（无 Tauri 依赖）——`game_state`（状态机/物理）、`ball`（球体）、`visual_config`（主题 SSOT）、`config`（用户配置持久化）、`stage_config`、`validation`（入参校验）、`error`、`events`。
- `src-tauri`：Tauri v2 应用壳——`commands/{game,window,config,theme}.rs` 命令模块、`state.rs`（AppState：Mutex<GameState> + Mutex<UserConfig>）、`lib.rs`（Builder/插件/invoke_handler）。
- `ui/`：TypeScript + Canvas2D 前端——`game.ts`（主循环/事件）、`renderer.ts`（绘制，主题来自后端）、`bridge.ts`（命令封装）、`audio.ts`（WebAudio，音频唯一真实实现）、`i18n.ts`、`input.ts`、`language-menu.ts`。

## 4. 主题与品牌（SSOT）
- 品牌主色：`#0B1B7D`（深蓝背景）、`#153FA1`（网格浅色）。
- 主题单一来源：`eyemotion-core/src/visual_config.rs` 的 `Theme::default()`；前端通过 `get_theme` 命令获取，禁止在 UI 中硬编码颜色字面量（CSS 侧统一用 `--brand-*` 变量）。
- 三个按钮渐变：primary `#FC6170→#E62538`、secondary `#EBBF42→#E68325`、accent `#41C6F0→#108FDF`。

## 5. 版本与发布
- 版本 SSOT：根 `Cargo.toml` 的 `workspace.package.version`，并由 `scripts/check-version.mjs` 强制校验 `package.json` / `src-tauri/tauri.conf.json` 三处一致。
- 发布由 `v*` tag 触发 GitHub Actions；当前目标版本 1.1.2。

## 6. 硬约束
- 生产代码禁止 `unwrap()`/`expect()`；错误用 `Result` + `thiserror`/`log`。
- 禁止硬编码系统路径；用 `PathBuf`/`join` 与标准目录 API。
- 公共 API 必须有中文 `///` 文档注释；提交遵循 Conventional Commits。
- 质量门禁：cargo fmt/clippy(-D warnings)/test、tsc/lint/prettier/vitest、覆盖率行 ≥60%（详见 CI）。
