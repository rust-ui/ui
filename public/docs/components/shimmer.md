+++
title = "Shimmer"
description = "Auto-adapting skeleton loader that mirrors your DOM structure."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticShimmer />




## Installation

<StaticInstallShimmer />




## Usage

```rust
use crate::components::ui::shimmer::Shimmer;
```

```rust
let loading = RwSignal::new(true);

view! {
    <Shimmer loading=loading.into()>
        <MyCard data=data />
    </Shimmer>
}
```




## How It Works

Unlike static `<Skeleton>` components that require manual sizing, `<Shimmer>` automatically:

1. Detects "leaf" elements (text, images, buttons, inputs)
2. Measures their exact positions using `getBoundingClientRect()`
3. Creates shimmer blocks that perfectly match your content structure
4. Re-measures on resize for responsive layouts




## With Server Functions

```rust
#[component]
fn MyPage() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_data());
    let loading = Signal::derive(move || data.loading());

    view! {
        <Shimmer loading=loading>
            <Transition fallback=|| ()>
                {move || data.get().map(|result| {
                    view! { <UserCard user=result /> }
                })}
            </Transition>
        </Shimmer>
    }
}
```




## Light Mode Colors

For light backgrounds, adjust the colors:

```rust
<Shimmer
    loading=loading.into()
    shimmer_color="rgba(0,0,0,0.06)"
    background_color="rgba(0,0,0,0.04)"
>
    <MyCard />
</Shimmer>
```


## See Also

- [Skeleton](/docs/components/skeleton)
- [Spinner](/docs/components/spinner)
