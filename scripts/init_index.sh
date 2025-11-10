#!/usr/bin/env bash
set -euo pipefail

WORKDIR=${1:-./data/index}
mkdir -p "$WORKDIR"

echo "[init_index] generating schema snapshot"
cat >"$WORKDIR/schema.json" <<JSON
{"fields":["path","contents","language"]}
JSON

echo "[init_index] done"
