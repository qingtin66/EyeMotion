# Phase 4 平台与依赖审核报告（只读评估）

> **后记（2026-11-11）**：用户已决定移除移动端，项目改为桌面-only。本报告中 Android/D-3 内容仅作历史记录，相关实现已回退；最新范围见 `docs/decisions/ADR-0001-desktop-only-platform.md`。

- **审核角色**：reviewer（只读）
- **任务**：task-11（Phase 4 平台与依赖审核）
- **基线**：已提交 HEAD = `3f8a674`（docs(plans): 记录构建、审核与测试验收结果）
- **重要说明**：撰写时工作区存在 Lead(task-12) 的**未提交** Phase 4 改动（`Cargo.toml`、`Cargo.lock`、`ball.rs`、`game_state.rs`、`package.json`、`package-lock.json`、`tauri.conf.json`、`.nvmrc`、`.node-version`、`ui/*`）。本报告以**已提交基线**为准做评估，并对工作区 WIP 仅作观察性引用；WIP 未经提交与全量门禁前不可作为验收依据。
- **范围**：D-2（release.yml 增加 macOS）、D-3（Android 官方流程替代 python workaround）、D-1（依赖升级风险）。仅评估，不改代码/配置。

---

## 1. 现有 Phase 4 相关文件盘点

| 文件 | 现状要点 | 文件:行 |
|---|---|---|
| `.github/workflows/release.yml` | 262 行；`release` 矩阵仅 2 平台；另有 `release-android` job 含 6 步 python workaround | release.yml:16、:88-262 |
| `.github/workflows/ci.yml` | Node 版本硬编码 20（两处） | ci.yml:18、ci.yml:56 |
| `docs/plans/modernization-plan.md` | Phase 4 定义 D-1..D-4 | plan:120-127 |
| `docs/plans/modernization-progress.md` | Phase 4 列为"视情况"（vite/vitest 升级、macOS CI、Android 规范化、llvm-cov、Q-7） | progress 尾节 |
| `package.json` | 已有 `build:macos`=`--target universal-apple-darwin`、`build:android`、`build:ios`；vite ^5 / vitest ^2.1.9 / @tauri-apps/cli 2.9.6 | package.json:18-20、:24、:34-35 |
| `src-tauri/tauri.conf.json` | `bundle.targets="all"`；`bundle.macOS.minimumSystemVersion="10.15"`；图标含 `icon.icns`/`icon.ico`（macOS 打包齐备） | tauri.conf.json:31、:50-52、:32-39 |
| `build_all.ps1` | Android 本地构建含"CLI 失败后手动拷贝 .so + Gradle"回退 | build_all.ps1:50-95 |
| `MOBILE_ENV_SETUP.md` / `local_env.ps1` | 本地 JDK17/SDK33/NDK 环境激活脚本 | — |
| `multi_platform.md` | 记录 Android `tauri.settings.gradle` 缺失与 `assembleUniversalRelease` 缺失两个坑及 python workaround | multi_platform.md:9-43 |
| `.github/dependabot.yml` | cargo / npm / github-actions 每周检查（会各自单开大版本 PR，需注意联动） | dependabot.yml:1-12 |
| `Cargo.toml` workspace.dependencies | 基线为 `rand="0.8"`、`thiserror="1.0"`（WIP 已改 0.9/2.0） | Cargo.toml:14-15 |
| `.nvmrc`/`.node-version`/`rust-toolchain.toml` | 基线 Node=20（WIP 已改 24）；Rust stable | .nvmrc、rust-toolchain.toml:2 |

---

## 2. D-2：release.yml 增加 macOS job 的可行性

### 2.1 结论
**可行，且成本可控**。三平台矩阵 + 无签名构建即可满足 DoD"windows+ubuntu+macOS 至少 build 通过"。需改动集中在 `release` job 的 matrix、Rust 目标、bundle args 与"发布说明提取"步骤的跨平台化。

