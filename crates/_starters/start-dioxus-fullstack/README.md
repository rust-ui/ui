# Dioxus Fullstack Starter

## Requirements

You need to have Dioxus (nightly), Just, and PostgreSQL on your machine.
If not already, you can refer to [PREREQUISITES.md](PREREQUISITES.md).

## CLI

```bash
cargo install ui-cli

ui starters
# └─> start-dioxus-fullstack
```

## Setup the project

```bash
# Install Tailwind CSS
pnpm install

# Create the DB (+ run migrations)
just reset_db
```

## Run the project

```bash
# Terminal 1 — Tailwind watch
pnpm css

# Terminal 2 — Dioxus dev server
dx serve
```

## License

MIT License - see [LICENSE](LICENSE) for details.
