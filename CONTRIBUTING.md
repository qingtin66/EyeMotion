# 贡献指南（Contributing to EyeMotion）

感谢参与 EyeMotion！本项目遵循 [Conventional Commits](https://www.conventionalcommits.org/) 与“先门禁后重构”的工程原则。

## 1. 环境准备
| 依赖 | 版本 | 说明 |
|---|---|---|
| Node.js | 20.x（见 `.nvmrc`） | 前端构建与测试 |
| Rust | stable 1.92（见 `rust-toolchain.toml`，含 rustfmt + clippy） | 后端 |
| Tauri CLI | 2.9.6（devDependency） | 打包 |

首次安装依赖请用可复现方式：
```bash
npm ci
```

## 2. 常用命令
| 命令 | 作用 |
|---|---|
| `npm run dev` | Vite 前端开发服务器 |
| `npm run build` | 构建前端产物到 `dist/` |
| `npm run typecheck` | `tsc --noEmit` 类型检查 |
| `npm run lint` | ESLint 检查 `ui`/`scripts` |
| `npm run format` / `format:check` | Prettier 格式化 / 校验 |
| `npm test` / `test:coverage` | Vitest 单元测试 / 覆盖率 |
| `npm run check:version` | 校验三处版本号一致（CI 强制） |
| `cargo fmt --all` | Rust 格式化 |
| `cargo clippy --workspace --all-targets -- -D warnings` | Rust 静态检查 |
| `cargo test --workspace` | Rust 全量测试 |

## 3. 提交前自检（与 CI 一致）
- [ ] `npm run check:version && npm run typecheck && npm run lint && npm run format:check`
- [ ] `npm test`（覆盖率行 ≥60%）
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`

## 4. 代码规范
- Rust：禁止生产代码 `unwrap()`/`expect()`；公共 API 必须有中文 `///` 文档；错误用 `Result` + `thiserror`/`log`。
- 前端：TypeScript 严格模式；颜色等品牌值取自后端主题或 CSS `--brand-*` 变量，不硬编码字面量。
- 任何版本号变更只改 SSOT（根 `Cargo.toml`），再同步其余两处并跑 `check:version`。

## 5. 分支与 PR
- 从 `main` 切分支：`feat/`、`fix/`、`refactor/`、`docs/`、`chore/`。
- 提交信息遵循 Conventional Commits（`feat:`/`fix:`/`refactor:`/`docs:`/`chore:`/`test:`/`ci:`）。
- PR 须描述动机、改动范围与验证方式；CI 全绿方可合并。
