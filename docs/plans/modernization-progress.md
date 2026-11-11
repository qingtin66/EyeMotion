# EyeMotion 现代化执行进度

> 依据：`docs/plans/modernization-plan.md`（用户已确认 §8 六决策点，按「Phase 0 + 1 先行」执行）
> 最后更新：2026-11-11

## 总体状态

| 阶段 | 状态 |
| --- | --- |
| Phase 0 止血（P0-1..8） | ✅ 完成 |
| Phase 1 质量门禁（Q-1..Q-8） | ✅ 完成（Q-7 withGlobalTauri=false 按计划后置为独立 PR；Q-8 Playwright 冒烟为可选项未做） |
| Phase 2 架构 SSOT（A-1..A-6） | ✅ 完成（提交 e2aacb0） |
| Phase 3 仓库卫生（H-1..H-5） | ✅ 完成（H-1/H-2/H-3/H-4/H-5 全部落地） |
| Phase 4 依赖与平台（D-1..D-4） | ⬜ 未开始（视情况） |

评审：reviewer 最终只读验收 task-8 = **PASS**（0 阻断项）。

## 提交

- `536c85e` fix(p0): 止血——版本统一、错误处理、安全基线、命令校验、构建可复现
- `5a181c8` chore(ci): Phase1 质量门禁基建——工具链钉桩、ESLint/Prettier、Vitest、CI 与依赖审计
- `d7fefd2` docs(plans): 记录 Phase0+1 执行进度、门禁实测与豁免项
- `e2aacb0` refactor(phase2): 架构 SSOT——主题单一来源、状态权威归后端、命令模块化、死代码清理

## 门禁实测（本地全绿，reviewer 独立复跑一致）

| 检查 | 结果 |
| --- | --- |
| `npm run check:version` | ✅ 三处 1.1.2 一致 |
| `npm run typecheck` | ✅ 0 error |
| `npm run lint` | ✅ 0 error |
| `npm run format:check` | ✅ All matched |
| `npm test` | ✅ 21 passed（i18n 15 + audio 6） |
| `npm run test:coverage` | ✅ 行 98.34% / 分支 91.86% / 函数 96.77% |
| `npm run build` | ✅ vite build 成功 |
| `cargo fmt --all -- --check` | ✅ clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ 0 告警 |
| `cargo test --workspace` | ✅ 54 passed（core 15 unit + config_contract 12 + game_state 16 + physics 11） |
| 干净临时目录 `npm ci` | ✅ exit 0（可复现） |
| `npm audit --omit=dev --audit-level=high` | ✅ 0 vulnerabilities |

## 关键实现与决策

- **版本 SSOT**：统一为 1.1.2；新增 `scripts/check-version.mjs` 校验 Cargo.toml / package.json / tauri.conf.json 一致，CI 强制执行。
- **错误处理**：`src-tauri/src/lib.rs` 移除 `.expect`，改为 `log::error!` + 非零退出；commands 锁错误改 `log::warn!`。
- **参数校验**：新增 `eyemotion-core/src/validation.rs`（`validate_dimensions` / `sanitize_dt`，dt clamp 到 `[0, MAX_DT=0.1]`），tick/reset_game/resize_game 接入并返回 `Result`。
- **安全基线**：`tauri.conf.json` 设置非 null CSP；`capabilities/default.json` 收紧为 core 最小集（移除 `shell:allow-open`、`window-state:default`）。
- **工具链钉桩**：`rust-toolchain.toml` channel=`stable`（=当前 1.92.0，避免与 CI 双轨下载）+ `Cargo.toml` `rust-version="1.92"`；`.nvmrc`/`.node-version`=20。
- **CI 门禁**（`.github/workflows/ci.yml`）：quality job（npm ci → check:version/typecheck/lint/format/test/build + cargo fmt/clippy/test）+ security job（rustsec + `npm audit --omit=dev --audit-level=high`）；`dependabot.yml` 跟踪 cargo/npm/actions。

## 已知豁免与遗留（非阻断，记录在案）

1. **CSP 仍含 `script-src/style-src 'unsafe-inline'`**：因本环境无法运行时验证 GUI（桌面自动化受限），留待 Q-7 独立 PR 以 nonce/hash 收紧。
2. **`withGlobalTauri=true` 仍在**：按计划 Q-7 改 ES import，独立 PR 处理。
3. **npm 全量审计 devDeps 20 项**（5 high / 3 critical，均在 vite/rollup/postcss/vitest/tinypool 等构建/测试链，不随包发布；运行时 `@tauri-apps/api`=0）。建议 Phase 4 升 vite/vitest 大版本 + dependabot 跟踪。
4. **Rust 覆盖率未入 CI**：本机无 `cargo-llvm-cov`（profile=minimal 缺 llvm-tools-preview）。建议 CI 加 `llvm-tools-preview` + `cargo llvm-cov -p eyemotion-core --fail-under-lines 60`（src-tauri 无测试不计入）。
5. **前端覆盖率口径**：当前仅统计 i18n/audio 纯逻辑（renderer/game/input/virtual-controls 依赖 Canvas/DOM 未纳入），后续补 Playwright 冒烟或 canvas mock。
6. **其他增强**：eslint 可加 `--max-warnings=0`；release.yml 可加缓存；公共 API 中文 `///` 100% 属 Phase 2 (A-4)。

