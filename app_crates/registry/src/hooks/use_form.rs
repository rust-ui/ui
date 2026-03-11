use std::collections::{HashMap, HashSet};

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use validator::Validate;

/// Trait alias for types that can be used with forms
pub trait FormData: Validate + Clone + Default + Serialize + for<'de> Deserialize<'de> + 'static {}

/// Blanket implementation for all types that satisfy the bounds
impl<T> FormData for T where T: Validate + Clone + Default + Serialize + for<'de> Deserialize<'de> + 'static {}

/// Type alias for form field value setter function
pub type SetValueFn = Box<dyn Fn(&str, String) + Send + Sync>;

/// Type alias for form field touch function (called on blur)
pub type TouchFieldFn = Box<dyn Fn(&str) + Send + Sync>;

pub struct Form<T> {
    pub values_signal: RwSignal<HashMap<String, String>>,
    pub errors_signal: RwSignal<HashMap<String, Option<String>>>,
    pub touched_signal: RwSignal<HashSet<String>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Clone for Form<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Form<T> {}

impl<T> Default for Form<T> {
    fn default() -> Self {
        Self {
            values_signal: RwSignal::new(Default::default()),
            errors_signal: RwSignal::new(Default::default()),
            touched_signal: RwSignal::new(Default::default()),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Form<T>
where
    T: FormData,
{
    pub fn value(&self, field: &str) -> String {
        self.values_signal.get().get(field).cloned().unwrap_or_default()
    }

    pub fn error(&self, field: &str) -> Option<String> {
        self.errors_signal.get().get(field).and_then(Clone::clone)
    }

    /// Updates the field value without triggering validation.
    /// Validation happens on blur via `touch_field`.
    pub fn set_value(&self, field: &str, value: String) {
        let field = field.to_string();

        self.values_signal.update(|values| {
            values.insert(field.clone(), value);
        });

        // If field was already touched, re-validate on each change
        if self.is_touched(&field) {
            let error = self.validate_field(&field);
            self.errors_signal.update(|errors| {
                errors.insert(field, error);
            });
        }
    }

    /// Marks a field as touched (called on blur) and validates it.
    /// Errors are only shown for touched fields.
    pub fn touch_field(&self, field: &str) {
        let field = field.to_string();

        self.touched_signal.update(|touched| {
            touched.insert(field.clone());
        });

        // Validate on blur
        let error = self.validate_field(&field);
        self.errors_signal.update(|errors| {
            errors.insert(field, error);
        });
    }

    /// Check if a field has been touched (blurred at least once)
    pub fn is_touched(&self, field: &str) -> bool {
        self.touched_signal.get().contains(field)
    }

    fn validate_field(&self, field: &str) -> Option<String> {
        let data = self.map_to_struct(&self.values_signal.get())?;

        data.validate().err()?.field_errors().get(field)?.first()?.message.as_ref().map(|m| m.to_string())
    }

    fn map_to_struct(&self, values: &HashMap<String, String>) -> Option<T> {
        let default_value = serde_json::to_value(T::default()).ok()?;
        let mut default_map: HashMap<String, serde_json::Value> = serde_json::from_value(default_value).ok()?;

        for (key, value) in values {
            // Skip empty values - keep the default value from the struct
            if value.is_empty() {
                continue;
            }

            // Try to parse as number first, fall back to string
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
        self.errors_signal.get().values().all(Option::is_none)
    }

    pub fn can_submit(&self) -> bool {
        let Some(data) = self.map_to_struct(&self.values_signal.get()) else {
            return false;
        };
        data.validate().is_ok()
    }

    pub fn reset(&self) {
        self.values_signal.set(Default::default());
        self.errors_signal.set(Default::default());
        self.touched_signal.set(Default::default());
    }

    pub fn get_data(&self) -> Option<T> {
        self.map_to_struct(&self.values_signal.get())
    }

    pub fn validate_and_get(&self) -> Result<T, String> {
        let data = self
            .map_to_struct(&self.values_signal.get())
            .ok_or_else(|| "Please fill in all required fields.".to_string())?;

        data.validate().map_err(|errors| {
            errors
                .field_errors()
                .values()
                .filter_map(|errs| errs.first())
                .filter_map(|err| err.message.as_ref())
                .map(|msg| msg.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        })?;

        Ok(data)
    }
}

pub fn use_form<T>() -> Form<T>
where
    T: FormData,
{
    Form::default()
}

#[derive(Clone, Copy)]
pub struct FormContext {
    pub values_signal: RwSignal<HashMap<String, String>>,
    pub errors_signal: RwSignal<HashMap<String, Option<String>>>,
    pub touched_signal: RwSignal<HashSet<String>>,
    pub set_value: StoredValue<SetValueFn>,
    pub touch_field: StoredValue<TouchFieldFn>,
}

#[derive(Clone)]
pub struct FieldContext {
    pub name: String,
}

/// Trait for structs that can automatically generate form fields.
///
/// This trait is automatically implemented by the `#[derive(AutoForm)]` macro.
pub trait AutoFormFields: FormData {
    /// Renders all form fields for this struct.
    fn render_fields(form: Form<Self>) -> impl IntoView;
}
