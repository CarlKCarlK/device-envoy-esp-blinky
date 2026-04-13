#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
project_root="$(cd "${script_dir}/.." && pwd)"
source_examples_dir="${SOURCE_EXAMPLES_DIR:-${project_root}/../device-envoy/crates/device-envoy-esp/examples}"
dest_examples_dir="${project_root}/examples"
cargo_toml="${project_root}/Cargo.toml"

if [[ ! -d "${source_examples_dir}" ]]; then
  echo "error: source examples directory not found: ${source_examples_dir}" >&2
  echo "set SOURCE_EXAMPLES_DIR to override" >&2
  exit 1
fi

chip_feature() {
  case "$1" in
    c2) echo "esp32c2" ;;
    c3) echo "esp32c3" ;;
    c6) echo "esp32c6" ;;
    esp32) echo "esp32" ;;
    h2) echo "esp32h2" ;;
    s2) echo "esp32s2" ;;
    s3) echo "esp32s3" ;;
    *)
      echo "error: unknown chip '$1'" >&2
      exit 1
      ;;
  esac
}

chip_target() {
  case "$1" in
    c2|c6|h2) echo "riscv32imac-unknown-none-elf" ;;
    c3) echo "riscv32imc-unknown-none-elf" ;;
    esp32) echo "xtensa-esp32-none-elf" ;;
    s2) echo "xtensa-esp32s2-none-elf" ;;
    s3) echo "xtensa-esp32s3-none-elf" ;;
    *)
      echo "error: unknown chip '$1'" >&2
      exit 1
      ;;
  esac
}

rm -rf "${dest_examples_dir}"
mkdir -p "${dest_examples_dir}"

examples_file="$(mktemp)"
trap 'rm -f "${examples_file}"' EXIT

find "${source_examples_dir}" -mindepth 3 -maxdepth 3 -type f -name 'blinky.rs' -print0 \
  | sort -z \
  | while IFS= read -r -d '' source_file; do
      relative_path="${source_file#${source_examples_dir}/}"
      chip="${relative_path%%/*}"
      board_and_file="${relative_path#*/}"
      board="${board_and_file%%/*}"

      destination_dir="${dest_examples_dir}/${chip}/${board}"
      mkdir -p "${destination_dir}"
      cp "${source_file}" "${destination_dir}/blinky.rs"

      feature="$(chip_feature "${chip}")"
      target="$(chip_target "${chip}")"
      example_name="blinky_${chip}_${board}"

      cat >> "${examples_file}" <<EXAMPLE
[[example]]
name = "${example_name}"
path = "examples/${chip}/${board}/blinky.rs"
required-features = ["${feature}"]

EXAMPLE

      printf '%s\n' "- ${chip}/${board}: \
  \`just run ${chip} ${board}\`\
  (target \`${target}\`, feature \`${feature}\`)"
    done > "${project_root}/projects/board-matrix.md"

begin_marker="# BEGIN GENERATED BLINKY EXAMPLES"
end_marker="# END GENERATED BLINKY EXAMPLES"

block_file="$(mktemp)"
trap 'rm -f "${examples_file}" "${block_file}"' EXIT

{
  echo "${begin_marker}"
  cat "${examples_file}"
  echo "${end_marker}"
} > "${block_file}"

if rg -q "${begin_marker}" "${cargo_toml}"; then
  awk -v begin="${begin_marker}" -v end="${end_marker}" -v block_file="${block_file}" '
    BEGIN {
      while ((getline line < block_file) > 0) {
        block = block line "\n"
      }
      close(block_file)
    }
    $0 == begin {
      printf "%s", block
      skip = 1
      next
    }
    $0 == end {
      skip = 0
      next
    }
    skip == 0 { print }
  ' "${cargo_toml}" > "${cargo_toml}.tmp"
  mv "${cargo_toml}.tmp" "${cargo_toml}"
else
  {
    cat "${cargo_toml}"
    echo
    cat "${block_file}"
  } > "${cargo_toml}.tmp"
  mv "${cargo_toml}.tmp" "${cargo_toml}"
fi

echo "Synced examples from: ${source_examples_dir}"
echo "Updated: ${dest_examples_dir}"
echo "Updated: ${cargo_toml}"
echo "Updated: ${project_root}/projects/board-matrix.md"