### 2.2 现状问题（证据）
- matrix 只有 `[windows-latest, ubuntu-22.04]`（release.yml:16），无 macOS。
- 发布说明步骤 `Extract release notes` 使用 `shell: pwsh`（release.yml:54）——PowerShell 在 GitHub 全部官方 runner（含 macOS）均预装，**未必失败**，但用 `Out-File` 写 `GITHUB_ENV` 在不同 shell 有编码/BOM 差异风险，建议改为 Node 脚本以保证跨平台一致。
- `--bundles` 参数用三元表达式（release.yml:86），仅对 ubuntu 特殊化；macOS 走默认 `bundle.targets="all"`（app+dmg），可接受。

### 2.3 建议改动（最小可行）
将 matrix 改为 `include` 形式，按平台给出 `args`，避免复杂表达式：

```yaml
jobs:
  release:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: windows-latest
            args: ''
          - platform: ubuntu-22.04
            args: '--bundles deb,appimage'
          - platform: macos-latest           # arm64 runner
            args: '--target universal-apple-darwin'   # 若只需本机架构可留空 ''
```

- 增加 macOS 通用二进制目标（仅在 macOS 且需要 universal 时）：
```yaml
      - name: Add macOS universal targets
        if: matrix.platform == 'macos-latest' && matrix.args != ''
        run: rustup target add aarch64-apple-darwin x86_64-apple-darwin
```
- tauri-action 步骤改为 `args: ${{ matrix.args }}`（替换 release.yml:86）。
- `Install Linux Dependencies` 已有 `if: matrix.platform == 'ubuntu-22.04'`（release.yml:36），无需改。
- 发布说明提取（release.yml:52-74）建议改写为跨平台 Node 脚本（如 `scripts/release-notes.mjs`，读取 CHANGELOG 并 `fs.appendFileSync(process.env.GITHUB_ENV, ...)`），用 `shell: bash` 或直接 `run: node scripts/release-notes.mjs` 在 Windows/macOS/Linux 均可运行；并加 `continue-on-error: true` 兜底（失败则回退默认 `RELEASE_BODY`）。

### 2.4 可选（Phase 4c，不影响"build 通过"）
- **签名与公证**：无签名可构建，但 macOS Gatekeeper 会拦截分发。若需面向终端用户，追加 secrets：`APPLE_CERTIFICATE`、`APPLE_CERTIFICATE_PASSWORD`、`APPLE_SIGNING_IDENTITY`、`APPLE_ID`、`APPLE_PASSWORD`、`APPLE_TEAM_ID`（tauri-action 自动读取）。**建议先落地"可构建"再加签名，拆两个 PR。**
- universal 与 per-arch 二选一：`macos-latest` 为 arm64，若不加 `--target universal-apple-darwin` 只产出 arm64 包；universal 体积翻倍但兼容 Intel。

### 2.5 风险与验收
| 风险 | 等级 | 缓解 |
|---|---|---|
| 发布说明脚本在 macOS 失败 | 中 | 改 Node 脚本 + continue-on-error |
| universal 目标未安装导致 tauri-action 失败 | 中 | 条件步骤 `rustup target add` |
| `bundle.targets="all"` 在某平台产出多余/失败包 | 低 | macOS 可显式 `--bundles app,dmg` |
| 构建时长增加约 1/3（三平台） | 低 | `fail-fast:false` 已设，可接受 |

**验收标准**：打 `v*` tag（或 workflow_dispatch）后，`release` job 在 windows-latest / ubuntu-22.04 / macos-latest 三平台全部 build 成功；Release 资产含 macOS 的 `.app`/`.dmg`（或 universal），且发布说明非空。

---

## 3. D-3：Android 官方流程替代 python workaround 评估

