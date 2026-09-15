#!/bin/bash
# ---------------------------------------------------------------------------
# WORKAROUND for dioxus-cli bug: `dx` does not bake the iOS AppIcon into the
# app bundle. The [bundle].icon key in Dioxus.toml is silently ignored on iOS
# (same class of bug reported for Android in dioxuslabs/dioxus#3685, still
# open as of dx 0.7.10) — every `.app`/`.ipa` we've inspected has no
# Assets.car / AppIcon.appiconset at all, so the simulator/device falls back
# to the generic grid placeholder icon.
#
# Fix pattern borrowed from leptos-ui/src-tauri (which pre-generates and
# commits every required icon size instead of relying on the build tool to
# generate them at build time — see leptos-ui/src-tauri/ios-icons/).
#
# This script (re)generates ios/AppIcon.appiconset/ from the single 1024
# source (ios/AppIcon-1024.png). Run it once, or whenever AppIcon-1024.png
# changes. The generated PNGs + Contents.json are committed to git.
#
# The actual injection into the .app bundle dx builds happens in
# ios/inject_app_icon.sh, called from the run_ios_*.sh scripts.
# ---------------------------------------------------------------------------

cd "$(dirname "$0")"

SRC="AppIcon-1024.png"
OUT="Assets.xcassets/AppIcon.appiconset"
mkdir -p "$OUT" "Assets.xcassets"

cat > "Assets.xcassets/Contents.json" <<'JSON'
{
  "info" : { "version" : 1, "author" : "xcode" }
}
JSON

resize() {
  sips -z "$2" "$2" "$SRC" --out "$OUT/$1" >/dev/null
  echo "  ✓ $1 (${2}x${2})"
}

echo "🎨 Generating iOS icon set from $SRC..."
resize "AppIcon-20x20@1x.png" 20
resize "AppIcon-20x20@2x.png" 40
resize "AppIcon-20x20@3x.png" 60
resize "AppIcon-29x29@1x.png" 29
resize "AppIcon-29x29@2x.png" 58
resize "AppIcon-29x29@3x.png" 87
resize "AppIcon-40x40@1x.png" 40
resize "AppIcon-40x40@2x.png" 80
resize "AppIcon-40x40@3x.png" 120
resize "AppIcon-60x60@2x.png" 120
resize "AppIcon-60x60@3x.png" 180
resize "AppIcon-76x76@1x.png" 76
resize "AppIcon-76x76@2x.png" 152
resize "AppIcon-83.5x83.5@2x.png" 167
resize "AppIcon-512x512@2x.png" 1024

cat > "$OUT/Contents.json" <<'JSON'
{
  "images" : [
    { "size" : "20x20", "idiom" : "iphone", "filename" : "AppIcon-20x20@2x.png", "scale" : "2x" },
    { "size" : "20x20", "idiom" : "iphone", "filename" : "AppIcon-20x20@3x.png", "scale" : "3x" },
    { "size" : "29x29", "idiom" : "iphone", "filename" : "AppIcon-29x29@2x.png", "scale" : "2x" },
    { "size" : "29x29", "idiom" : "iphone", "filename" : "AppIcon-29x29@3x.png", "scale" : "3x" },
    { "size" : "40x40", "idiom" : "iphone", "filename" : "AppIcon-40x40@2x.png", "scale" : "2x" },
    { "size" : "40x40", "idiom" : "iphone", "filename" : "AppIcon-40x40@3x.png", "scale" : "3x" },
    { "size" : "60x60", "idiom" : "iphone", "filename" : "AppIcon-60x60@2x.png", "scale" : "2x" },
    { "size" : "60x60", "idiom" : "iphone", "filename" : "AppIcon-60x60@3x.png", "scale" : "3x" },
    { "size" : "20x20", "idiom" : "ipad", "filename" : "AppIcon-20x20@1x.png", "scale" : "1x" },
    { "size" : "20x20", "idiom" : "ipad", "filename" : "AppIcon-20x20@2x.png", "scale" : "2x" },
    { "size" : "29x29", "idiom" : "ipad", "filename" : "AppIcon-29x29@1x.png", "scale" : "1x" },
    { "size" : "29x29", "idiom" : "ipad", "filename" : "AppIcon-29x29@2x.png", "scale" : "2x" },
    { "size" : "40x40", "idiom" : "ipad", "filename" : "AppIcon-40x40@1x.png", "scale" : "1x" },
    { "size" : "40x40", "idiom" : "ipad", "filename" : "AppIcon-40x40@2x.png", "scale" : "2x" },
    { "size" : "76x76", "idiom" : "ipad", "filename" : "AppIcon-76x76@1x.png", "scale" : "1x" },
    { "size" : "76x76", "idiom" : "ipad", "filename" : "AppIcon-76x76@2x.png", "scale" : "2x" },
    { "size" : "83.5x83.5", "idiom" : "ipad", "filename" : "AppIcon-83.5x83.5@2x.png", "scale" : "2x" },
    { "size" : "1024x1024", "idiom" : "ios-marketing", "filename" : "AppIcon-512x512@2x.png", "scale" : "1x" }
  ],
  "info" : { "version" : 1, "author" : "xcode" }
}
JSON

echo "✅ ios/Assets.xcassets/AppIcon.appiconset/ regenerated."
