# EyeMotion 现代化计划方案

> 版本：v0.2（三人评审后草案，待用户确认） · 日期：2026-11-11 · 团队：Lead（架构/主刀）+ Reviewer（审核）+ Tester（测试/构建）

---

## 1. 背景与目标

EyeMotion 是早期用旧版 Agent 客户端开发的 **Tauri v2 + Rust workspace + TypeScript/Canvas2D** 游戏，工程结构、配置、测试与 CI 已明显落后于当前主流实践。

本方案目标：在不改变游戏可玩性的前提下，把工程提升到 **可复现构建 + 有质量门禁 + 单一事实源(SSOT) + 安全基线达标** 的生产级基线，再逐步做依赖与平台现代化。

**这是评审草案**：确认范围与优先级后才进入实施；确认前只做讨论，不做结构性改造。

---

## 2. 现状评估摘要

以下每条均有实测证据（文件:行号 或命令），由 Reviewer / Tester 独立核查、Lead 复核。

### 2.1 P0 — 安全 / 正确性 / 合规（必须先止血）

| 编号 | 问题 | 证据 | 影响 |
|---|---|---|---|
| S1 | 生产代码使用 `.expect(...)` | `src-tauri/src/lib.rs:43` | 违反 AGENTS.md 硬禁令；崩溃不可控 |
| S2 | `csp=null` + `withGlobalTauri=true` + 授 `shell:allow-open`/`core:window:*` | `src-tauri/tauri.conf.json:13,26`；`capabilities/default.json` | XSS 可直达 IPC / 打开任意 URL |
| S3 | 版本 SSOT 三处不一致 | `Cargo.toml:6=1.1.0` / `package.json:3=1.1.2` / `tauri.conf.json:4=1.1.2` | 发布版本错误、AGENTS 规定以 Cargo 为准已落后 |
| S4 | `AGENTS.md` 被 gitignore，协议未入库 | `.gitignore:63`；`git ls-files AGENTS.md` 为空 | 新成员/AI 拿不到规范；`.gitignore:41` `! .env.example` 语法错误 |
| S5 | Tauri command 参数无校验（w/h/dt） | `src-tauri/src/commands.rs`（reset/resize/tick） | 非法入参可致 panic/异常状态 |
| S6 | 单元测试当前为红 | `cargo test -p eyemotion-core` → 6 passed / 1 FAILED（`ball.rs:162` 断言 radius==25.0 实际 35.5556；因 `ball.rs:20` 公式改而测试陈旧） | 阻断任何测试门禁 |
| S7 | TS 严格检查失败 | `npx tsc --noEmit` → `ui/js/i18n.ts(6,13) TS6133 ready 未使用` | 类型检查形同虚设 |
| S8 | `npm ci` 不可用（锁漂移） | `npm ci` → EUSAGE：lock 的 `@tauri-apps/cli@2.9.6` 不满足 `package.json` 精确 `2.1.0`；CI 用 `npm install` 掩盖 | 构建不可复现 |

### 2.2 P1 — 架构漂移 / 质量基线

