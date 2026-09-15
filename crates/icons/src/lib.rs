#![allow(ambiguous_glob_reexports)]

// ----- Common -----
pub mod common;

// ----- Leptos -----
#[cfg(feature = "leptos")]
pub mod leptos;
// Re-export all icons from leptos compatibility layer (registry pattern)
#[cfg(feature = "leptos")]
pub use leptos::*;

// ----- Leptos Animated -----
#[cfg(feature = "leptos_animated")]
pub mod leptos_animated;
// Re-export all animated icons
#[cfg(feature = "leptos_animated")]
pub use leptos_animated::*;

// ----- Dioxus -----
#[cfg(feature = "dioxus")]
pub mod dioxus;
#[cfg(feature = "dioxus")]
pub use dioxus::*;

// ----- Dioxus Animated -----
#[cfg(feature = "dioxus_animated")]
pub mod dioxus_animated;
#[cfg(feature = "dioxus_animated")]
pub use dioxus_animated::*;
