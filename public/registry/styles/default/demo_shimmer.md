---
title: "Demo Shimmer"
name: "demo_shimmer"
cargo_dependencies: []
registry_dependencies: ["button", "card", "shimmer"]
type: "components:demos"
path: "demos/demo_shimmer.rs"
---

# Demo Shimmer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_shimmer
```

## Component Code

```rust
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::components::ui::shimmer::Shimmer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub title: String,
    pub description: String,
}

/// Simulates a database fetch with 1 second delay
#[server]
pub async fn fetch_card_data() -> Result<CardData, ServerFnError> {
    // Simulate network/database latency (only on server)
    #[cfg(feature = "ssr")]
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(CardData {
        title: "Fetched Title".to_string(),
        description: "This content was fetched from the server after a 1 second simulated delay. The shimmer effect automatically showed during the loading period.".to_string(),
    })
}

#[component]
pub fn DemoShimmer() -> impl IntoView {
    // Loading state
    let loading = RwSignal::new(false);

    // Store fetched data
    let card_data = RwSignal::new(None::<CardData>);

    // Fetch handler using spawn_local for reliable repeated calls
    let on_fetch = move |_| {
        spawn_local(async move {
            loading.set(true);
            let result = fetch_card_data().await;
            if let Ok(data) = result {
                card_data.set(Some(data));
            }
            loading.set(false);
        });
    };

    view! {
        <div class="flex flex-col gap-4">
            <div class="flex gap-2">
                <Button variant=ButtonVariant::Outline on:click=move |_| loading.set(!loading.get())>
                    "Toggle Loading"
                </Button>
                <Button variant=ButtonVariant::Default on:click=on_fetch>
                    "Fetch Data (1s)"
                </Button>
            </div>

            <Shimmer loading=Signal::from(loading)>
                <Card class="max-w-lg lg:max-w-2xl">
                    <CardHeader>
                        <CardTitle>
                            {move || {
                                card_data.get().map(|data| data.title).unwrap_or_else(|| "Card Title".to_string())
                            }}
                        </CardTitle>
                    </CardHeader>

                    <CardContent>
                        <CardDescription>
                            {move || {
                                card_data
                                    .get()
                                    .map(|data| data.description)
                                    .unwrap_or_else(|| {
                                        "Click 'Toggle Loading' for manual control, or 'Fetch Data' to simulate a real server call with 1 second delay."
                                            .to_string()
                                    })
                            }}
                        </CardDescription>
                    </CardContent>

                    <CardFooter class="justify-end">
                        <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                        <Button>"Confirm"</Button>
                    </CardFooter>
                </Card>
            </Shimmer>
        </div>
    }
}
```
