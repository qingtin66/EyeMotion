# 🌍 EyeMotion 跨平台开发与 CI/CD 坑点记录（桌面三平台）

本文件记录 EyeMotion 在 Windows / macOS / Linux 桌面开发、构建及 GitHub Actions 中遇到的典型问题与解决方案。项目为 **桌面-only**，移动端（Android/iOS）已按 2026-11-11 决策移除，不再维护移动构建。

---

## 🪟 Windows (NSIS/MSI)

### 1. WebView2Loader.dll 缺失（绿色版）
- **现象**：打包出的 .zip 绿色版由于缺少 `WebView2Loader.dll` 无法在某些精简版系统运行。
- **解决方案**：在 `build_all.ps1` 中显式从编译产物目录拷贝该 DLL 到便携包根目录。

### 2. WebView2 Runtime 依赖
- Windows 10/11 通常自带 WebView2 Runtime；极简系统需提示用户安装，否则应用窗口空白。

---

## 🍎 macOS

### 1. 架构与 universal 产物
- 当前 release 使用 `--bundles app,dmg`，默认产出 arm64 包；Intel Mac 需后续增加 `universal-apple-darwin` 目标。
- 未做签名/公证时，用户首次打开需在「系统设置 → 隐私与安全性」允许；正式发布应配置 Apple Developer 证书与 notarization。

### 2. 系统最低版本
- `src-tauri/tauri.conf.json` 的 `bundle.macOS.minimumSystemVersion` 为 10.15；新增系统 API 时需同步评估。

---

## 🐧 Linux

### 1. WebKitGTK 系统依赖
- Ubuntu 22.04 CI 需安装：`libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`。
- `--bundles deb,appimage` 产出 deb 与 AppImage；不同发行版 glibc 版本需实测。

### 2. 文件系统与权限
- 配置写入遵循 XDG：`~/.config/eyemotion`；禁止硬编码平台路径。

---

## 🚀 GitHub Actions 策略建议

- **版本控制 (SSOT)**：版本号变更必须同步 `package.json`、`src-tauri/tauri.conf.json`、根 `Cargo.toml`，由 `npm run check:version` 校验。
- **Tag 驱动发布**：工作流由 `v*` tag 触发，确保发布版本与代码库通过 Git 强对齐。
- **平台矩阵**：`windows-latest / ubuntu-22.04 / macos-latest`；release 通过 tauri-action 按平台传 `--bundles` 参数。
- **缓存**：npm cache + Swatinem/rust-cache，缩短三平台构建时间。

---

## 🧰 现代化基线（Phase 0–4 之后）

- **版本 SSOT**：根 `Cargo.toml` 的 `workspace.package.version` 为唯一来源；`scripts/check-version.mjs` 校验三处一致。
- **工具链钉桩**：`rust-toolchain.toml`（stable + rustfmt + clippy）、`.nvmrc`/`.node-version`=24；CI 与本地一致。
- **质量门禁（.github/workflows/ci.yml）**：quality job 跑 `npm ci` → check:version/typecheck/lint/format:check/test/build + `cargo fmt --check`/clippy `-D warnings`/test --workspace；security job 跑 rustsec 审计 + `npm audit --omit=dev --audit-level=high`；另有 bundle-macos 构建 job。
- **依赖更新**：`.github/dependabot.yml` 每周检查 cargo / npm / actions。
- **构建脚本**：已移除 `patch_config.js`/`restore_config.js` 与 Android 构建段；`build_all.ps1` 仅负责 Windows 前端 + Tauri 打包 + 便携版收集。
