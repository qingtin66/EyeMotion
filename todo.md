# EyeMotion 任务进度（Todo）

> 权威计划见 [docs/plans/modernization-plan.md](docs/plans/modernization-plan.md)，执行记录见 [docs/plans/modernization-progress.md](docs/plans/modernization-progress.md)。项目已于 2026-11-11 收敛为桌面-only（Windows/macOS/Linux）。

## 已完成
- [x] Phase 0 止血 8 项（版本 SSOT、错误处理、CSP/capabilities、命令校验、构建可复现等）
- [x] Phase 1 质量门禁 8 项（工具链钉桩、ESLint/Prettier、Vitest、CI、依赖审计、Rust 补测 ≥60%）
- [x] Phase 2 架构 SSOT 6 项（主题单一来源、状态权威归后端、命令模块化、死代码清理、window-state 死依赖移除）
- [x] Phase 3 仓库卫生 5 项（去二进制/重复锁、旧客户端目录、构建 hack、骨架补齐、版本文档对齐）
- [x] Phase 4 依赖与平台 4 项（Rust/前端依赖升级、macOS CI、Android 规范化、E2E 评估）——其中 Android 相关成果已按桌面-only 决策回退

## 当前：桌面-only 收敛
- [x] 更新方案/AGENTS/product_context/todo/multi_platform/CHANGELOG 与 ADR
- [x] 移除 release-android job、package android/ios 脚本、src-tauri/gen/android、MOBILE_ENV_SETUP.md、local_env.ps1
- [x] 移除前端移动 UI（virtual-controls、移动布局分支、.mobile 样式）与 Rust 移动条件编译
- [ ] 全量门禁 + 桌面构建（已完成）+ reviewer/tester 独立验收 + 提交推送

## 后续可选
- [ ] E2E：Playwright 需先做 Bridge mock 与 Canvas headless 抽象（D-4 评估暂缓）
- [ ] 启用 cargo-llvm-cov 覆盖率门禁（-p eyemotion-core --fail-under-lines 60）
- [ ] macOS 增加 universal 产物（当前仅 arm64）
