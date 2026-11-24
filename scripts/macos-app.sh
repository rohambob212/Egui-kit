#!/bin/bash
# Build the gallery as a macOS .app bundle on this Mac.
# usage: scripts/macos-app.sh   -> ./dist/egui-kit Gallery.app
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release -p gallery
APP="dist/egui-kit Gallery.app"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp target/release/gallery "$APP/Contents/MacOS/gallery"
cat > "$APP/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>egui-kit Gallery</string>
  <key>CFBundleIdentifier</key><string>ir.rohambob.egui-kit.gallery</string>
  <key>CFBundleExecutable</key><string>gallery</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>0.1.0</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST
codesign --force --deep --sign - "$APP" 2>/dev/null || true
echo "built: $APP"
echo "run:   open \"$APP\""
echo "note:  first launch may need right-click > Open (unsigned build)"
