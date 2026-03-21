# Rust/UI
Inspired by shadcn/ui, built for Rust fullstack apps. A component registry for Leptos — built with Tailwind CSS, copy-paste ready.
[![Rust/UI](https://www.rust-ui.com/og-image.png)](https://www.rust-ui.com)


## Why Rust/UI?
Rust/UI isn't a component library you install as a crate. It’s a collection of re-usable components that you copy and paste into your apps.

- Full Control: The code is yours. No node_modules or opaque crate dependencies for your UI.
- Styling: Built with Tailwind CSS for easy customization.
- Framework: Designed specifically for the Leptos fullstack ecosystem.
- Type Safe: Leverages Rust's powerful type system for component props.


## Getting Started
Visit [rust-ui.com](https://www.rust-ui.com) to browse components and get install commands.

### Quick Start
1. Ensure you have the following installed:
    - [Tailwind CSS](https://tailwindcss.com/docs/installation/tailwind-cli)
    - [Cargo Leptos](https://github.com/leptos-rs/cargo-leptos) 

2. Add Dependencies
  
  Add the necessary support crates to your Cargo.toml:

  ```toml
  [dependencies]
  leptos = { version = "0.6", features = ["hydrate"] }
  lucide-leptos = "0.1" # Common for icons
  # Add any other required utils mentioned in component docs
  ```

3. Browse the [registry](https://www.rust-ui.com/), find a component (e.g., `Button`), and copy the source into your project's components/ directory.
  ```rust
  use crate::components::ui::button::Button;
  
  #[component]
  pub fn App() -> impl IntoView {
      view! {
          <Button variant="outline" on:click=|_| println!("Clicked!")>
              "Click Me"
          </Button>
      }
  }
  ```


## Development
If you want to run this project locally to preview components or contribute:

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
  
    *Install `lld` to be used for linking instead of the default `ld` linker
    ```bash
    brew install lld
    ```

4. Run the dev server
    ```bash
    cargo leptos watch 
    ``` 

## Contribution
We love contributions! Whether it's a new component, a bug fix, or a CSS tweak.

- Fork the Project [Currently Disabled].
- Create your Feature Branch (git checkout -b feature/AmazingComponent).
- Commit your Changes (git commit -m 'Add some AmazingComponent').
- Push to the Branch (git push origin feature/AmazingComponent).
- Open a Pull Request.


## License
Distributed under the MIT License. See [LICENSE](LICENSE) for more information. Crafted by [Max Wells](https://www.rust-ui.com)