### 3.1 现状（证据）
`release-android` job（release.yml:88-262）构建流程 6 步：
1. `rm -rf src-tauri/gen/android` + `npx tauri android init --ci`（:139-140）
2. `cargo fetch`（:144）
3. **python 脚本**从 Cargo registry 精确查找 `tauri`/`tauri-plugin-shell` 源码并手写 `tauri.settings.gradle`（:146-208）
4. **python 脚本**向 `app/build.gradle.kts` 注入 `splits { abi { ... isUniversalApk = true } }`（:210-232）
5. shell 校验两处文件/标记存在（:234-243）
6. `npx tauri android build --target aarch64`（:246）

### 3.2 根因核对（外部证据）
- Tauri 官方 issue **#13902**（tauri.settings.gradle missing）：维护者/回答指出——**`tauri.settings.gradle` 由 `tauri android build`/`dev` 生成，而非 `init`**。"You should use tauri android build or tauri android dev. The init command doesn't generate the tauri gradle property file."（issue 已 closed as not planned）→ 说明第 3 步 python 生成是**对 CLI 行为的误解**，正常 `android build` 会产出该文件。
- 官方文档 `v2.tauri.app/distribute/google-play`：`tauri android build --apk` 生成 APK、`--aab` 生成 AAB；**默认构建 universal APK**，按架构拆包用 `--split-per-abi`，架构选择用 `--target`。→ 第 4 步手改 `splits` 属**非官方补丁**，官方已用 `--split-per-abi`/`--apk` 表达。
- Tauri issue **#13357**（`assembleUniversalRelease not found`）**仍 open**：在部分 CLI 版本用 `--apk`/`--aab` 仍可能触发该 Gradle 任务缺失。→ 说明"完全去 workaround"**依赖 CLI 版本**，需要验证而非盲信。

### 3.3 结论
**可基本替代，但属"版本相关"的替换**，建议分两步走：
1. **先升级 `@tauri-apps/cli` 到最新稳定**（官方 2.12+ 已让 `init` 自动跳过交互，不再需要 `--ci`；并更新 Gradle 模板）。新版对 `tauri.settings.gradle` 生成与 universal 任务问题修复更完善。
2. 用**官方命令流**替换 python 两段脚本：
```yaml
      - name: Build Android APK
        env: { ANDROID_HOME: ..., NDK_HOME: ... }
        run: |
          rm -rf src-tauri/gen/android
          npx tauri android init
          npx tauri android build --apk --target aarch64
```
   保留**轻量断言**（构建后检查 APK 存在），删除第 3、4 步 python 与 splits 补丁。
3. **保留回退开关**：若升级后 `--apk` 仍命 #13357，则退而用 `npx tauri android build --aab`（AAB 不触发 universal APK 任务）或 `--split-per-abi`，并把 CLI 版本**钉死**到已验证版本；仅在确需手补时保留最小 bash 校验。

> 另注意：当前 workaround 里 `--target aarch64`（仅 arm64）但 splits 却 include `arm64-v8a/armeabi-v7a/x86_64` 且 `isUniversalApk=true`，语义本身不一致；官方 `--apk`（universal）或明确 `--split-per-abi` 更清晰。

### 3.4 风险与验收
| 风险 | 等级 | 缓解 |
|---|---|---|
| CLI 升级导致 `gen/android` 模板（Gradle 9/AGP）与旧 SDK/NDK 不匹配 | 中 | 同步升 `sdk-version/build-tools`（33→34/35）、NDK 保持 r27；分支验证 |
| #13357 universal 任务缺失在目标版本仍存在 | 中 | 用 `--aab` 或 `--split-per-abi`；钉 CLI 版本 |
| 移除 python 后若 `settings.gradle` 仍未生成 | 低 | 加"文件不存在则 fail 并打印命令日志"断言，便于定位 |
| 构建只出 arm64，未覆盖其他 ABI | 低 | 明确需求后选择 universal 或 split |

**验收标准**：CI Android job 不再含 python 脚本/手改 `build.gradle.kts`；`tauri android build --apk` 一次成功产出 APK 并上传；构建在固定 CLI 版本下可重复稳定；若保留任何最小校验，须有注释说明对应 issue 编号。

---

## 4. D-1：依赖升级风险清单

