#[cfg(all(not(dev), not(any(target_os = "android", target_os = "ios"))))]
use std::process::Child;
#[cfg(all(not(dev), not(any(target_os = "android", target_os = "ios"))))]
use std::sync::Mutex;

#[cfg(all(not(dev), not(any(target_os = "android", target_os = "ios"))))]
struct ServerProcess(Mutex<Option<Child>>);

/// Setup for production builds - starts the bundled Leptos server if present
/// In dev mode, mobile, or online mode (no bundled server), this is a no-op
pub fn setup(_app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(not(dev), not(any(target_os = "android", target_os = "ios"))))]
    {
        use std::process::Command;
        use tauri::Manager;

        let resource_dir = _app.path().resource_dir()?;
        let server_path = resource_dir
            .join("_up_")
            .join("target")
            .join("aarch64-apple-darwin")
            .join("release")
            .join("server");
        let site = resource_dir.join("_up_").join("target").join("site");

        // Online mode: skip server startup if binary not bundled
        if !server_path.exists() {
            return Ok(());
        }

        let child = Command::new(&server_path)
            .env("LEPTOS_SITE_ROOT", &site)
            .env("LEPTOS_SITE_ADDR", "127.0.0.1:14100")
            .spawn()?;

        _app.manage(ServerProcess(Mutex::new(Some(child))));

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}

/// Kill the server process on app exit
pub fn cleanup(_app: &tauri::AppHandle) {
    #[cfg(all(not(dev), not(any(target_os = "android", target_os = "ios"))))]
    {
        use tauri::Manager;

        if let Some(state) = _app.try_state::<ServerProcess>() {
            if let Ok(mut guard) = state.0.lock() {
                if let Some(mut child) = guard.take() {
                    let _ = child.kill();
                }
            }
        }
    }
}
