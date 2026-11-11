# EyeMotion 现代化执行进度

> 依据：`docs/plans/modernization-plan.md`（用户已确认 §8 六决策点，按「Phase 0 + 1 先行」执行）
> 最后更新：2026-11-11

## 总体状态

| 阶段 | 状态 |
| --- | --- |
| Phase 0 止血（P0-1..8） | ✅ 完成 |
| Phase 1 质量门禁（Q-1..Q-8） | ✅ 完成（Q-7 withGlobalTauri=false 按计划后置为独立 PR；Q-8 Playwright 冒烟为可选项未做） |
| Phase 2 架构 SSOT（A-1..A-6） | ⬜ 未开始 |
| Phase 3 仓库卫生（H-1..H-5） | ⬜ 未开始 |
| Phase 4 依赖与平台（D-1..D-4） | ⬜ 未开始 |

评审：reviewer 最终只读验收 task-8 = **PASS**（0 阻断项）。

## 提交

- `536c85e` fix(p0): 止血——版本统一、错误处理、安全基线、命令校验、构建可复现
- `5a181c8` chore(ci): Phase1 质量门禁基建——工具链钉桩、ESLint/Prettier、Vitest、CI 与依赖审计

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

## 下一步

1. 发布检查点：Phase 0+1 → 版本 1.1.3（CHANGELOG + tag）。
2. Phase 2 架构 SSOT：主题单一来源（品牌色 #0B1B7D/#153FA1 反写 Rust）、状态 SSOT、音频收敛、commands 拆分 + 中文文档、死代码清理（cargo-udeps）、window-state 插件注册/移除。
3. Phase 3 仓库卫生、Phase 4 依赖与平台。
