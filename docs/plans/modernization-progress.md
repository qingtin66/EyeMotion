# EyeMotion 现代化执行进度

> 依据：`docs/plans/modernization-plan.md`（用户已确认 §8 六决策点，按「Phase 0 + 1 先行」执行）
> 最后更新：2026-11-11
>
> **2026-11-11 范围变更**：用户决定移除移动端，项目改为桌面-only（Windows/macOS/Linux），执行记录见文末「桌面-only 收敛（v0.3 平台范围变更）」。

## 总体状态

| 阶段 | 状态 |
| --- | --- |
| Phase 0 止血（P0-1..8） | ✅ 完成 |
| Phase 1 质量门禁（Q-1..Q-8） | ✅ 完成（Q-7 已于 Phase 4 落地；Q-8 Playwright 冒烟按 D-4 评估暂缓） |
| Phase 2 架构 SSOT（A-1..A-6） | ✅ 完成（提交 e2aacb0） |
| Phase 3 仓库卫生（H-1..H-5） | ✅ 完成（H-1/H-2/H-3/H-4/H-5 全部落地） |
| Phase 4 依赖与平台（D-1..D-4） | ✅ 完成（本地门禁全绿；macOS CI 构建通过；RustSec 5 项传递依赖漏洞已修复） |
| Phase 5 桌面-only 收敛（v0.3） | 🚧 实施完成，待 reviewer/tester 独立验收 |

评审：reviewer 最终只读验收 task-8 = **PASS**（0 阻断项）。

## 提交

- `536c85e` fix(p0): 止血——版本统一、错误处理、安全基线、命令校验、构建可复现
- `5a181c8` chore(ci): Phase1 质量门禁基建——工具链钉桩、ESLint/Prettier、Vitest、CI 与依赖审计
- `d7fefd2` docs(plans): 记录 Phase0+1 执行进度、门禁实测与豁免项
- `e2aacb0` refactor(phase2): 架构 SSOT——主题单一来源、状态权威归后端、命令模块化、死代码清理
- `6e43198` chore(phase4): 依赖与平台现代化——Rust/前端升级、macOS/Android CI、Q-7 安全收尾
- `aa3b4b1` fix(security): 修复 RustSec 传递依赖漏洞与 CI security 权限
- `9a77a4c` docs(plans): 记录 Phase 4 安全门禁修复与独立验证结果

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


## Phase 4 依赖与平台（✅ 完成，本轮）

### D-1 依赖升级（Rust + 前端）

- **Rust**：`Cargo.toml` 升级 `rand = "0.9"`、`thiserror = "2.0"`；锁定版本 rand 0.9.5 / rand_chacha 0.9.0 / rand_core 0.9.5 / thiserror 2.0.17。迁移点：`eyemotion-core/src/ball.rs:34`、`eyemotion-core/src/game_state.rs:107,229` 的 `thread_rng()` → `rng()`；`ball.rs:40-41`、`game_state.rs:108,232,242,258` 的 `gen_range` → `random_range`；`game_state.rs:234,244,249,260,268` 的 `gen_bool` → `random_bool`。
- **前端**：`vite ^8.3.8`、`vitest ^5.1.0`、`@vitest/coverage-v8 ^5.1.0`、`jsdom ^29.1.1`；Node 24（`.nvmrc` / `.node-version` = 24，`engines.node >=22.12.0`）；`package.json` 增加 `"type": "module"`。
- **Tauri npm 与 crate 对齐（关键坑）**：crates.io 最新稳定 `tauri` crate 为 **2.9.5**，而 npm `@tauri-apps/api` 已到 2.12.4；CLI 会因 `tauri (2.9.5) : @tauri-apps/api (2.12.4)` 版本不匹配直接 exit 1。因此固定 `@tauri-apps/api = 2.9.1`、`@tauri-apps/cli = 2.9.6`（同 2.9 minor），不使用 2.12 CLI；待 crate 发布 2.12 后再同步。

### D-2 macOS 构建

- `.github/workflows/release.yml` matrix 改为 `[windows-latest, ubuntu-22.04, macos-latest]`，node 24 + `Swatinem/rust-cache@v2`；tauri-action 按平台传 `args`（ubuntu `--bundles deb,appimage`、macos `--bundles app,dmg`、windows 空）。
- 发布说明提取从 `shell: pwsh` + awk 改为跨平台 Node 脚本 `scripts/release-notes.mjs`（读取 `CHANGELOG.md` 对应版本段，写入 `GITHUB_ENV`）。
- `.github/workflows/ci.yml` 新增 `bundle-macos` job（macos-latest，`npx tauri build --no-bundle`，timeout 30m）；全部 `node-version` 由 20 → 24（ci.yml 与 release.yml）。

