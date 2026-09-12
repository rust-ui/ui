fn main() {
    let now = time::OffsetDateTime::now_utc();
    let build_date = format!("{}-{:02}-{:02}", now.year(), now.month() as u8, now.day());
    println!("cargo:rustc-env=BUILD_DATE={build_date}");
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("ios") {
        purge_stale_simulator_webkit_cache();
        build_native_webkit_patches();
    }
}

/// Ported from leptos-ui's `__DisableContentInsetAdjustment.m` /
/// `__HideKeyboardAccessory.m`.
///
/// See justfile `run_ios`/`run_ipad`, which
/// copies them into the Tauri/xcodegen apple project as direct target
/// sources. `dx` has no xcodegen step: it compiles the crate as a staticlib
/// and links the bundle itself, and silently drops `-force_load`, so an
/// object file whose only symbol is an unreferenced `static` constructor
/// gets pruned by the linker's archive-member selection before the
/// constructor ever runs.
///
/// Fix: the `.m` files declare the constructor
/// functions with external linkage, and `src/main.rs` declares+calls them
/// via `unsafe extern "C"`, which forces the linker to pull the object files
/// in (the constructor then also fires automatically at load, same as
/// Leptos; the explicit call is just what guarantees inclusion here).
fn build_native_webkit_patches() {
    println!("cargo:rerun-if-changed=__DisableContentInsetAdjustment.m");
    println!("cargo:rerun-if-changed=__HideKeyboardAccessory.m");
    cc::Build::new()
        .file("__DisableContentInsetAdjustment.m")
        .file("__HideKeyboardAccessory.m")
        .flag("-fobjc-arc")
        .compile("webkit_ios_patches");
    println!("cargo:rustc-link-lib=framework=UIKit");
    println!("cargo:rustc-link-lib=framework=WebKit");
}

/// `dx serve --platform ios` reinstalls the app bundle on every relaunch but
/// never touches the simulator's separate `WebKit` data container.
///
/// A plain kill+relaunch keeps serving stale cached HTML/CSS/JS (see
/// `PLAN_FIX_BOTTOM_NAV.md` section 13). Purge it here so every ios build gets
/// a fresh `WKWebView` cache, same effect as `xcrun simctl uninstall` without
/// needing to reinstall the app.
///
/// Best-effort: no-op (and no build failure) if
/// `xcrun`/simctl is unavailable, no simulator is booted, or the app isn't
/// installed yet.
fn purge_stale_simulator_webkit_cache() {
    // Points rerun-if-changed at a path that never exists so cargo can never
    // cache this as "unchanged" and this runs on every `dx serve` launch, not
    // just when build.rs itself changes.
    let out_dir = std::env::var("OUT_DIR").unwrap_or_default();
    println!("cargo:rerun-if-changed={out_dir}/__force_rerun_never_exists");

    let Ok(list) = std::process::Command::new("xcrun")
        .args(["simctl", "list", "devices", "booted", "-j"])
        .output()
    else {
        return;
    };
    let Ok(text) = String::from_utf8(list.stdout) else {
        return;
    };
    let Some(udid) = text
        .lines()
        .find(|line| line.contains("\"udid\""))
        .and_then(|line| line.split('"').nth(3))
    else {
        return;
    };

    let home = std::env::var("HOME").unwrap_or_default();
    let data_apps_dir =
        format!("{home}/Library/Developer/CoreSimulator/Devices/{udid}/data/Containers/Data/Application");
    let Ok(entries) = std::fs::read_dir(&data_apps_dir) else {
        return;
    };

    for entry in entries.flatten() {
        let metadata_plist = entry.path().join(".com.apple.mobile_container_manager.metadata.plist");
        let Ok(content) = std::fs::read(&metadata_plist) else {
            continue;
        };
        if String::from_utf8_lossy(&content).contains("com.rust-ui") {
            // `Library/WebKit/<bundle-id>` (WebsiteData: LocalStorage, IndexedDB,
            // etc.) and `Library/Caches/<bundle-id>/WebKit` (NSURLCache-backed
            // HTTP/network disk cache: NetworkCache, CacheStorage, HSTS) are two
            // separate on-disk stores. Both must be purged or stale HTML/CSS/JS
            // keeps being served from the network cache even after WebsiteData
            // is cleared.
            let _ = std::fs::remove_dir_all(entry.path().join("Library/WebKit"));
            let _ = std::fs::remove_dir_all(entry.path().join("Library/Caches/com.rust-ui"));
        }
    }
}
