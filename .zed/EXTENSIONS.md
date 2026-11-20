# Recommended Zed extensions

Install extensions from Zed's extension UI (Extensions cannot be vendored per project).

## Recommended
- Rust (rust-analyzer integration is built-in, but keep Rust tooling up to date)
- TOML
- HTML
- CSS
- ESLint
- Prettier
- CodeLLDB (for debugging)

## Notes
- Project settings live in `.zed/settings.json`.
- Tasks live in `.zed/tasks.json` and can be run with `task: spawn`.
- Debug scenarios live in `.zed/debug.json` and can be started with `debugger: start`.
- User-level preferences (theme, keymap, vim mode, auto-install extensions) belong in your personal Zed settings, not this repo.
