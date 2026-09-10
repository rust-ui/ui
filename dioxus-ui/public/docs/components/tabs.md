+++
title = "Tabs"
description = "Rust/UI component that displays a set of layered sections of content, known as tab pages, that are displayed one at a time."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/tabs.webp"
image_dark = "/images/thumbnails/tabs-dark.webp"
+++

<StaticTabs />

## Installation

<StaticInstallTabs />

## Usage

```rust
use crate::ui::tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
```

```rust
rsx! {
    Tabs { default_value: "account",
        TabsList {
            TabsTrigger { value: "account", "Account" }
            TabsTrigger { value: "password", "Password" }
        }
        TabsContent { value: "account",
            p { "Change your account settings here." }
        }
        TabsContent { value: "password",
            p { "Update your password here." }
        }
    }
}
```

## See Also

- [Card](/components/card)
- [Separator](/components/separator)
