use std::env;

fn main() {
    let now = time::OffsetDateTime::now_utc();
    let build_date = format!("{}-{:02}-{:02}", now.year(), now.month() as u8, now.day());
    println!("cargo:rustc-env=BUILD_DATE={build_date}");
    println!("cargo:rerun-if-changed=build.rs");

    ios_webview_fixes();
}

/// iOS WKWebView runtime patches, ported verbatim from the Leptos site
/// (`leptos-ui/__HideKeyboardAccessory.m` + `__DisableContentInsetAdjustment.m`,
/// which it compiles into the Tauri xcodegen project). The Dioxus app is native
/// dioxus-mobile with no xcodeproj, so compile the same `.m` files straight into
/// the binary with `cc`. Each file is an `__attribute__((constructor))`, so it
/// runs at launch with no call site; `-force_load` keeps the objects from being
/// dead-stripped.
///
/// - `__HideKeyboardAccessory.m`: swaps `-[WKContentView inputAccessoryView]` for
///   `nil` so the grey prev/next/Done bar stops covering the bottom nav.
/// - `__DisableContentInsetAdjustment.m`: forces
///   `scrollView.contentInsetAdjustmentBehavior = .never` so iOS does not add its
///   own safe-area inset on top of the CSS `env(safe-area-inset-*)` handling.
fn ios_webview_fixes() {
    println!("cargo:rerun-if-changed=__HideKeyboardAccessory.m");
    println!("cargo:rerun-if-changed=__DisableContentInsetAdjustment.m");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("ios") {
        return;
    }

    let lib = "rust_ui_ios_webview_fixes";
    cc::Build::new()
        .file("__HideKeyboardAccessory.m")
        .file("__DisableContentInsetAdjustment.m")
        .flag("-fobjc-arc")
        .compile(lib);

    let Ok(out_dir) = env::var("OUT_DIR") else {
        println!("cargo:warning=OUT_DIR unset; skipping -force_load for {lib}");
        return;
    };
    println!("cargo:rustc-link-arg=-Wl,-force_load,{out_dir}/lib{lib}.a");
    println!("cargo:rustc-link-lib=framework=WebKit");
    println!("cargo:rustc-link-lib=framework=UIKit");
    println!("cargo:rustc-link-lib=framework=Foundation");
}
