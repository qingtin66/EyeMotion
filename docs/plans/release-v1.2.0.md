# EyeMotion v1.2.0 发布记录

> 发布日期：2026-11-11
> 发布分支：`main`
> 发布类型：桌面-only 首个正式版本（Windows / macOS / Linux）

## 1. 版本与变更

- 版本 SSOT（`scripts/check-version.mjs` 自动校验三处）：`1.2.0`
  - `Cargo.toml`（workspace.package.version）
  - `package.json`
  - `src-tauri/tauri.conf.json`
- 同步更新的锁文件：`Cargo.lock`、`package-lock.json`（当前与三处 SSOT 一致；`check-version` 暂不自动校验锁文件）
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

- Release：<https://github.com/qingtin66/EyeMotion/releases/tag/v1.2.0>
- Tag：`v1.2.0`（annotated，指向提交 `e36d167`；发布状态 draft=false / prerelease=false）
- 发布时间：2026-11-11T08:04:49Z（北京时间 16:04:49）
- Release workflow：`.github/workflows/release.yml`，run [42983867080](https://github.com/qingtin66/EyeMotion/actions/runs/42983867080)；矩阵 `windows-latest / ubuntu-22.04 / macos-latest` 三平台 job 全部 success（macOS 3m0s、Ubuntu 7m24s、Windows 8m53s）。
- 发布说明：来自 `CHANGELOG.md` 1.2.0 段，包含 Summary、⚠️ Breaking Changes、Changed、Removed、Added、Fixed。

| 发布资产 | 大小（bytes） |
| --- | ---: |
| `EyeMotion_1.2.0_x64-setup.exe`（Windows NSIS） | 1,652,131 |
| `EyeMotion_1.2.0_x64_en-US.msi`（Windows MSI） | 2,342,912 |
| `EyeMotion_1.2.0_amd64.deb`（Linux Debian/Ubuntu） | 2,254,642 |
| `EyeMotion_1.2.0_amd64.AppImage`（Linux AppImage） | 79,976,952 |
| `EyeMotion_1.2.0_aarch64.dmg`（macOS arm64） | 2,672,985 |
| `EyeMotion_aarch64.app.tar.gz`（macOS app 包） | 2,330,421 |
| `EyeMotion_1.2.0_SHA256SUMS.txt`（6 个发布物校验和） | 576 |

- 发布后验证：Lead 已核对 Release 元数据与资产；tester task-26 独立复验 **PASS**，确认 7 个资产齐全、Windows NSIS/MSI 下载后 SHA256 与 `SHA256SUMS` 一致、安装包版本均为 1.2.0，且 run 42983867080 三平台 job 全 success。

## 5. 已知非阻断事项

- macOS 当前仅 arm64 产物，Intel/universal 后续补。
- 安装包未签名；后续可补 Windows Authenticode 与 macOS Developer ID/notarization。
- `cargo#6313` 的 `eyemotion.pdb` 同名警告不影响构建。
