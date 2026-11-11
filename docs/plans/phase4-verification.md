# Phase 4 独立验证报告（task-13）

- 验证人：tester（独立复跑，未修改源文件）
- 日期：2026-11-11
- 被验证提交：`6e43198`（main，已 push origin）
- 环境：Windows / node v24.12.0 / npm 11.21.2 / rustc·cargo 1.92.0 / tauri-cli 2.9.6

## 结论：本地门禁全部 PASS；CI run 42970839854 的 security job 失败（1 项阻断：见 §10 与「阻断问题」）

## 1) 仓库状态与提交

- `git status --porcelain`：空（干净）
- `git show --stat 6e43198`：24 files changed, 1588 insertions(+), 2477 deletions(-)；含 ci.yml/release.yml、Cargo.toml/lock（rand 0.9 / thiserror 2.0）、package.json/lock（vite 8.3.8 / vitest 5.1.0 / jsdom 29.1.1）、tauri.conf.json（CSP + withGlobalTauri=false）、bridge.ts、build.gradle.kts（keystore 签名）、删除 src-tauri/gen/release.jks（Bin 2766 → 0）

## 2) 全新安装

- 删除 node_modules 后 `npm ci`：exit 0；关键依赖实测 @tauri-apps/api@2.9.1、@tauri-apps/cli@2.9.6、vite@8.3.8、vitest@5.1.0、jsdom@29.1.1、@vitest/coverage-v8@5.1.0

## 3) 前端门禁

| 命令 | 结果 |
| --- | --- |
| `npm run check:version` | exit 0，✅ 版本一致: 1.1.2 |
| `npm run typecheck` | exit 0 |
| `npm run lint` | exit 0（eslint ui scripts） |
| `npm run format:check` | exit 0（Prettier 全部通过） |
| `npm test` | exit 0，2 files / 21 tests passed（audio 6、i18n 15） |
| `npm run test:coverage` | exit 0；Stmts 97.34% / Branch 90.12% / Funcs 96.87% / Lines 98.56%（audio.ts 97.28/90.74/100，i18n.ts 97.61/88.88/91.66） |
| `npm run build` | exit 0，vite 8.3.8，13 modules，177ms（index.html 0.85kB、main.js 27.22kB gzip 8.05、main.css 3.34kB） |

说明：测试 stderr 中 "AudioContext not supported" / "no tauri" 为用例刻意触发的错误路径，非失败。

## 4) Rust 门禁

| 命令 | 结果 |
| --- | --- |
| `cargo fmt --all -- --check` | exit 0（无差异） |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0（零 warning） |
| `cargo test --workspace` | exit 0；eyemotion lib 0 / main 0 / eyemotion_core lib 15 / config_contract 12 / game_state 16 / physics 11 / doc-tests 0 = **54 passed / 0 failed** |

## 5) 依赖审计

- `npm audit --omit=dev --audit-level=high`：exit 0，**found 0 vulnerabilities**（dev 侧 7 moderate 不阻断，由 dependabot 跟进）

## 6) CI workflow 静态核对（js-yaml 解析 OK）

- ci.yml jobs：quality / bundle-macos / security。quality 在 ubuntu-22.04 上完整执行 check:version → typecheck → lint → format:check → test → build → fmt → clippy(-D warnings) → test --workspace，含 webkit2gtk 依赖与 Swatinem/rust-cache；bundle-macos 用 `npx tauri build --no-bundle`（macOS 覆盖 ✓）；security 跑 rustsec/audit-check + `npm audit --omit=dev --audit-level=high`。
- release.yml jobs：release（matrix windows-latest / ubuntu-22.04 / macos-latest，fail-fast:false；ubuntu args `--bundles deb,appimage`、macos `--bundles app,dmg`；release notes 由 `node scripts/release-notes.mjs` 提取）+ release-android（Java 17 zulu、4 个 Android Rust targets、android-actions/setup-android@v3 sdk-version 36 / build-tools 36.0.0 / NDK 27.2.12479018、`tauri android init --ci` 幂等、`tauri android build --apk --split-per-abi --ci`、APK 数量断言 `test "$count" -gt 0`、artifact 上传 + tag 时 softprops 传 Release）。
- 人工核对：matrix/args/SDK 36/断言逻辑均符合描述。

## 7) Tauri 独立构建

- `npx tauri build --debug --no-bundle`：exit 0，编译 22.09s，产物 `target\debug\eyemotion.exe`

## 8) Android 签名静态核对（本地无 Android SDK，未真构建）

- `src-tauri/gen/android/app/build.gradle.kts`：release 签名读 `rootProject.file("keystore.properties")`；存在则用 release keystore，不存在回退 `signingConfigs.getByName("debug")`（注释明示 CI 无密钥时的回退策略）✓
- `git ls-files` 无 release.jks / keystore.properties（release.jks 已从跟踪中移除）✓；磁盘上 release.jks 仍存在（本地保留、被忽略）✓
- `src-tauri/gen/android/.gitignore` 含 `keystore.properties`、`release.jks`、`*.jks`；根 .gitignore 含 `src-tauri/gen/*.jks` ✓

## 9) Q-7 安全收尾核对

- `tauri.conf.json`：`withGlobalTauri: false`；CSP `script-src 'self'`（无 unsafe-inline）✓
- `ui/index.html`：仅 `<script type="module" src="/js/game.ts">`，无内联脚本 ✓
- `ui/js/bridge.ts`：`import { invoke } from '@tauri-apps/api/core'`，运行时以 `'__TAURI_INTERNALS__' in window` 探测，非 Tauri 环境回退 null ✓