### 4.1 版本现状 vs 目标
| 依赖 | 当前（基线） | 目标 | 说明 |
|---|---|---|---|
| rand | 0.8（Cargo.toml:14） | 0.9 | API 重命名 |
| thiserror | 1.0（Cargo.toml:15） | 2.0 | 主要加 no_std，用法基本兼容 |
| vite | 5.4.21（package-lock） | 8.x | Rolldown 打包器、ESM-only |
| vitest | 2.1.9 | 5.x | 需 Vite≥6.4 且 Node≥22.12；Vite 变 peer |
| @vitest/coverage-v8 | ^2.1.9 | 5.x | 必须与 vitest 同主版本 |
| Node | 20（.nvmrc、ci.yml:18/56、release.yml:30/102） | 24 | Node 20 于 2026-04-30 EOL |
| @tauri-apps/cli | 2.9.6 | 最新(≥2.12) | 配合 D-3 |

### 4.2 rand 0.8 → 0.9
官方迁移（rust-random book "Updating to 0.9"）：`thread_rng()`→`rng()`、`gen_range`→`random_range`、`gen`→`random`、`gen_bool`→`random_bool`。本项目使用点（基线）：`ball.rs:34,40`、`game_state.rs:107-108,229-268`。
- 工作区 WIP 已见对应改法（`thread_rng→rng`、`gen_range→random_range`、`gen_bool→random_bool`），与官方映射一致，**改动面小、风险低**。
- 注意：0.9 改变了部分 distribution/seed API（未使用）；若测试断言依赖具体随机序列需复核（本项目未见此类断言）。

### 4.3 thiserror 1 → 2
`eyemotion-core/src/error.rs` 仅用 `#[derive(Error)]` + `#[error("...{0}")]` + `#[from]`（std::io::Error、serde_json::Error）。thiserror 2.0 对这些用法**保持兼容**（2.0 主要新增 no_std，MSRV 略升）。**风险低**；升级后跑 `cargo build/test/clippy` 即可确认。

### 4.4 vite 5 → 8
Vite 8.0（2026-03-12 发布）要点：改用 **Rolldown**（Rust）作单一打包器，宣称构建快 10–30×；**ESM-only**；**Node 要求 20.19+/22.12+**（与 Node24 兼容）。本项目 `vite.config.ts` 仅用 `root/base/build.outDir/rollupOptions.input/server`，Rolldown 兼容 Rollup 选项，**无自定义插件**，升级风险中低。需复核：build 产物行为、`base:'./'` 相对路径、CSS 处理。

### 4.5 vitest 2 → 5（关键联动）
- Vitest 5.0（2026-09-03）**要求 Vite ≥ 6.4.0 且 Node ≥ 22.12.0**（官方迁移页）。
- Vite 由 vitest 的**直接依赖改为 peer 依赖**：需项目显式安装并锁定 vite；因此 **vite 与 vitest 必须同 PR 升级**，`@vitest/coverage-v8` 同升 5.x。
- 项目 `vitest.config.ts` 使用 `environment:'jsdom'`、`coverage.provider:'v8'`、`coverage.thresholds`（lines/statements/functions 60、branches 50）——这些 API 在 v5 仍保留，但需复核 thresholds 字段与 jsdom 版本兼容（jsdom 25 建议复核）。

### 4.6 Node 20 → 24
- Node 20 已于 **2026-04-30 EOL**；Node 24 为 Active LTS（EOL 2028-04-30，latest 24.22.2）。**升级合理且必要**。
- 且 Vitest 5 要求 Node ≥22.12，与 Node24 一致 → **Node 升 24 是 vite8+vitest5 的前置**。
- 需同步修改：`.nvmrc`、`.node-version`、`ci.yml:18` 与 `:56`、`release.yml:30` 与 `:102`（本机当前 Node v24.12.0 已可本地验证）。工作区 WIP 已把 `.nvmrc/.node-version` 改为 24，但 CI/release 的 `node-version: 20` 仍需同步（属 task-12 实施范围）。

