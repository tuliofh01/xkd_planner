# Zed snippets

Zed loads custom snippets from your user config directory (project-local snippet files are not automatically loaded by Zed).

## Provided snippets
- `docs/zed-snippets/rust.json`
- `docs/zed-snippets/typescript.json`
- `docs/zed-snippets/tsx.json`

## Install (Linux)
```bash
mkdir -p ~/.config/zed/snippets
cp docs/zed-snippets/rust.json ~/.config/zed/snippets/rust.json
cp docs/zed-snippets/typescript.json ~/.config/zed/snippets/typescript.json
cp docs/zed-snippets/tsx.json ~/.config/zed/snippets/tsx.json
```

Restart Zed or reload snippets if needed. In `.zed/settings.json` this repo sets `snippet_sort_order` to prioritize snippets in completions.

## Notes
- Snippets complement LSP completions and hover docs.
- Add your own user snippets without committing personal preferences to the repo.
