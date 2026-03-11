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







## Components

- **Tabs**: Main wrapper with `default_value` and `orientation` props
- **TabsList**: Tab button container with `variant` prop
- **TabsTrigger**: Clickable tab button that activates its content
- **TabsContent**: Content panel shown when its tab is active


## Usage

```rust
use crate::components::ui::tabs::{
    Tabs,
    TabsList,
    TabsTrigger,
    TabsContent,
};
```

```rust
<Tabs default_value="account">
    <TabsList>
        <TabsTrigger value="account">"Account"</TabsTrigger>
        <TabsTrigger value="password">"Password"</TabsTrigger>
    </TabsList>
    <TabsContent value="account">"Account settings."</TabsContent>
    <TabsContent value="password">"Change your password."</TabsContent>
</Tabs>
```


## Examples

### Default

<StaticTabs />

### Line

Underline indicator variant via `variant=TabsVariant::Line` on `TabsList`.

<StaticTabsLine />

### Vertical

Vertical tab layout via `orientation=TabsOrientation::Vertical` on `Tabs`.

<StaticTabsVertical />


### RTL

Tab bar with Arabic labels. Active indicator, content alignment, and tab ordering all respect the RTL reading direction.

<StaticTabsRtl />

## See Also

- [Accordion](/docs/components/accordion)
- [Bottom Nav](/docs/components/bottom-nav)
