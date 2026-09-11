fn main() {
    let now = time::OffsetDateTime::now_utc();
    let build_date = format!("{}-{:02}-{:02}", now.year(), now.month() as u8, now.day());
    println!("cargo:rustc-env=BUILD_DATE={build_date}");
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("ios") {
        purge_stale_simulator_webkit_cache();
    }
}

/// `dx serve --platform ios` reinstalls the app bundle on every relaunch but
/// never touches the simulator's separate WebKit data container, so a plain
/// kill+relaunch keeps serving stale cached HTML/CSS/JS (see
/// PLAN_FIX_BOTTOM_NAV.md section 13). Purge it here so every ios build gets
/// a fresh WKWebView cache, same effect as `xcrun simctl uninstall` without
/// needing to reinstall the app. Best-effort: no-op (and no build failure) if
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
    let data_apps_dir = format!(
        "{home}/Library/Developer/CoreSimulator/Devices/{udid}/data/Containers/Data/Application"
    );
    let Ok(entries) = std::fs::read_dir(&data_apps_dir) else {
        return;
    };

    for entry in entries.flatten() {
        let metadata_plist = entry
            .path()
            .join(".com.apple.mobile_container_manager.metadata.plist");
        let Ok(content) = std::fs::read(&metadata_plist) else {
            continue;
        };
        if String::from_utf8_lossy(&content).contains("com.rust-ui") {
            let _ = std::fs::remove_dir_all(entry.path().join("Library/WebKit"));
        }
    }
}