### D-3 Android 官方流程

- `.github/workflows/release.yml` 的 `release-android` 删除两段 python workaround（生成 `tauri.settings.gradle` / patch Gradle），改为：
  - 仅当 `src-tauri/gen/android` 不存在时执行 `npx tauri android init --ci`（本仓库该目录已入库）；
  - `npx tauri android build --apk --split-per-abi --ci`；
  - 新增 **Verify APK artifacts** 断言（`find ... -name '*.apk'` 数量 > 0）。
- Android SDK 33 → 36 / build-tools 36.0.0（与生成模板 `src-tauri/gen/android/app/build.gradle.kts:17,23` 的 compileSdk/targetSdk=36 对齐），NDK 保持 r27（27.2.12479018）。
- 风险：Tauri issue #13357（`assembleUniversalRelease`）在旧版 CLI 仍可能触发；当前钉 CLI 2.9.6，若 CI 首次 tag 构建失败，回退方案为 `--aab` / `--split-per-abi` 并记录版本。
- **签名规范化**：`src-tauri/gen/android/app/build.gradle.kts` 移除硬编码 `release.jks` 密码，改为读取 `src-tauri/gen/android/keystore.properties`（存在时用 release 签名，否则 CI 回退 debug 签名）；`git rm --cached src-tauri/gen/release.jks` 并忽略 `src-tauri/gen/*.jks` / `keystore.properties`。历史提交仍含旧密钥，建议轮换 keystore 密码。

### D-4 E2E 评估（结论：暂缓）

- 现状：前端测试仅 Vitest + jsdom 覆盖 `ui/js/i18n.ts`、`ui/js/audio.ts` 纯逻辑；`renderer.ts` / `game.ts` / `input.ts` / `virtual-controls.ts` 依赖 Canvas/DOM/Tauri IPC，未纳入。
- 评估：Playwright 对 Tauri 桌面应用需 `tauri-driver`（WebDriver），Windows/Linux 可、**macOS 不支持**，且 CI 无显示环境、WebView2 启动脆弱；纯浏览器 `vite dev` 又因 `Bridge.invoke` 在非 Tauri 环境返回 null 无法复现真实交互。
- 结论：本轮不引入 E2E；先以「三平台 `--no-bundle` 构建 + Android APK 断言」作为平台回归门禁。后续若需 E2E，前置条件为抽出 `Bridge` 可注入 mock + Canvas 渲染层可替换 headless，再评估 Playwright/tauri-driver。

### Q-7 安全收尾（随 Phase 4 落地）

- `src-tauri/tauri.conf.json`：`withGlobalTauri: true → false`；CSP `script-src 'self' 'unsafe-inline'` → `script-src 'self'`（`style-src` 保留 `'unsafe-inline'`，因运行时动态 style）。
- `ui/js/bridge.ts`：删除 `window.__TAURI__` 全局，改为 `import { invoke as tauriInvoke } from '@tauri-apps/api/core'`，以 `'__TAURI_INTERNALS__' in window` 判断后调用。
- `ui/index.html`：删除内联 `<style>` ×2 与内联 `<script>`；`ui/css/style.css` 增加品牌背景与 `.mobile #window-controls{display:none!important}`；`ui/js/game.ts` 承接移动端 UA 检测。
- `vite.config.ts`：改用 `import.meta.dirname` + `resolve` 解析入口。

### Phase 4 门禁实测（本地，全新 `npm ci`）

| 检查 | 结果 |
| --- | --- |
| `npm ci` | ✅ 194 packages，7s |
| `npm run check:version` | ✅ 1.1.2 |
| `npm run typecheck` / `lint` / `format:check` | ✅ 全部通过 |
| `npm test` | ✅ 21 passed（i18n 15 + audio 6，Vitest 5.1.0） |
| `npm run test:coverage` | ✅ 语句 97.34% / 分支 90.12% / 函数 96.87% / 行 98.56% |
| `npm run build` | ✅ vite 8.3.8，13 modules |
| `cargo fmt --all -- --check` | ✅ clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ 0 告警 |
| `cargo test --workspace` | ✅ 54 passed（core 15 + config_contract 12 + game_state 16 + physics 11） |
| `npx tauri build --debug --no-bundle` | ✅ exit 0，`target/debug/eyemotion.exe` 18,144,256 bytes（2026-11-11 14:07:02） |
| `npm audit --omit=dev --audit-level=high` | ✅ 0 vulnerabilities |
| workflow YAML 解析 | ✅ ci.yml / release.yml 均通过 js-yaml |

