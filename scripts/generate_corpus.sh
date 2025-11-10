#!/usr/bin/env bash
set -euo pipefail

TARGET=${1:-./data/corpus.txt}
mkdir -p "$(dirname "$TARGET")"

for i in $(seq 1 100); do
  echo "fn demo_$i() {}" >> "$TARGET"
done

echo "[corpus] generated $TARGET"
