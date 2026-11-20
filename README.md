<div align="center">

# xkdPlanner

*A desktop organization tool built in Rust, with a React + TypeScript Tauri front-end.*

![Rust](https://img.shields.io/badge/Rust-1.90%2B-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri)
![React](https://img.shields.io/badge/React-19.x-61dafb?logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript-5.x-3178c6?logo=typescript)
![Vite](https://img.shields.io/badge/Vite-6.x-646cff?logo=vite)
![Edition](https://img.shields.io/badge/edition-2024-lightgrey)
![License](https://img.shields.io/badge/license-MIT-green)

</div>

---

## Current status

The project is a skeleton MVC desktop app. Rust, Tauri, and React/TypeScript files compile, but command bodies are still `todo!()` and will panic until implemented. Layout is Option C:

```text
xkdPlanner/
├── Cargo.toml
├── model/                 # Model crate
├── controller/            # Controller + services crate
├── view/
│   ├── desktop/           # Tauri host
│   └── web/               # React + Vite UI
├── scripts/
├── docs/
├── .zed/
└── dist/
```

## Commands

```bash
npm --prefix view/web install
npm --prefix view/web run dev
npm --prefix view/web run build
npm --prefix view/web run lint
npm --prefix view/web run format
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
./scripts/build_app.sh
./scripts/run_app.sh
```

## Development

- Rust hover docs and completions come from rust-analyzer.
- TypeScript/React hover docs and completions come from the TS language server.
- Zed project settings are in `.zed/`; tasks are in `.zed/tasks.json`.
- Snippets are documented in `docs/zed-snippets.md`.
- Zed setup is documented in `docs/zed.md`.

## Logs

- `./scripts/run_app.sh` starts the app and writes logs to `logs/`.
- Rust/Tauri logs stream to the terminal and log file.
- Webview console logs are not captured by the script; use webview devtools when needed.

## License

Released under the MIT License. See [LICENSE](LICENSE).