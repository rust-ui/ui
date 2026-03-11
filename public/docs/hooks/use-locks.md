+++
title = "Use Locks"
description = "Context hook for locking design params against randomization — each param can be individually toggled."
tags = ["utils"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticUseLocks />



## Installation

<StaticInstallUseLocks />



## Components

- **`UseLocks::init()`**: Initialize and provide the context at the page root
- **`use_locks()`**: Access the context from any child component
- **`LockableParam`**: Enum of all lockable design params



## Usage

```rust
use crate::components::hooks::use_locks::{LockableParam, UseLocks, use_locks};
```

```rust
// In page component:
let _ = UseLocks::init();

// In child component:
let locks = use_locks();
let is_locked = locks.is_locked(LockableParam::Font);

view! {
    <button on:click=move |_| locks.toggle_lock(LockableParam::Font)>
        {move || if is_locked.get() { "Locked" } else { "Unlocked" }}
    </button>
}
```



## API

| Method | Returns | Description |
|--------|---------|-------------|
| `is_locked(param)` | `Signal<bool>` | Reactive lock state for a param |
| `toggle_lock(param)` | `()` | Toggle lock on/off |
| `lock(param)` | `()` | Lock explicitly |
| `unlock(param)` | `()` | Unlock explicitly |
| `can_randomize(param)` | `Signal<bool>` | `true` when param is not locked |
| `locked_params()` | `HashSet<LockableParam>` | All locked params; tracked inside reactive closures |



## Examples

### Default

Click the lock icon to toggle each param. Locked params show a strikethrough and are skipped during randomization.

<StaticUseLocks />



## See Also

- [Use History](/docs/hooks/use-history)
