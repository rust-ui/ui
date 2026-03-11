mod production;

#[cfg(mobile)]
#[tauri::mobile_entry_point]
fn mobile_main() {
    run();
}

/// Check if we can reach rust-ui.com
fn is_online() -> bool {
    use std::net::ToSocketAddrs;

    // First resolve DNS, then try to connect
    if let Ok(mut addrs) = "rust-ui.com:443".to_socket_addrs() {
        if let Some(addr) = addrs.next() {
            return std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_secs(3))
                .is_ok();
        }
    }
    false
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            production::setup(app)?;

            // Check connectivity and redirect to offline page if needed
            #[cfg(not(dev))]
            {
                use tauri::Manager;

                if !is_online() {
                    if let Some(webview) = app.get_webview_window("main") {
                        let offline_path = app
                            .path()
                            .resource_dir()
                            .ok()
                            .map(|p| p.join("offline.html"));

                        if let Some(path) = offline_path {
                            if path.exists() {
                                if let Ok(url) = tauri::Url::from_file_path(&path) {
                                    let _ = webview.navigate(url);
                                }
                            }
                        }
                    }
                }
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event {
                production::cleanup(app);
            }
        });
}
