#!/bin/bash

cd "$(dirname "$0")"

echo "🗑️  Uninstalling com.rust-ui from booted simulator..."
xcrun simctl uninstall booted com.rust-ui || echo "⚠️  Uninstall failed (app not installed? no booted sim?) — continuing"

echo "🔪 Killing any running dx serve..."
pkill -f "dx serve" 2>/dev/null || echo "ℹ️  No dx serve process was running"

echo "🚀 Starting dx serve --platform ios..."
exec dx serve --platform ios
