# Rust/UI — Dioxus

Inspired by shadcn/ui, built for Rust fullstack apps. A component registry for Dioxus — Tailwind CSS, copy-paste ready.

[![Rust/UI](https://www.rust-ui.com/og-image.png)](https://www.rust-ui.com)

> **Early stage, experimental** — expect breaking changes. Open to contributors.

> **Not affiliated with or endorsed by the Dioxus team.** This is an independent personal project.

## Why Rust/UI?

Rust/UI is not a component library you install as a crate. It is a collection of reusable components that you copy into your Dioxus applications.

- **Full control:** The code is yours.
- **Styling:** Built with Tailwind CSS v4.
- **Framework:** Designed for the Dioxus fullstack ecosystem.
- **Type safe:** Uses Rust's type system for component props.

## Stack

| Tool | Role |
|------|------|
| [Dioxus](https://dioxuslabs.com/) 0.7 | Fullstack Rust UI framework |
| [Tailwind CSS](https://tailwindcss.com/) v4 | Styling |
| [`tw_merge`](https://crates.io/crates/tw_merge) | Tailwind class merging |
| [`icons`](https://crates.io/crates/icons) | Lucide icons for Dioxus |

## Getting Started

Visit [rust-ui.com](https://www.rust-ui.com) to browse components and get install commands.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) and [pnpm](https://pnpm.io/installation) for Tailwind tooling
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/)

Install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

### Use a component

Browse the [registry](https://www.rust-ui.com/), find a component, and copy its source into your project's `components/` directory.

## Development

Clone the repository, install JavaScript dependencies, then start Dioxus:

```bash
git clone https://github.com/rust-ui/dioxus-ui.git
cd dioxus-ui
pnpm install
dx serve
```

`dx serve` automatically starts the Tailwind watcher and generates `assets/tailwind.css` from the root `tailwind.css` file. No second CSS process needed.

Useful targets:

```bash
dx serve --web --fullstack
dx serve --platform desktop
dx serve --platform ios
```

Open [http://127.0.0.1:8080](http://127.0.0.1:8080).

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=rust-ui/ui&type=Date)](https://star-history.com/#rust-ui/ui&Date)

## License

MIT — see [LICENSE](./LICENSE).
