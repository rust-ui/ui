# Rust/UI
Inspired by shadcn/ui, built for Rust fullstack apps. A component registry for Leptos — built with Tailwind CSS, copy-paste ready.
[![Rust/UI](https://www.rust-ui.com/og-image.png)](https://www.rust-ui.com)


## Why Rust/UI?
Rust/UI isn’t a component library you install as a crate. It’s a collection of re-usable components that you copy and paste into your apps.

- Full Control: The code is yours. No black-box UI framework crates to fight against.
- Styling: Built with Tailwind CSS for easy customization.
- Framework: Designed specifically for the Leptos fullstack ecosystem.
- Type Safe: Leverages Rust’s powerful type system for component props.


## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=rust-ui/ui&type=Date)](https://star-history.com/#rust-ui/ui&Date)


## Ecosystem
Rust/UI is backed by a set of purpose-built crates:

| Crate | Description |
|-------|-------------|
| [`tw-merge`](https://crates.io/crates/tw-merge) | Tailwind class merging utility |
| [`icons`](https://crates.io/crates/icons) | Icon components for Leptos |
| [`ui-cli`](https://crates.io/crates/ui-cli) | CLI for adding components to your project |

**Starters**

| Repo | Description |
|------|-------------|
| [start-tauri-fullstack](https://github.com/rust-ui/start-tauri-fullstack) | Leptos + Tauri fullstack starter |
| [start-tauri](https://github.com/rust-ui/start-tauri) | Tauri starter with Rust/UI |


## Getting Started
Visit [rust-ui.com](https://www.rust-ui.com) to browse components and get install commands.

### Quick Start
1. Ensure you have the following installed:
    - [Tailwind CSS](https://tailwindcss.com/docs/installation/tailwind-cli)
    - [Cargo Leptos](https://github.com/leptos-rs/cargo-leptos) 
    
    Install cargo-leptos:
    ```bash
    cargo install --locked cargo-leptos
    ```

2. Browse the [registry](https://www.rust-ui.com/), find a component (e.g., `Button`), and copy the source into your project's components/ directory.
  ```rust
  use crate::components::ui::button::{Button, ButtonVariant};
  
  #[component]
  pub fn App() -> impl IntoView {
      view! {
          <Button variant=ButtonVariant::Outline on:click=move |_| {}>
              "Click Me"
          </Button>
      }
  }
  ```


## Development
If you'd like to contribute or preview the registry locally:

> **Windows users:** See [README_WINDOWS.md](./README_WINDOWS.md) for setup troubleshooting (Perl conflict, OpenSSL).

1. Clone the repo
    ```bash
    git clone git@github.com:rust-ui/ui.git rust-ui 
    cd rust-ui
    ```

2. Install JS dependencies (for Tailwind/Tooling)
    ```bash
    pnpm install
    ```

3. Install Rust dependencies (Leptos)
    ```bash
    cargo install --locked cargo-leptos
    ```
    
   ### Linker Optimization (Optional)
    *Note for macOS Users*: To use the faster lld linker, install it via Homebrew and ensure it's in your PATH, or configure your .cargo/config.toml to point to the absolute path.
    ```bash
    brew install lld
    ```

4. Run the dev server
    ```bash
    cargo leptos watch 
    ``` 

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).


## Contributors

<a href="https://github.com/rust-ui/ui/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=rust-ui/ui" />
</a>

## License
Distributed under the MIT License. See [LICENSE](LICENSE) for more information. Crafted by [Max Wells](https://www.rust-ui.com)
