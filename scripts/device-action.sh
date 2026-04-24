#!/usr/bin/env bash
set -euo pipefail

action="${1:-}"
arg1="${2:-}"
arg2="${3:-}"

if [[ -z "${action}" ]]; then
  echo "usage: scripts/device-action.sh <run|check|build> [chip] [board]" >&2
  exit 1
fi

case "${action}" in
  run|check|build) ;;
  *)
    echo "invalid action '${action}' (expected: run, check, build)" >&2
    exit 1
    ;;
esac

chip="${arg1:-c6}"
board="${arg2:-}"

if [[ "${chip}" == "blinky" ]]; then
  chip="${board:-c6}"
  board=""
fi

cargo_bin=(cargo)
build_std_args=()
target=""
feature=""

case "${chip}" in
  c2)
    target="riscv32imac-unknown-none-elf"
    feature="esp32c2"
    ;;
  c3)
    target="riscv32imc-unknown-none-elf"
    feature="esp32c3"
    ;;
  c6)
    target="riscv32imac-unknown-none-elf"
    feature="esp32c6"
    ;;
  h2)
    target="riscv32imac-unknown-none-elf"
    feature="esp32h2"
    ;;
  esp32)
    target="xtensa-esp32-none-elf"
    feature="esp32"
    cargo_bin=(cargo +esp)
    build_std_args=(-Zbuild-std=core,alloc)
    ;;
  s2)
    target="xtensa-esp32s2-none-elf"
    feature="esp32s2"
    cargo_bin=(cargo +esp)
    build_std_args=(-Zbuild-std=core,alloc)
    ;;
  s3)
    target="xtensa-esp32s3-none-elf"
    feature="esp32s3"
    cargo_bin=(cargo +esp)
    build_std_args=(-Zbuild-std=core,alloc)
    ;;
  *)
    echo "invalid chip '${chip}' (expected one of: c2, c3, c6, h2, esp32, s2, s3)" >&2
    exit 1
    ;;
esac

release_args=()
if [[ "${action}" != "check" ]]; then
  release_args=(--release)
fi

if [[ "${#build_std_args[@]}" -gt 0 ]]; then
  source "$HOME/export-esp.sh"
fi

example_args=()
if [[ -n "${board}" ]]; then
  example_args=(--example "blinky_${chip}_${board}")
fi

"${cargo_bin[@]}" "${action}" \
  --target "${target}" \
  "${release_args[@]}" \
  --no-default-features \
  --features "${feature}" \
  "${example_args[@]}" \
  "${build_std_args[@]}"
