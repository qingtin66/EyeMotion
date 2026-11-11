# EyeMotion 任务进度（Todo）

> 权威计划见 [docs/plans/modernization-plan.md](docs/plans/modernization-plan.md)，执行记录见 [docs/plans/modernization-progress.md](docs/plans/modernization-progress.md)。

## 已完成
- [x] Phase 0 止血 8 项（版本 SSOT、错误处理、CSP/capabilities、命令校验、构建可复现等）
- [x] Phase 1 质量门禁 8 项（工具链钉桩、ESLint/Prettier、Vitest、CI、依赖审计、Rust 补测 ≥60%）
- [x] Phase 2 架构 SSOT 6 项（主题单一来源、状态权威归后端、命令模块化、死代码清理、window-state 死依赖移除）

## 进行中（Phase 3 仓库卫生）
- [x] H-1 移除入库二进制 zip 与重复 `src-tauri/Cargo.lock`
- [x] H-2 备份并删除 10 个旧客户端点目录（.agent/.claude/.codex/... 约 95 文件）
- [x] H-3 移除 `patch_config.js`/`restore_config.js` 与 build_all.ps1 的 patch/restore 步骤
- [ ] H-4 补齐 CONTRIBUTING / ISSUE+PR 模板 / product_context.md / todo.md / README 徽章修链 / multi_platform.md 更新
- [ ] H-5 README_ZH 版本号对齐 1.1.2 / 2026-03-01
- [ ] Phase 3 全量门禁复跑并提交

## 待办（Phase 4，视情况）
- [ ] 依赖升级：vite/vitest（修复 devDeps 审计告警）、rand 0.9、thiserror 2、tauri 生态
- [ ] CI 增加 macOS job；Android 构建规范化；E2E（Playwright 冒烟）
- [ ] 启用 cargo-llvm-cov 覆盖率门禁（-p eyemotion-core --fail-under-lines 60）
- [ ] Q-7：withGlobalTauri=false + CSP 去 unsafe-inline（独立 PR）