- **主题 4 处来源且已漂移**：Rust `visual_config.rs` Theme::default（grid #011368/#00319F、ball #10B4C3/#11C58C、radius 1/40） vs `ui/js/renderer.ts:48-70`（#0B1B7D/#153FA1、#FC6170/#E62538、0.05） vs `ui/index.html:11` 硬编码 #0B1B7D vs `ui/css/style.css:139`。
- **音频配置双份**：`eyemotion-core/src/audio_config.rs` AudioParams 无前端消费（死配置）；`ui/js/audio.ts:128-137` 另硬编码 chords/melody。
- **状态双写**：后端 `Mutex<GameState>` 是权威，但前端 `ui/js/game.ts` 直接改本地 `this.gameState`（L218-221/L236-240/L274）；`commands.rs` tick 每帧 clone 全量 GameState。
- **commands 职责混杂**：`src-tauri/src/commands.rs`（113 行）把 游戏逻辑 + 状态访问 + 窗口控制(L74-99) + 配置 混在一起；多处 `let _=` 吞错（L77,80,85,95-97,110）；reset/resize 无校验无返回。
- **死代码 / 死依赖**：`bridge.ts:106 closeWindow()` 调用不存在的 `close_window`；`lib.rs` 未注册 `tauri-plugin-window-state`（依赖 + `window-state:default` 权限均死）；`ui/js/main.js` 与 Rust audio 死配置。
- **公共 API 无中文 `///` 文档**（违反 AGENTS 3.1）。
- **无质量门禁**：`.github/workflows/release.yml` 仅在 `push tags v*` 触发，无 PR/push lint/test/build、无缓存。
- **工具链未钉桩**：缺 `rust-toolchain(.toml)`、`.nvmrc`、`.node-version`、`.editorconfig`、`rustfmt.toml`、`clippy.toml`；本地 node v24.12.0 / rustc 1.92.0，而 CI 用 node 20 / `rust-toolchain@stable`（未固定）。
- **双 Cargo.lock + 二进制入库**：根 `Cargo.lock` 与 `src-tauri/Cargo.lock` 均被跟踪；`release/EyeMotion_Portable_Green.zip`（2.1MB）入库。

### 2.3 P2 — 卫生 / 依赖 / 平台

- 依赖陈旧：rand 0.8、thiserror 1.0。
- 11 个旧客户端目录（`.agent/.claude/.codex/.cursor/.gemini/.kiro/.roo/.shared/.trae/.windsurf/.workbuddy`，约 95 文件）冗余，其中 `.trae/skills` 42 文件重复。
- 构建 hack：`patch_config.js` / `restore_config.js` / `build_all.ps1` 会 patch/restore `tauri.conf.json`（`:66/:145`，破坏性风险）。
- 上下文文件缺失：`product_context.md`、`todo.md`（AGENTS.md 要求读取）；`multi_platform.md` 存在(5054B)。
- README 徽章错链 `eyemotion/eyemotion`（实际 `qingtin66/EyeMotion`）。
- 平台矩阵仅 windows-latest + ubuntu-22.04 + 单独 android job（大量 python workaround 生成 `tauri.settings.gradle`、patch build.gradle.kts），**无 macOS/iOS**。

---

## 3. 总体原则

1. **先门禁后重构**：Phase 0/1 先让问题能被自动发现，再动架构，避免重构无回归保护。
2. **小步可回滚**：每项独立成 commit/PR，可单独回退。
3. **SSOT 优先**：版本以 Cargo workspace 为准；主题以 Rust `Theme` 为准；运行状态以后端为准。
4. **只读分析已完成**：Reviewer（task-1）、Tester（task-2）已交付带证据的现状报告；实施阶段写操作按 disjoint scope 分工。
5. **合规**：所有新代码遵守 AGENTS.md（禁 unwrap/expect、pub API 中文注释、Conventional Commits）。

---

## 4. 分阶段路线图

> 工作量：S(<0.5d) / M(0.5–2d) / L(>2d)。风险：低/中/高。
>
> **关键依赖/门禁顺序**：Q-2 blocked_by P0-1/P0-2/P0-8（否则门禁首跑即红）；`withGlobalTauri→ES import`（Q-7）属结构改造须在门禁就绪后独立 PR；A-1/A-2 须在 Q-2 + Q-8 就绪后再动。

### Phase 0 — 止血与合规（P0，建议立即执行，约 S–M 总量）

