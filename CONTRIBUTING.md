# Contributing to Rust/UI

Thanks for helping build Rust/UI, a Dioxus component registry for Rust fullstack apps.

## Before opening an issue

- Search open and closed issues.
- Ask usage and support questions in [Discussions](https://github.com/rust-ui/ui/discussions/new?category=general).
- For bugs, include exact reproduction steps, expected and actual behavior, evidence, a minimal reproduction, version, and system info.
- Reports without actionable evidence may be closed until updated.

## Pull requests

- Open an issue first for substantial changes.
- Keep each PR focused.
- Use `Related to #N` unless the PR fully fixes the issue. Use `Closes #N` only when automatic issue closure is intended.
- Include screenshots or recordings for UI changes.
- Update `CHANGELOG_DEV.md` for behavior or workflow changes.

## Local checks

```bash
git config core.hooksPath .githooks
cargo fmt --check
cargo clippy -p <touched-crate> --no-deps
```

Run the narrowest relevant tests. For browser UI changes, verify the actual rendered page and capture a screenshot.

## Commit style

Use conventional messages such as:

```text
fix(button): correct disabled hover state
feat(cli): add registry validation command
docs: update contribution workflow
```
