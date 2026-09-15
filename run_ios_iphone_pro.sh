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
exec dx serve --platform ios