> 说明：macOS CI 构建已实测通过（run 42970839854，bundle-macos success）；Android 实际构建仍需首次 tag 发布验证；本地为 Windows，仅静态校验 workflow 与 Android 模板版本。

### Phase 4 安全门禁修复（CI security job）

- **问题**：CI run 42970839854 的 security job 失败。两层原因：
  1. `rustsec/audit-check@v2.0.0` 使用 `GITHUB_TOKEN` 创建 check run 时报 `Resource not accessible by integration`；job 缺少 `permissions`。
  2. `cargo audit` 报告 5 项漏洞（传递依赖）：RUSTSEC-2026-0007 `bytes 1.11.0`、RUSTSEC-2026-0194/0195 `quick-xml 0.38.4`、RUSTSEC-2026-0235 `rkyv 0.7.46`、RUSTSEC-2026-0009 `time 0.3.44`。
- **修复**：
  - `.github/workflows/ci.yml` security job 增加 `permissions: contents: read / checks: write / security-events: write`。
  - Cargo.lock 精准升级：`bytes 1.11.1`、`time 0.3.47`（time-core 0.1.8 / time-macros 0.2.27）、`plist 1.10.3`（`quick-xml 0.42.0`）、`rust_decimal 1.43.0`（移除 `rkyv 0.7`、`bytecheck` 等）、`byte-unit 5.2.6`、`tokio 1.53.3`、`anyhow 1.0.103`（附带修复 RUSTSEC-2026-0190）。
  - 验证：`cargo fmt --check` clean、`cargo clippy --workspace --all-targets -- -D warnings` exit 0、`cargo test --workspace` 54 passed；本地 `cargo audit` exit 0，**0 vulnerabilities**（10 allowed warnings）。
- **非阻断 warning 仍存在**：8 unmaintained（fxhash、pin-utils、proc-macro-error、unic-* 等）、2 unsound（glib 0.18 / rand 0.7，均为 tauri 构建链传递依赖，非运行时业务代码）；audit-check 只对 vulnerabilities 失败，warning 记录跟进。
- **CI 复跑（2026-11-11 14:40）**：CI run [42973041682](https://github.com/qingtin66/EyeMotion/actions/runs/42973041682)（HEAD `9a77a4c`）全部 success —— security / quality / bundle-macos / Security audit 全绿；security job 日志 `No vulnerabilities were found`、`Found 8 unmaintained, 2 unsound`、npm audit `found 0 vulnerabilities`。
- **复验结论**：reviewer task-15 终审 **PASS（0 阻断项）**；tester task-16 独立复验 **PASS（安全阻断已解除）**，本地全门禁绿 + CI 全绿。

---

## 桌面-only 收敛（v0.3 平台范围变更）

> 决策依据：用户 2026-11-11 评估后认为项目不适合移动端小屏幕，要求移除移动端。ADR：`docs/decisions/ADR-0001-desktop-only-platform.md`。

### 变更内容

- **文档/方案**：`docs/plans/modernization-plan.md` 升 v0.3；新增 ADR-0001；AGENTS.md、product_context.md、todo.md、multi_platform.md、CHANGELOG 同步为桌面三平台。
- **构建链**：`.github/workflows/release.yml` 删除 `release-android` job；`package.json` 删除 `build:android`/`build:ios`；`build_all.ps1` 删除 Android 段；删除 `MOBILE_ENV_SETUP.md`、`local_env.ps1` 与 `src-tauri/gen/android`（40 文件）。
- **前端**：删除 `VirtualControls` 与虚拟控件；renderer 移除 `isMobile` 分支（字体/按钮缩放、Game Over/Pause 横排、移动暂停按钮、stats 边距）；game.ts 去掉移动 UA 与移动退出分支；`ui/index.html`/`ui/css/style.css` 移除移动容器与 `.mobile` 样式；语言菜单拆为 `ui/js/language-menu.ts`。
- **Rust**：移除 `lib.rs` 的 `mobile_entry_point`、`commands/window.rs` 的 Android/iOS 条件编译与空实现、`eyemotion-core/src/config.rs` 的移动配置分支。

### 验收

- 本地门禁已通过：npm check:version / typecheck / lint / format:check / test（21 passed）/ build，cargo fmt / clippy -D warnings / test（54 passed），`npx tauri build --debug --no-bundle` 成功。
- 待 reviewer 只读终审、tester 独立复验后补记结论并提交。
