---
title: "Area Chart Placeholder"
name: "area_chart_placeholder"
cargo_dependencies: []
registry_dependencies: ["card"]
type: "components:charts"
path: "charts/area_chart_placeholder.rs"
---

# Area Chart Placeholder

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add area_chart_placeholder
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::card::Card;

#[component]
pub fn AreaChartPlaceholder() -> impl IntoView {
    view! {
        <Card class="hidden border-dashed shadow-none md:block h-[400px] bg-muted">
            <div />
        </Card>
    }
}
```