| 编号 | 任务 | 关键改动 | 工作量 | 风险 | 验收标准 |
|---|---|---|---|---|---|
| P0-1 | 修红测试 | 让 `ball.rs:162` 断言与 `ball.rs:20` 的 `radius=screen_h/22.5` 一致（改断言或抽常量，不动游戏逻辑）。**改断言前先确认该公式为设计意图，勿改测试迁就代码** | S | 低 | `cargo test -p eyemotion-core` 全绿 |
| P0-2 | 修 TS 严格 | 处理 `ui/js/i18n.ts:6` 的 `ready` 未使用（删除或接入） | S | 低 | `npx tsc --noEmit` 0 error |
| P0-3 | 版本 SSOT 统一 | 三处统一为 1.1.2（`Cargo.toml:6` workspace 改 1.1.2）；新增脚本校验三处一致 | S | 低 | 三处版本一致且 CI 断言通过 |
| P0-4 | 消除生产 expect/吞错 | `lib.rs:43` 改 `Result` 传播；`commands.rs` `let _=` 改为记录/传播 | S | 低 | 生产代码 0 处 unwrap/expect（测试除外） |
| P0-5 | Tauri 安全基线（配置层） | 设 `csp` 白名单（非 null）；收紧 capabilities（移除 `shell:allow-open` 或改 allowlist、`core:window` 最小集）。注：`withGlobalTauri=false → ES import` 属结构性前端改造，拆为 Q-7 在门禁后独立 PR | M | 中 | csp 非 null、前端正常加载、`npm audit`/`cargo audit` 无 high/critical |
| P0-6 | 规范入库 | 从 `.gitignore:63` 移除 AGENTS.md 使其被跟踪；修 `.gitignore:41` 语法 | S | 低 | `git ls-files AGENTS.md` 非空 |
| P0-7 | Command 参数校验 | `w/h > 0 且 is_finite`、`dt` clamp 到合理区间，非法返回 `Err` | M | 低 | 非法入参返回 Err 且有单测 |
| P0-8 | 构建可复现 | 对齐 `package.json` 与 `package-lock.json`（@tauri-apps/cli 版本统一）；CI 改 `npm ci` | S | 低 | 干净环境 `npm ci` 成功 |

### Phase 1 — 质量基线 / CI 门禁（P1）

| 编号 | 任务 | 关键改动 | 工作量 | 风险 | 验收标准 |
|---|---|---|---|---|---|
| Q-1 | 工具链钉桩 | `rust-toolchain.toml`（固定通道 + `rustfmt`/`clippy` 组件）、`.nvmrc`/`.node-version`（与 CI 对齐）、`.editorconfig`、`rustfmt.toml`、`clippy.toml` | S | 低 | 新机按文件即可一致构建，node/rust 与 CI 一致 |
| Q-2 | PR 质量门禁（**依赖 P0-1/P0-2/P0-8**） | 新 workflow：PR/push 跑 `cargo fmt --check`、`clippy -D warnings`、`cargo test --workspace`、`tsc --noEmit`、ESLint/Prettier `--check`、`npm test`、`npm run build`；用 `npm ci`；ubuntu 装 `webkit2gtk` 等系统依赖（照 release.yml）；快/慢门禁拆分 + rust-cache/npm cache | M | 低 | PR 全绿方可合并；首跑不因已知红项失败；缓存命中 |
| Q-3 | 前端测试基座 | vitest(jsdom) 覆盖 `i18n.t`/`audio` 纯逻辑；`package.json` 加 test 脚本 | M | 中 | vitest 通过，行覆盖 ≥60%（istanbul） |
| Q-4 | Rust 补测 | 补 game_state/ball/config/stage 边界测试 + `tests/` 集成目录；随机重置用固定 seed 防 flaky | M | 中 | `cargo test --workspace` 全绿，行覆盖 ≥60%（cargo-llvm-cov） |
| Q-5 | 依赖安全 | CI 加 `cargo audit` + `npm audit`（含安装步骤，失败阈值：存在 high/critical 即失败）；配 dependabot | S | 低 | 无 high/critical，PR 自动升级 |
| Q-6 | Lint 规范接入 | 增 `eslint.config.*` + `.prettierrc`，CI `--check` 与本地一致 | S | 低 | ESLint/Prettier 0 error |
| Q-7 | 前端安全重构（门禁后） | `withGlobalTauri=false` + 改 `@tauri-apps/api` ES import（独立 PR） | M | 中 | 前端功能不变、CSP 生效、可回退 |
| Q-8 | 前端冒烟（可选） | 最简 Playwright 冒烟，护 A-1/A-2 重构 | M | 中 | 关键路径通过 |

