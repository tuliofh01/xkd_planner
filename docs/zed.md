# Zed setup

This repository includes project-local Zed configuration in `.zed/`.

## Files
- `.zed/settings.json`: formatting, inlay hints, diagnostics, completion/signature/hover behavior, and language/LSP settings.
- `.zed/tasks.json`: common dev/build/check tasks (`task: spawn`).
- `.zed/debug.json`: CodeLLDB debug scenario for the desktop app (`debugger: start`).
- `.zed/EXTENSIONS.md`: recommended extensions.

## Hover docs and autocomplete
- Rust hover docs and completions come from `rust-analyzer` reading `///` doc comments and types.
- TypeScript/React hover docs and completions come from the TS language server reading types and JSDoc.
- Snippet completions are provided by user-level snippet files; see `docs/zed-snippets.md`.

## Recommended workflow
1. Open the repository root in Zed.
2. Install frontend dependencies: `npm --prefix view/web install`.
3. Use `task: spawn` to run `cargo: check`, `frontend: build`, or `app: dev run`.
4. Use `./scripts/run_app.sh` for a dev run with log output in `logs/`.

## Logging note
- Rust/Tauri logs stream to the terminal and `logs/dev-*.log` via `scripts/run_app.sh`.
- Webview console logs are not automatically captured by this script; open the webview devtools when needed.
