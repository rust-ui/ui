# Contributing

Rust/UI is the shared tooling and registry repository for the Rust UI ecosystem.

Repository ownership:

- Leptos website changes belong in `leptos-ui`.
- Dioxus website changes belong in `dioxus-ui`.
- Shared crates, CLI, starters, registry artifacts, and governance belong here.

## Development

Clone repository:

```bash
git clone git@github.com:rust-ui/ui.git rust-ui
cd rust-ui
```

Run shared workspace checks:

```bash
cargo metadata --no-deps --format-version 1
cargo check --workspace
```

Website development uses its owning repository. Do not add website source or deployment files here.

## Scope

- Shared crate fixes and features.
- CLI and starter improvements.
- Registry generation and generated registry artifacts.
- Cross-repository governance and tooling.

Use the relevant repository for website-specific changes.

## Commit convention

Use `category(scope): message`:

`feat`, `fix`, `docs`, `refactor`, `build`, `test`, `ci`, or `chore`.

## License

MIT. See [LICENSE](LICENSE).