### 4.7 升级顺序与联动约束
`Node24 → (vite8 + vitest5 + coverage-v8 同 PR) → rand0.9/thiserror2（可独立 Rust PR）`。dependabot 会各自单开大版本 PR，**不可逐个大版本盲合**，建议人工 grouped PR。

### 4.8 风险与验收清单
| 项 | 风险 | 验收命令 / 标准 |
|---|---|---|
| rand 0.9 | 低 | `cargo tree -i rand` 仅 0.9；无 `thread_rng/gen_range/gen_bool/gen(` 残留；`cargo test --workspace` 全绿 |
| thiserror 2 | 低 | `cargo tree -i thiserror` 仅 2.x；build/test 通过 |
| vite 8 | 中 | `npm run build` 成功；产物对比体积/资源无回归 |
| vitest 5 | 中 | `npm test` 21 例通过；`npm run test:coverage` 阈值仍达标；三包主版本一致 |
| Node 24 | 低 | `.nvmrc/.node-version=24`；ci.yml/release.yml 均 24；本地 `npm ci`+全套通过 |
| 汇总 | — | 全量门禁：typecheck/lint/format:check/test/build + cargo fmt/clippy -D warnings/test 全绿；`npm run check:version` 通过 |

**验收标准（DoD 对齐）**：升级后 `cargo test --workspace`、`cargo clippy --workspace --all-targets -D warnings`、`cargo fmt --check`、`npx tsc --noEmit`、ESLint/Prettier、`npm test`、`npm run build`、`npm ci` 在干净环境全部通过；锁文件无重复/冲突主版本。

---

## 5. 交叉影响、建议顺序与阻断项

- **建议顺序**：D-1（Node24+vite8+vitest5 一组；rand/thiserror 一组）→ D-2（macOS job）→ D-3（Android 官方流程，需先升 CLI）。D-2/D-3 互不依赖，可并行。
- **联动强约束**：vitest5 ⇒ Node≥22.12 ⇒ Node24；vite8 与 vitest5 必须同升；@vitest/coverage-v8 随 vitest 主版本。
- **阻断项**：本评估未见**硬阻断**。两类"条件性风险"需在实施分支先行验证：(a) D-3 依赖 CLI 版本对 #13357 的修复程度；(b) D-1 三包联动必须一次性升级，否则 vitest5 在 Vite<6.4/Node<22.12 下不工作。
- **对 WIP 的观察（非本任务结论）**：工作区已有 rand 0.9/thiserror 2.0、`thread_rng→rng` 迁移、Node=24、`withGlobalTauri:false`（Q-7）等未提交改动，方向与本报告一致；但其**尚未提交且未跑全量门禁**，不作为验收结论，final gate 需以提交后 CI 为准。

---

## 6. 证据索引

- release.yml 矩阵/参数：`.github/workflows/release.yml:13-18、:52-86`；Android workaround：`:88-262`（python：`:146-232`，校验：`:234-243`，build：`:246`）
- ci/release Node 版本：`.github/workflows/ci.yml:18,56`、`.github/workflows/release.yml:30,102`
- 依赖基线：`Cargo.toml:14-15`、`package.json:24,34-35`、`package-lock.json`（vite 5.4.21 / vitest 2.1.9）
- thiserror 用法：`eyemotion-core/src/error.rs:5-24`；rand 用法：`eyemotion-core/src/ball.rs:34,40`、`game_state.rs:107-108,229-268`
- 平台配置：`src-tauri/tauri.conf.json:31,50-52`；图标 `src-tauri/icons/icon.icns`
- 外部依据：Tauri issue #13902（settings.gradle 由 build 生成）、#13357（assembleUniversalRelease，open）、`v2.tauri.app/distribute/google-play`、Vite 8 公告（Node 20.19+/22.12+、Rolldown）、Vitest 5 迁移页（Vite≥6.4、Node≥22.12）、endofLife.date Node 20 EOL=2026-04-30。

---
_本报告为只读评估，未修改任何代码/配置。_
