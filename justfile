# Cross-platform justfile for RUST-UI
# Install just: cargo install just

# Default recipe - show available commands
default:
    @just --list

# ============================================================================
# DESKTOP
# ============================================================================

# Run Tauri desktop dev with custom port (reload_port = port + 1)
[macos]
run_desktop port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    sed -i '' "s|http://localhost:[0-9]*|http://localhost:{{port}}|g" src-tauri/tauri.conf.json
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="127.0.0.1:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri dev

[linux]
run_desktop port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    sed -i "s|http://localhost:[0-9]*|http://localhost:{{port}}|g" src-tauri/tauri.conf.json
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="127.0.0.1:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri dev

[windows]
run_desktop port="3000":
    #!powershell
    $reload_port = [int]{{port}} + 1
    (Get-Content src-tauri/tauri.conf.json) -replace 'http://localhost:[0-9]+', "http://localhost:{{port}}" | Set-Content src-tauri/tauri.conf.json
    Write-Host "Running on port {{port}} (reload: $reload_port)"
    $env:LEPTOS_SITE_ADDR = "127.0.0.1:{{port}}"
    $env:LEPTOS_RELOAD_PORT = "$reload_port"
    cargo tauri dev

# ============================================================================
# MOBILE - iOS (macOS only)
# ============================================================================

# Run iOS simulator (macOS only)
[macos]
run_ios port="3000" device="iPhone 16 Pro":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(ipconfig getifaddr en0)
    # Update IP and ports in iOS dev config
    sed -i '' "s|http://[0-9]*\.[0-9]*\.[0-9]*\.[0-9]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.ios.dev.conf.json
    sed -i '' "s|LEPTOS_SITE_ADDR=0.0.0.0:[0-9]*|LEPTOS_SITE_ADDR=0.0.0.0:{{port}}|g" src-tauri/tauri.ios.dev.conf.json
    sed -i '' "s|LEPTOS_RELOAD_PORT=[0-9]*|LEPTOS_RELOAD_PORT=$reload_port|g" src-tauri/tauri.ios.dev.conf.json
    sed -i '' "s|\"devUrl\": \"http://localhost:[0-9]*\"|\"devUrl\": \"http://localhost:{{port}}\"|g" src-tauri/tauri.ios.dev.conf.json
    sed -i '' "s|\"devUrl\": \"http://localhost:[0-9]*\"|\"devUrl\": \"http://localhost:{{port}}\"|g" src-tauri/tauri.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"

    APP_NAME=$(grep '^name' src-tauri/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
    cp __HideKeyboardAccessory.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cp __DisableContentInsetAdjustment.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cd src-tauri/gen/apple && xcodegen generate && cd ../../..

    xcrun simctl boot "{{device}}" || true
    open -a Simulator
    echo "Running on port {{port}} (reload: $reload_port)"
    cargo tauri ios dev "{{device}}"

# Run iPad simulator (macOS only)
[macos]
run_ipad port="3000" device="iPad Pro (12.9-inch) (3rd generation)":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(ipconfig getifaddr en0)
    # Update IP and ports in iOS dev config
    sed -i '' "s|http://[0-9]*\.[0-9]*\.[0-9]*\.[0-9]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.ios.dev.conf.json
    sed -i '' "s|LEPTOS_SITE_ADDR=0.0.0.0:[0-9]*|LEPTOS_SITE_ADDR=0.0.0.0:{{port}}|g" src-tauri/tauri.ios.dev.conf.json
    sed -i '' "s|LEPTOS_RELOAD_PORT=[0-9]*|LEPTOS_RELOAD_PORT=$reload_port|g" src-tauri/tauri.ios.dev.conf.json
    sed -i '' "s|\"devUrl\": \"http://localhost:[0-9]*\"|\"devUrl\": \"http://localhost:{{port}}\"|g" src-tauri/tauri.ios.dev.conf.json
    sed -i '' "s|\"devUrl\": \"http://localhost:[0-9]*\"|\"devUrl\": \"http://localhost:{{port}}\"|g" src-tauri/tauri.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"

    APP_NAME=$(grep '^name' src-tauri/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
    cp __HideKeyboardAccessory.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cp __DisableContentInsetAdjustment.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cd src-tauri/gen/apple && xcodegen generate && cd ../../..

    xcrun simctl boot "{{device}}" || true
    open -a Simulator
    echo "Running on port {{port}} (reload: $reload_port)"
    cargo tauri ios dev "{{device}}"

[linux]
[windows]
run_ios port="" device="":
    @echo "Error: iOS development requires macOS"
    @exit 1

[linux]
[windows]
run_ipad port="" device="":
    @echo "Error: iOS development requires macOS"
    @exit 1

# ============================================================================
# E2E TESTING
# ============================================================================

# Run e2e tests (requires server running on specified port)
e2e port="3000":
    cd e2e && PORT={{port}} pnpm test

# Run e2e tests with visible browser
e2e_headed port="3000":
    cd e2e && PORT={{port}} pnpm test:headed

# Run e2e tests with Playwright UI
e2e_ui port="3000":
    cd e2e && PORT={{port}} pnpm test:ui

# Run server and e2e tests together (cargo leptos end-to-end)
[macos]
e2e_full port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    LEPTOS_SITE_ADDR="127.0.0.1:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo leptos end-to-end

[linux]
e2e_full port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    LEPTOS_SITE_ADDR="127.0.0.1:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo leptos end-to-end

[windows]
e2e_full port="3000":
    #!powershell
    $reload_port = [int]{{port}} + 1
    $env:LEPTOS_SITE_ADDR = "127.0.0.1:{{port}}"
    $env:LEPTOS_RELOAD_PORT = "$reload_port"
    cargo leptos end-to-end

# ============================================================================
# MOBILE - Android (all platforms)
# ============================================================================

# Run Android emulator/device
[macos]
run_android port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(ipconfig getifaddr en0)
    sed -i '' "s|http://[0-9.]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.android.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="0.0.0.0:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri android dev

[linux]
run_android port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(hostname -I | awk '{print $1}')
    sed -i "s|http://[0-9.]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.android.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="0.0.0.0:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri android dev

[windows]
run_android port="3000":
    #!powershell
    $reload_port = [int]{{port}} + 1
    $SERVER_IP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.InterfaceAlias -notmatch 'Loopback' -and $_.PrefixOrigin -eq 'Dhcp' } | Select-Object -First 1).IPAddress
    (Get-Content src-tauri/tauri.android.conf.json) -replace 'http://[0-9.]+:[0-9]+', "http://${SERVER_IP}:{{port}}" | Set-Content src-tauri/tauri.android.conf.json
    Write-Host "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    Write-Host "Running on port {{port}} (reload: $reload_port)"
    $env:LEPTOS_SITE_ADDR = "0.0.0.0:{{port}}"
    $env:LEPTOS_RELOAD_PORT = "$reload_port"
    cargo tauri android dev
