---
title: "Demo Dropzone"
name: "demo_dropzone"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_dropzone.rs"
---

# Demo Dropzone

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dropzone
```

## Component Code

```rust
use leptos::prelude::*;

#[component]
pub fn DemoDropzone() -> impl IntoView {
    view! {
        <div class="relative my-6">
            <input id="id-dropzone01" name="file-upload" accept=".gif,.jpg,.png,.jpeg" type="file" class="hidden" />
            <label
                for="id-dropzone01"
                class="flex relative flex-col gap-4 items-center py-6 px-3 text-sm font-medium text-center rounded border border-dashed transition-colors cursor-pointer border-slate-300"
            >
                <span class="inline-flex justify-center items-center self-center px-3 h-12 rounded-full bg-slate-100/70 text-slate-400">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        aria-label="File input icon"
                        role="graphics-symbol"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="size-6"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M12 16.5V9.75m0 0 3 3m-3-3-3 3M6.75 19.5a4.5 4.5 0 0 1-1.41-8.775 5.25 5.25 0 0 1 10.233-2.33 3 3 0 0 1 3.758 3.848A3.752 3.752 0 0 1 18 19.5H6.75Z"
                        />
                    </svg>
                </span>
                <span class="text-slate-500">Drag & drop or <span class="text-emerald-500">upload a file</span></span>
            </label>
        </div>
    }
}
```
