#!/usr/bin/env bash
set -euo pipefail

PACK=$1
TAG=$2
LABEL=$3
TARGET=$4

OUT=${PACK}-${TAG}-${LABEL}
mkdir -p pkg/temp/bin

BIN_NAMES=$(cargo metadata --format-version 1 --no-deps \
  | jq -r '.packages[] | select(.source == null) | .targets[]  | select(.kind[]=="bin") | .name' | sort -u)

for BIN in $BIN_NAMES; do
  SRC="target/${TARGET}/release/${BIN}"
  cp ${SRC}* pkg/temp/bin/
done
cp LICENSE README.md pkg/temp/ 2>/dev/null || true

tar -C pkg/temp -czf pkg/${OUT}.tar.gz .
rm -rf pkg/temp 2>/dev/null || true