### Phase 2 — 架构收敛 / SSOT（P1）

| 编号 | 任务 | 关键改动 | 工作量 | 风险 | 验收标准 |
|---|---|---|---|---|---|
| A-1 | 视觉主题单一来源 | Rust `Theme` 为唯一 SSOT，前端经 IPC/注入读取；删 `renderer.ts:48-70` 默认、`index.html:11`、`style.css:139` 硬编码 | M | 中 | 仅 1 处主题定义，改色只改 Rust |
| A-2 | 状态 SSOT | 前端只消费后端快照；去掉 `game.ts` 直接写权威字段；tick 回传改增量而非每帧 clone 全量 | M | 中 | 前端不再直接写权威字段，状态由 tick 回传 |
| A-3 | 音频单一来源 | Rust `AudioParams` 与 `audio.ts` 收敛（后端下发或删死配置） | S | 低 | 音频参数仅 1 处定义 |
| A-4 | commands 拆分与文档 | 窗口控制/配置/游戏逻辑分离；pub API 加中文 `///` | M | 中 | 单函数职责单一、pub 文档覆盖 100% |
| A-5 | 死代码清理 | 修/删 `bridge.ts:106 closeWindow`、`main.js`、Rust audio 死配置 | S | 低 | 无未引用符号/依赖（clippy/dead_code + `cargo udeps` 干净） |
| A-6 | 插件注册修复 | `lib.rs` 注册 `window-state` 或移除依赖+权限 | S | 低 | 依赖/权限与注册一致 |

### Phase 3 — 仓库卫生与协作（P1–P2）

| 编号 | 任务 | 关键改动 | 工作量 | 风险 | 验收标准 |
|---|---|---|---|---|---|
| H-1 | 去二进制/冗余锁 | `git rm --cached release/EyeMotion_Portable_Green.zip`、`src-tauri/Cargo.lock`（保留根锁），加 ignore | S | 低 | git 无二进制/重复锁 |
| H-2 | 旧客户端目录收敛 | 确认无引用后删除 11 个旧目录（约 95 文件） | S | 中 | 仓库干净、构建不受影响 |
| H-3 | 移除构建 hack | 清理 `patch_config.js`/`restore_config.js`/`build_all.ps1` 的 patch 步骤 | M | 中 | 构建不再改写 tauri.conf.json |
| H-4 | 骨架补齐 | CONTRIBUTING、ISSUE/PR 模板、`product_context.md`、`todo.md`、README 徽章修链、`multi_platform.md` 更新 | S | 低 | 文件齐全、链接可达 |
| H-5 | 文档版本对齐 | CHANGELOG / README_ZH 版本与 SSOT 一致 | S | 低 | 文档版本一致 |

### Phase 4 — 依赖与平台现代化（P2，需评估，可后置）

| 编号 | 任务 | 关键改动 | 工作量 | 风险 | 验收标准 |
|---|---|---|---|---|---|
| D-1 | 依赖升级 | rand 0.8→0.9、thiserror 1→2、tauri 2.x/vite 评估，逐个升级+回归 | M | 高 | 升级后构建+测试通过 |
| D-2 | 平台矩阵扩展 | 增加 macOS build job；iOS 需 macOS runner+签名（暂缓/评估） | M | 中 | windows+ubuntu+macOS 至少 build 通过 |
| D-3 | Android job 规范化 | 评估用 tauri 官方生成流程替代 python workaround | M | 中 | Android 构建稳定可维护 |
| D-4 | E2E 冒烟 | Playwright（Web 层）+ Tauri WebDriver 桌面冒烟（可选、高成本） | L | 高 | 关键路径端到端回归 |

---

## 5. 总体验收标准（Definition of Done）

- `cargo test --workspace` 全绿；`cargo clippy --workspace --all-targets -- -D warnings` 0 警告；`cargo fmt --check` 通过。
- `npx tsc --noEmit` 0 error；ESLint + Prettier --check 通过（Phase 1 引入）。
- 干净环境 `npm ci` 成功；CI 改用 `npm ci`。
- PR 触发 lint + test + build 全绿方可合并，且缓存命中。
- 三处版本号一致；`git ls-files AGENTS.md` 非空；仓库无二进制/重复锁。
- 主题仅 Rust 一处定义；前端不直接写权威状态；pub API 中文文档 100%。

