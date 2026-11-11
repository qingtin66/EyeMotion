# EyeMotion v1.2.0 发布记录

> 发布日期：2026-11-11
> 发布分支：`main`
> 发布类型：桌面-only 首个正式版本（Windows / macOS / Linux）

## 1. 版本与变更

- 版本 SSOT：`1.2.0`
  - `Cargo.toml`（workspace.package.version）
  - `package.json`
  - `src-tauri/tauri.conf.json`
  - 锁文件：`Cargo.lock`、`package-lock.json`
- 发布说明来源：`CHANGELOG.md` 的 `## [1.2.0] - 2026-11-11` 段，由 `scripts/release-notes.mjs` 提取。
- 核心变更：
  - **Breaking**：平台范围收缩为桌面-only，移除 Android/iOS。
  - 依赖现代化：Rust `rand 0.9` / `thiserror 2`；前端 Vite 8 / Vitest 5 / jsdom 29 / Node 24。
  - 安全收尾：CSP 收紧、`withGlobalTauri=false`、前端改用 `@tauri-apps/api/core`。
  - 沿用 Phase 4 的 RustSec 传递依赖修复与 CI security 权限修复。

## 2. 本地发布前验证

- `npm run check:version`：✅ 1.2.0
- `npm run typecheck` / `lint` / `format:check`：✅
- `npm test`：✅ 21 passed（2 files）
- `npm run build`：✅ vite 8.3.8，12 modules
- `cargo fmt --all -- --check`：✅
- `cargo clippy --workspace --all-targets -- -D warnings`：✅
- `cargo test --workspace`：✅ 54 passed（15+12+16+11）
- `node scripts/release-notes.mjs`（模拟 `GITHUB_REF_NAME=v1.2.0`）：✅ 正确提取 1.2.0 发布说明

## 3. Windows 本地打包验证

`npx tauri build --bundles nsis,msi`：✅ exit 0

| 产物 | 大小 |
| --- | ---: |
| `target/release/bundle/nsis/EyeMotion_1.2.0_x64-setup.exe` | 1,679,968 bytes |
| `target/release/bundle/msi/EyeMotion_1.2.0_x64_en-US.msi` | 2,400,256 bytes |

> 安装包当前未做代码签名；Windows SmartScreen 可能提示“未知发布者”，详见 README 安全说明。

## 4. GitHub Release

- Tag：`v1.2.0`（待创建）
- Release workflow：`.github/workflows/release.yml`，矩阵 `windows-latest / ubuntu-22.04 / macos-latest`
- 预期产物：
  - Windows：NSIS `-setup.exe`、MSI
  - Linux：`.deb`、`.AppImage`
  - macOS：`.app`、`.dmg`（当前 runner 为 arm64）
- 发布说明：来自 `CHANGELOG.md` 1.2.0 段
- 验证结果：待 tag 后补充

## 5. 已知非阻断事项

- macOS 当前仅 arm64 产物，Intel/universal 后续补。
- 安装包未签名；后续可补 Windows Authenticode 与 macOS Developer ID/notarization。
- `cargo#6313` 的 `eyemotion.pdb` 同名警告不影响构建。
