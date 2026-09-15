#!/bin/bash

cd "$(dirname "$0")"

DEVICE_ID="4265C417-5575-477C-9DD4-556626825D51" # iPhone 16 Pro (18.6)

echo "📱 Shutting down all simulators..."
xcrun simctl shutdown all 2>/dev/null || true

echo "🚀 Booting iPhone 16 Pro..."
xcrun simctl boot "$DEVICE_ID" || echo "ℹ️  Already booted"
open -a Simulator

echo "🗑️  Uninstalling com.rust-ui from booted simulator..."
xcrun simctl uninstall booted com.rust-ui || echo "⚠️  Uninstall failed (app not installed?) — continuing"

echo "🔪 Killing any running dx serve..."
pkill -f "dx serve" 2>/dev/null || echo "ℹ️  No dx serve process was running"

echo "🚀 Starting dx serve --platform ios..."
# WORKAROUND: dx doesn't bake the AppIcon into the .app bundle (see
# ios/inject_app_icon.sh header for the full story / dioxus#3685). We run
# dx serve in the background and re-patch + reinstall the icon every time
# it rebuilds the app bundle, so hot-reload doesn't revert to the
# placeholder icon.
APP_BUNDLE="target/dx/dioxus-ui/debug/ios/DioxusUi.app"
dx serve --platform ios &
DX_PID=$!
trap 'kill "$DX_PID" 2>/dev/null' EXIT

LAST_MTIME=""
while kill -0 "$DX_PID" 2>/dev/null; do
  if [ -f "$APP_BUNDLE/Info.plist" ]; then
    MTIME=$(stat -f %m "$APP_BUNDLE/Info.plist" 2>/dev/null)
    if [ -n "$MTIME" ] && [ "$MTIME" != "$LAST_MTIME" ]; then
      sleep 1 # let dx finish writing the bundle before we touch it
      ./ios/inject_app_icon.sh "$APP_BUNDLE" && xcrun simctl install booted "$APP_BUNDLE" 2>/dev/null
      # re-stat after injecting: PlistBuddy just rewrote Info.plist itself,
      # so its mtime moved again. If we saved the pre-inject MTIME here,
      # next poll would see our own write as "new" and re-inject forever.
      LAST_MTIME=$(stat -f %m "$APP_BUNDLE/Info.plist" 2>/dev/null)
    fi
  fi
  sleep 2
done
wait "$DX_PID"
