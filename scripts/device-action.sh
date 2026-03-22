#!/usr/bin/env bash
set -euo pipefail

action="${1:-}"
arg1="${2:-}"
arg2="${3:-}"

chip="c6"
if [[ "$arg1" == "blinky" ]]; then
  chip="${arg2:-c6}"
elif [[ -n "$arg1" ]]; then
  chip="$arg1"
fi

if [[ -z "$action" ]]; then
  echo "usage: scripts/device-action.sh <run|check|build> [blinky] [chip]" >&2
  exit 1
fi

case "$action" in
  run|check|build) ;;
  *)
    echo "invalid action '$action' (expected: run, check, build)" >&2
    exit 1
    ;;
esac

cargo_bin=(cargo)
build_std_args=()
target=""

case "$chip" in
  c6)
    target="riscv32imac-unknown-none-elf"
    ;;
  s3)
    target="xtensa-esp32s3-none-elf"
    cargo_bin=(cargo +esp)
    build_std_args=(-Zbuild-std=core,alloc)
    ;;
  *)
    echo "invalid chip '$chip' (expected one of: c6, s3)" >&2
    exit 1
    ;;
esac

release_args=()
if [[ "$action" != "check" ]]; then
  release_args=(--release)
fi

if [[ "${#build_std_args[@]}" -gt 0 ]]; then
  source "$HOME/export-esp.sh"
fi

"${cargo_bin[@]}" "$action" \
  --target "$target" \
  "${release_args[@]}" \
  --no-default-features \
  "${build_std_args[@]}"
