#!/bin/bash
# Publish a built wasm bundle (gallery.js, gallery_bg.wasm, index.html) to the host.
# The bundle comes from the `web` workflow's artifact:
#   gh run download --name egui-kit-web   (or the Actions UI)
# usage: scripts/deploy-web.sh <dist-dir>
set -euo pipefail
SRC="${1:?usage: scripts/deploy-web.sh <dist-dir containing index.html>}"
DEST=/srv/egui-kit-web

[ -f "$SRC/index.html" ] || { echo "no index.html in $SRC"; exit 1; }
[ -f "$SRC/gallery_bg.wasm" ] || { echo "no gallery_bg.wasm in $SRC"; exit 1; }

mkdir -p "$DEST"
rsync -a --delete "$SRC"/ "$DEST"/
systemctl is-active --quiet egui-kit-web.service || systemctl restart egui-kit-web.service
sleep 1
echo "published $(du -sh "$DEST" | cut -f1) to https://egui-kit.rohambob.ir"
curl -s -o /dev/null -w 'live check: %{http_code}\n' --max-time 15 https://egui-kit.rohambob.ir/