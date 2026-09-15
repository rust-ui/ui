#!/bin/bash

cd "$(dirname "$0")"

DEVICE_ID="E429BCBA-D1F4-4FFB-A51E-7CA05ECABD19" # iPad Pro (12.9-inch) (3rd generation)

echo "📱 Shutting down all simulators..."
xcrun simctl shutdown all 2>/dev/null || true

echo "🚀 Booting iPad Pro..."
xcrun simctl boot "$DEVICE_ID" || echo "ℹ️  Already booted"
open -a Simulator

echo "🗑️  Uninstalling com.rust-ui from booted simulator..."
xcrun simctl uninstall booted com.rust-ui || echo "⚠️  Uninstall failed (app not installed?) — continuing"

echo "🔪 Killing any running dx serve..."
pkill -f "dx serve" 2>/dev/null || echo "ℹ️  No dx serve process was running"

echo "🚀 Starting dx serve --platform ios..."
exec dx serve --platform ios
