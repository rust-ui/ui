#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use crate::domain::bug_report::bug_reports::{BugReportRequest, BugType, report_client_bug};

#[cfg(target_arch = "wasm32")]
pub fn set_stack_trace_limit(n: u32) -> Result<(), JsValue> {
    use js_sys::{Reflect, global};

    let global_obj = global();
    let error_constructor = Reflect::get(&global_obj, &"Error".into())?;
    Reflect::set(&error_constructor, &"stackTraceLimit".into(), &n.into())?;
    Ok(())
}

/// Initializes client-side diagnostic handlers for WASM.
///
/// This function sets up:
/// 1. JavaScript's `Error.stackTraceLimit` for better stack traces
/// 2. Panic hooks for console logging and automatic bug reporting
/// 3. Console warning interception for framework and browser warnings
///
/// Note: User agent and URL are extracted server-side from HTTP headers.
#[cfg(target_arch = "wasm32")]
pub fn init() {
    use std::sync::atomic::{AtomicBool, Ordering};

    use dioxus::logger::tracing::error;

    // `App()` re-runs on every render; the panic-hook chain and console.warn
    // patch must be installed exactly once.
    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    if INITIALIZED.swap(true, Ordering::Relaxed) {
        return;
    }

    if let Err(err) = set_stack_trace_limit(30) {
        error!("Failed to set Error.stackTraceLimit: {err:?}");
    }

    add_bug_report_handler();

    if let Err(err) = intercept_console_warnings() {
        error!("Failed to intercept console.warn: {err:?}");
    }
}

/// Sends a bug report via server function (saves to SQLite + forwards to RUSTIFY).
#[cfg(target_arch = "wasm32")]
async fn send_bug_report(report: BugReportRequest) -> Result<(), String> {
    report_client_bug(report).await.map_err(|err| format!("Failed to send bug report: {err}"))
}

/// Adds an additional panic handler that automatically reports bugs to RUSTIFY.
/// User agent and URL are extracted server-side from HTTP headers.
#[cfg(target_arch = "wasm32")]
fn add_bug_report_handler() {
    use dioxus::logger::tracing::error;

    let previous_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        previous_hook(panic_info);

        let message = panic_info.to_string();
        let location = panic_info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown location".to_string());

        let stack_trace = get_combined_stack_trace();

        let report = BugReportRequest::new(BugType::Wasm, format!("Panic at {location}: {message}"))
            .with_stack_trace(stack_trace);

        wasm_bindgen_futures::spawn_local(async move {
            if let Err(err) = send_bug_report(report).await {
                error!("Error sending bug report: {err}");
            }
        });
    }));
}

/// Creates a new JavaScript Error object and extracts its stack property.
#[cfg(target_arch = "wasm32")]
fn get_javascript_stack_trace() -> String {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        type Error;

        #[wasm_bindgen(constructor)]
        fn new() -> Error;

        #[wasm_bindgen(method, getter)]
        fn stack(this: &Error) -> String;
    }

    Error::new().stack()
}

/// Captures both Rust and JavaScript stack traces for comprehensive debugging.
#[cfg(target_arch = "wasm32")]
fn get_combined_stack_trace() -> String {
    use std::backtrace::Backtrace;

    let rust_trace = Backtrace::capture().to_string();
    let js_trace = get_javascript_stack_trace();

    format!("=== JavaScript Stack Trace ===\n{js_trace}\n\n=== Rust Backtrace ===\n{rust_trace}")
}

/// Patterns that indicate a framework-specific warning worth capturing.
#[cfg(target_arch = "wasm32")]
const LEPTOS_WARNING_PATTERNS: &[&str] = &[
    "outside of Suspense",
    "outside a Suspense",
    "reactive context",
    "signal was disposed",
    "reading a resource",
    "without a Transition",
];

/// Warnings matching these patterns will NOT be saved as bug reports.
#[cfg(target_arch = "wasm32")]
const IGNORED_WARNING_PATTERNS: &[&str] = &["Live-reload stopped"];

/// Checks if a warning should be ignored (not saved as a bug report).
#[cfg(target_arch = "wasm32")]
fn should_ignore_warning(message: &str) -> bool {
    IGNORED_WARNING_PATTERNS.iter().any(|pattern| message.contains(pattern))
}

/// Checks if a message matches any framework warning pattern.
#[cfg(target_arch = "wasm32")]
fn is_leptos_warning(message: &str) -> bool {
    let lower = message.to_lowercase();
    LEPTOS_WARNING_PATTERNS.iter().any(|pattern| lower.contains(&pattern.to_lowercase()))
}

/// Intercepts console.warn to capture framework-specific warnings and report them.
/// User agent and URL are extracted server-side from HTTP headers.
#[cfg(target_arch = "wasm32")]
fn intercept_console_warnings() -> Result<(), wasm_bindgen::JsValue> {
    use js_sys::{Array, Function, Reflect};
    use wasm_bindgen::prelude::*;

    let window = web_sys::window().ok_or("no window")?;
    let console = Reflect::get(&window, &"console".into())?;
    let original_warn = Reflect::get(&console, &"warn".into())?;
    let original_warn_fn: Function = original_warn.dyn_into()?;

    let original_warn_clone = original_warn_fn.clone();
    let console_clone = console.clone();

    let closure = Closure::<dyn Fn(JsValue)>::new(move |args: JsValue| {
        let args_array = Array::new();
        args_array.push(&args);
        let _ = original_warn_clone.apply(&console_clone, &args_array);

        let message = args.as_string().unwrap_or_default();
        if message.is_empty() || should_ignore_warning(&message) {
            return;
        }

        let bug_type = if is_leptos_warning(&message) { BugType::LeptosWarning } else { BugType::BrowserWarning };

        let stack_trace = get_javascript_stack_trace();

        let report = BugReportRequest::new(bug_type, message).with_stack_trace(stack_trace);

        wasm_bindgen_futures::spawn_local(async move {
            let _ = send_bug_report(report).await;
        });
    });

    Reflect::set(&console, &"warn".into(), closure.as_ref())?;
    closure.forget(); // Prevent closure from being dropped

    Ok(())
}
