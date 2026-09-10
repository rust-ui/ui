---
title: "Use Form"
name: "use_form"
cargo_dependencies: ["serde"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_form.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Form

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_form
```

## Component Code

```rust
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

/// Trait alias for types that can be used with forms
pub trait FormData: Clone + Default + Serialize + for<'de> Deserialize<'de> + 'static {}

/// Blanket implementation for all types that satisfy the bounds
impl<T> FormData for T where T: Clone + Default + Serialize + for<'de> Deserialize<'de> + 'static {}

/// Type alias for form field value setter function
pub type SetValueFn = Arc<dyn Fn(&str, String)>;

/// Type alias for form field touch function (called on blur)
pub type TouchFieldFn = Arc<dyn Fn(&str)>;

pub struct Form<T> {
    pub values_signal: Signal<HashMap<String, String>>,
    pub errors_signal: Signal<HashMap<String, Option<String>>>,
    pub touched_signal: Signal<HashSet<String>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Clone for Form<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Form<T> {}

impl<T> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values_signal == other.values_signal
            && self.errors_signal == other.errors_signal
            && self.touched_signal == other.touched_signal
    }
}

impl<T> Default for Form<T>
where
    T: FormData,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Form<T>
where
    T: FormData,
{
    pub fn new() -> Self {
        Self {
            values_signal: use_signal(HashMap::default),
            errors_signal: use_signal(HashMap::default),
            touched_signal: use_signal(HashSet::default),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn value(&self, field: &str) -> String {
        self.values_signal.read().get(field).cloned().unwrap_or_default()
    }

    pub fn error(&self, field: &str) -> Option<String> {
        self.errors_signal.read().get(field).and_then(Clone::clone)
    }

    /// Updates the field value without triggering validation.
    pub fn set_value(&self, field: &str, value: String) {
        let field = field.to_string();
        let mut values_signal = self.values_signal;
        values_signal.with_mut(|values| {
            values.insert(field.clone(), value);
        });
    }

    /// Marks a field as touched (called on blur).
    pub fn touch_field(&self, field: &str) {
        let field = field.to_string();
        let mut touched_signal = self.touched_signal;
        touched_signal.with_mut(|touched| {
            touched.insert(field.clone());
        });
    }

    /// Check if a field has been touched (blurred at least once)
    pub fn is_touched(&self, field: &str) -> bool {
        self.touched_signal.read().contains(field)
    }

    fn map_to_struct(&self, values: &HashMap<String, String>) -> Option<T> {
        let default_value = serde_json::to_value(T::default()).ok()?;
        let mut default_map: HashMap<String, serde_json::Value> = serde_json::from_value(default_value).ok()?;

        for (key, value) in values {
            if value.is_empty() {
                continue;
            }

            let json_value = if let Ok(num) = value.parse::<i64>() {
                serde_json::Value::Number(num.into())
            } else if let Ok(num) = value.parse::<f64>() {
                serde_json::Number::from_f64(num)
                    .map(serde_json::Value::Number)
                    .unwrap_or_else(|| serde_json::Value::String(value.clone()))
            } else {
                serde_json::Value::String(value.clone())
            };
            default_map.insert(key.clone(), json_value);
        }

        serde_json::from_value(serde_json::to_value(default_map).ok()?).ok()
    }

    pub fn is_valid(&self) -> bool {
        self.errors_signal.read().values().all(Option::is_none)
    }

    pub fn reset(&self) {
        let mut values_signal = self.values_signal;
        values_signal.set(Default::default());
        let mut errors_signal = self.errors_signal;
        errors_signal.set(Default::default());
        let mut touched_signal = self.touched_signal;
        touched_signal.set(Default::default());
    }

    pub fn get_data(&self) -> Option<T> {
        self.map_to_struct(&self.values_signal.read())
    }

    pub fn validate_and_get(&self) -> Result<T, String> {
        let data = self
            .map_to_struct(&self.values_signal.read())
            .ok_or_else(|| "Please fill in all required fields.".to_string())?;
        Ok(data)
    }
}

pub fn use_form<T>() -> Form<T>
where
    T: FormData,
{
    Form::new()
}

#[derive(Clone)]
pub struct FormContext {
    pub values_signal: Signal<HashMap<String, String>>,
    pub errors_signal: Signal<HashMap<String, Option<String>>>,
    pub touched_signal: Signal<HashSet<String>>,
    pub set_value: SetValueFn,
    pub touch_field: TouchFieldFn,
}

#[derive(Clone)]
pub struct FieldContext {
    pub name: String,
}
```