---

## 6. 风险与回滚

- **Q-7 前端安全重构**（withGlobalTauri→ES import）可能影响前端加载：先在分支验证，保留回退开关；A-1/A-2 依赖其就绪。
- **A-1/A-2 结构性改动**风险中：必须 Phase 1 门禁就绪后再做，且分 PR 合并。
- **H-2 删除旧目录**不可逆：删除前 `git tag`/记录清单，确认无脚本引用。
- 所有改动走 feature 分支 + PR，主分支始终可构建；每阶段末打 tag。

---

## 7. 团队分工（实施阶段）

- **Lead**：架构改造主刀（Phase 2）、方案与文档、最终 diff 审核、合并。
- **Reviewer**：每阶段 PR 的 Standards/Spec 双轴代码审查（对照 AGENTS.md 与本文验收标准）。
- **Tester**：维护测试与 CI、执行冒烟/回归、把关门禁全绿。
- 写作用域互斥，通过共享任务板登记；跨阶段依赖用 blocked_by 排序。

### 7.1 团队成员期望模型（待平台支持）

- 审核（reviewer）→ **Hy4 routing · group**（模型 ID：`magpie / auto/hy4free`）
- 测试（tester）→ **EAC DeepSeek 4.1 Flash**（模型 ID：`our-free-model / deepseek-ai/deepseek-v4.1-flash`）
- 现状限制：当前 Agent Teams 构建的 `spawn_teammate` 无 model 参数，成员统一走会话默认模型 `group/power-first`，无法按成员区分；待平台支持后按上表配置。

---

## 8. 待用户确认的决策点

1. **推进顺序**：是否先做 Phase 0+1（止血+门禁）再做结构改造？_团队建议：0+1 先行，确认后再做 2/3/4。_
2. **平台范围**：桌面优先还是全平台？_团队建议：桌面优先（win/linux/mac），Android 保现状，iOS 暂缓（缺 mac runner+签名）。_
3. **版本目标**：_团队建议：先统一到 1.1.2；Phase 0+1 发 1.1.3；含结构改造发 1.2.0。_
4. **破坏性清理**：是否授权删除 11 个旧客户端目录与 `release` 归档 zip（不可逆）？_团队建议：授权删 zip+重复锁；11 个旧目录先打 tag 再删。_
5. **前端重构**：是否接受 `withGlobalTauri=false` + ES import？_团队建议：接受，作为独立 PR 于门禁后（Q-7）。_
6. **品牌色定稿**：_团队建议：以 UI 现状 #0B1B7D/#153FA1 定稿并反写 Rust Theme（旧色 #011368 实际未生效）。_

---

## 9. 证据索引（关键锚点）

- 版本：`Cargo.toml:6`、`package.json:3`、`src-tauri/tauri.conf.json:4`、`README_ZH.md:61`
- 安全：`src-tauri/tauri.conf.json:13,26`、`src-tauri/capabilities/default.json`、`src-tauri/src/lib.rs:43`
- 状态/主题/音频：`src-tauri/src/commands.rs:13,74-99`、`ui/js/game.ts:218-221,236-240,274`、`eyemotion-core/src/visual_config.rs`、`ui/js/renderer.ts:48-70`、`eyemotion-core/src/audio_config.rs`、`ui/js/audio.ts:128-137`
- 测试/构建：`eyemotion-core/src/ball.rs:162,20`、`ui/js/i18n.ts:6`、`package-lock.json`、`.github/workflows/release.yml`
- 卫生：`.gitignore:41,63`、`release/EyeMotion_Portable_Green.zip`、`src-tauri/Cargo.lock`、`patch_config.js`/`restore_config.js`/`build_all.ps1`

---

_本文件为评审草案，等待用户确认 §8 决策点后进入实施；确认前不改动代码结构。_