## Phase 2 架构 SSOT（✅ 完成，e2aacb0）

- **A-1 主题单一来源**：`visual_config.rs` 新增 `GradientStyle` 与三个按钮渐变；`Theme::default()` 品牌化（#0B1B7D/#153FA1、球体 #FC6170→#E62538、radius_ratio 0.05）；前端 `renderer.ts` 删除 `getDefaultTheme()`，全部颜色取自后端主题 / CSS `--brand-*` 变量。
- **A-2 状态 SSOT**：前端 `game.ts` 不再直接写权威状态；新增后端命令 `start_game` 迁移逻辑与 `return_to_start_screen`；`quitGame`/restart/togglePause 仅调命令。
- **A-4 命令模块化 + 中文文档**：`commands.rs` 拆为 `commands/{mod,game,window,config,theme}.rs`，全部 pub fn 中文 `///`。
- **A-3/A-5 死代码清理**：删除 Rust 死配置 `audio_config.rs`、前端死入口 `ui/js/main.js`、死依赖 `tauri-plugin-window-state`；`UserConfig` 移除未消费的 `audio` 字段。
- **A-6**：`lib.rs` invoke_handler 注册 `return_to_start_screen`。

## Phase 3 仓库卫生（✅ 完成）

- **H-1**：`git rm --cached` 并删除入库的 `release/EyeMotion_Portable_Green.zip` 与重复的 `src-tauri/Cargo.lock`；`.gitignore` 增加 `release/*.{zip,exe,msi,apk}` 与 `src-tauri/Cargo.lock`。
- **H-2**：备份 10 个旧客户端点目录（.agent/.claude/.codex/.cursor/.gemini/.kiro/.roo/.shared/.trae/.windsurf，约 95 文件 ~1.1MB）到仓库外 `D:\Soft\eyemotion-legacy-backup\legacy-clients-2026-11-11.zip`（408,076 bytes）后删除；保留 .workbuddy（harness 使用）。
- **H-3**：删除 `patch_config.js`/`restore_config.js`，并从 `build_all.ps1` 移除 patch/restore 步骤（Tauri v2 原生支持 `build.devUrl`）。
- **H-4**：新增 `CONTRIBUTING.md`、`.github/PULL_REQUEST_TEMPLATE.md`、`.github/ISSUE_TEMPLATE/{bug_report,feature_request}.md + config.yml`、`product_context.md`、`todo.md`；修复 README 下载徽章错链为 `qingtin66/EyeMotion`；`multi_platform.md` 增补「现代化基线」小节。
- **H-5**：`README_ZH.md` 版本信息对齐 SSOT 1.1.2 / 2026-03-01。

## 下一步

1. Phase 3 全量门禁复跑 + 提交（本阶段）。
2. 可选发布检查点：版本 1.1.3（CHANGELOG + tag + push）。
3. Phase 4（视情况）：vite/vitest 升级、macOS CI job、Android 规范化、cargo-llvm-cov 覆盖率门禁、Q-7 withGlobalTauri=false + CSP 收紧。

## 构建 / 审核 / 测试记录（HEAD=6e5d5a2）

**构建**
- `npm run build` ✅ vite 5.4.21，12 modules，dist 产物正常
- `cargo build --workspace` ✅ dev profile
- `npx tauri build --debug --no-bundle` ✅ 产物 `target\debug\eyemotion.exe`（18,208,768 bytes）
- 已知 warning：bin 与 lib 同名产出 `eyemotion.pdb`（cargo#6313），未来可能变 hard error；建议后续给 src-tauri 的 lib 改名。

**独立测试（tester, task-10）**
- npm test 21 passed；覆盖率 语句/行 98.34%、分支 91.86%、函数 96.77%
- cargo fmt --check clean；clippy -D warnings exit0；cargo test --workspace 54 passed
- check:version 1.1.2；工作树干净

**独立审核（reviewer, task-9）**
- 判定 **PASS（0 阻断项）**：Phase2 A-1..A-6、Phase3 H-1..H-5 实质达成
- 非阻断项处理：`state.rs` AppState / `lib.rs` run() 补中文 `///`；CONNECTING 文本改用主题色；停止跟踪 Android 生成产物（`src-tauri/gen/android/app/src/main/assets/` 已加入 .gitignore）→ 提交 `6e5d5a2`
- 未处理（记录）：`.github/prompts/ui-ux-pro-max.prompt.md` 引用已删旧路径；`style.css` 静态品牌回退色（已有注释说明，仅首帧回退用途）；`rust-toolchain.toml` 用 stable（当前=1.92.x）

