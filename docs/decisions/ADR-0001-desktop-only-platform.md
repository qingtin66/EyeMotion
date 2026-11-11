# ADR-0001：平台范围收敛为桌面-only（Windows/macOS/Linux）

- 状态：已接受
- 日期：2026-11-11
- 决策人：项目所有者
- 相关：`docs/plans/modernization-plan.md` v0.3

## 背景

EyeMotion 是面向桌面大屏的视觉追踪训练应用，交互以鼠标与全屏画布为核心。项目早期曾规划 Android/iOS 支持，并在 Phase 4 中按当时计划实现了 Android CI、SDK/NDK 配置与移动端虚拟控件。

经实际评估，移动端小屏幕不适合本产品的视觉训练场景：追踪目标与状态信息需要足够视野，手机尺寸会显著降低训练有效性；同时移动构建链（Gradle/NDK/签名/多 ABI）带来持续维护成本。

## 决策

项目平台范围收敛为 **桌面-only**：

1. 正式支持 Windows、macOS、Linux 三个桌面平台。
2. Android/iOS 不再作为构建、发布或测试目标。
3. 移除移动端专用构建链、生成物、脚本、UI 适配与 Rust 条件编译；保留桌面端全部功能。

## 后果

### 正面

- 发布矩阵与 CI 更简单：仅三桌面平台 + macOS 构建 job。
- 无需维护 Android SDK/NDK、Gradle、keystore、多 ABI 与移动 UI 分支。
- 代码与文档不再宣称不支持的平台，降低误导。

### 负面 / 约束

- 不再提供移动端安装包；未来若恢复移动端，需要重新引入构建链并做小屏 UX 设计（不复用当前已删除的生成目录）。
- 桌面触摸屏仍保留基础 touch 输入；这不代表恢复移动端产品支持。
- 历史 CHANGELOG 与 git 提交仍记录旧 Android 支持，属历史事实，不表示当前支持。

## 实施要点

- 删除 `.github/workflows/release.yml` 的 `release-android` job、`package.json` 的 Android/iOS 脚本、`src-tauri/gen/android`、`MOBILE_ENV_SETUP.md`、`local_env.ps1` 与 `build_all.ps1` 的 Android 段。
- 移除前端移动 UI（VirtualControls、移动布局分支、`.mobile` 样式）与 Rust 移动条件编译；语言菜单拆为独立的 `ui/js/language-menu.ts`。
- 更新 AGENTS.md、product_context.md、todo.md、multi_platform.md、CHANGELOG 与现代化计划 v0.3。