## 10) GitHub CI run 42970839854（main push）—— 最终状态

- quality ✓ / bundle-macos ✓（2m34s）/ **security ✗ failure**（run conclusion: failure）
- 失败原文（RustSec audit 步骤）：
  - `##[warning]5 vulnerabilities found!` / `##[warning]12 warnings found!` / `Found 5 advisories, 8 unmaintained, 4 unsound`
  - `##[error]Resource not accessible by integration - https://docs.github.com/rest/checks/runs#create-a-check-run`
- 5 个漏洞（均来自 Cargo.lock 传递依赖）：
  1. RUSTSEC-2026-0007：bytes 1.11.0（BytesMut::reserve 整数溢出，memory-corruption；patched >=1.11.1）
  2. RUSTSEC-2026-0194：quick-xml 0.38.4（重复属性名二次方复杂度 DoS；patched >=0.41.0）
  3. RUSTSEC-2026-0195：quick-xml 0.38.4（NsReader 无界命名空间分配内存耗尽 DoS；patched >=0.41.0）
  4. RUSTSEC-2026-0235：rkyv 0.7.46（Rc/Arc archive 校验不足越界读；patched >=0.8.17）
  5. RUSTSEC-2026-0009：time 0.3.44（RFC2822 解析栈耗尽 DoS；patched >=0.3.47）
- 另有 8 unmaintained（fxhash/pin-utils/proc-macro-error/unic-*）与 4 unsound（anyhow<1.0.103 downcast_mut、glib 0.18、rand 0.7.3 传递、tokio AsyncFd），均为 warning，不直接判失败。
- 失败判定的两个原因（需 lead 处置）：
  a) 权限错误（动作自身失败）：rustsec/audit-check@v2.0.0 需要 permissions: checks: write（其默认还会 security-events: write 上传 SARIF）；ci.yml 顶层无 permissions 块，仓库默认 token 只读 → Resource not accessible by integration。
  b) 5 个真实 advisory：即使权限修好，cargo audit 也会因漏洞非零退出。bytes/time 可经 `cargo update -p bytes --precise 1.11.1`、`cargo update -p time --precise 0.3.47` 等 semver 兼容升级解决；quick-xml(→0.41) 与 rkyv(→0.8.17) 是 tauri 栈的传递依赖，需评估上游兼容性或临时 ignore。

## 发现的问题

### 阻断

1. **CI security job 失败（main 分支红）**：如 §10 所述，rustsec/audit-check@v2.0.0 因缺 `permissions` 无法创建 check-run（`Resource not accessible by integration`），且 Cargo.lock 存在 5 个 RUSTSEC advisory（bytes/quick-xml×2/rkyv/time，均为传递依赖）。修复建议：ci.yml 给 security job 加 `permissions: { checks: write, contents: read }`（或改用 `cargo audit` 直接跑避免 check-run 依赖），同时升级可兼容的传递依赖并对暂无兼容补丁的（quick-xml 0.41、rkyv 0.8.17 跨大版本）评估 ignore 清单。

### 非阻断（建议）

1. **actions 版本注解**：checkout@v4 / setup-node@v4 触发 "Node.js 20 is deprecated" 注解（forced to Node 24）。dependabot 已开 PR（setup-java 4→6、tauri-action 0→1）。建议升级 actions/* 到 v5+/v6 消除注解。
2. **cargo#6313 警告仍在**：src-tauri bin 与 lib 同名 eyemotion 导致 `eyemotion.pdb` output filename collision 警告（未来或变 hard error）。建议 lib 改名（如 eyemotion-app）。
3. **rust-toolchain.toml 仍是 channel="stable"**：未精确 pin（本机 1.92.0），复现性略弱；与 Phase 3 观察一致，未在本次范围内处理。
4. coverage include 范围仍只有 i18n.ts/audio.ts；vite 8 升级后 i18n.ts 覆盖率从 100/93.33/88.88 变为 97.61/88.88/91.66（行 13、18 未覆盖），仍高于阈值。

## 后续处理与复验（Lead，2026-11-11）

- **原阻断项已修复**：
  1. `.github/workflows/ci.yml` security job 增加 `permissions: contents: read / checks: write / security-events: write`；
  2. Cargo.lock 精准升级消除全部 5 项 RUSTSEC 漏洞：`bytes 1.11.1`、`time 0.3.47`、`plist 1.10.3`（`quick-xml 0.42.0`）、`rust_decimal 1.43.0`（移除 `rkyv 0.7`）、`byte-unit 5.2.6`、`tokio 1.53.3`、`anyhow 1.0.103`。
- **本地复验**：`cargo fmt --all -- --check` exit 0；`cargo clippy --workspace --all-targets -- -D warnings` exit 0；`cargo test --workspace` 54 passed；`cargo audit` exit 0，**0 vulnerabilities**（10 allowed warnings：8 unmaintained + 2 unsound glib/rand0.7）。
- **修复提交**：`aa3b4b1`（fix(security): 修复 RustSec 传递依赖漏洞与 CI security 权限）。
- **待最终确认**：推送后由 GitHub Actions security job 复跑确认；若通过，本报告 §10/阻断项 1 即关闭。
