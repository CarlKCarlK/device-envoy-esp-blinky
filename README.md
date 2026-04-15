# device-envoy-esp-blinky

[![GitHub](https://img.shields.io/badge/github-device--envoy--esp--blinky-8da0cb?style=flat&labelColor=555555&logo=github)](https://github.com/CarlKCarlK/device-envoy-esp-blinky)
[![crates.io](https://img.shields.io/crates/v/device-envoy-esp?style=flat&color=fc8d62&logo=rust)](https://crates.io/crates/device-envoy-esp)

Minimal blinky project using [`device-envoy-esp`](https://crates.io/crates/device-envoy-esp) and a built-in NeoPixel-style (WS2812) LED (where available).

## What This Repo Contains

- `src/main.rs` is the default quick start. It assumes a built-in NeoPixel-style (WS2812) LED on `GPIO8` (where available).
- `examples/<chip>/<board>/blinky.rs` contains chip- and board-specific variants.

What if your board is different from the default setup?

1. If your built-in NeoPixel LED is on a different GPIO, edit `src/main.rs`.
2. Alternatively, find your board at `examples/<chip>/<board>/blinky.rs`, copy that `blinky.rs` into `src/main.rs`, and adjust wiring as needed (including external plain LED wiring if required).

## Quick Start

```bash
git clone https://github.com/CarlKCarlK/device-envoy-esp-blinky.git
cd device-envoy-esp-blinky
cargo xtask run --release
```

For non-default chips:

```bash
cargo xtask run --chip c3 --release
cargo xtask check --chip s3
cargo xtask build --chip esp32 --release
```

`--chip` supports:
`c2`, `c3`, `c6`, `h2`, `esp32`, `s2`, `s3`.

If you want board-specific code, copy from `examples/<chip>/<board>/blinky.rs` into `src/main.rs`, then run `cargo xtask ...`.

## Commands

Default app (C6 devkit):

```bash
cargo xtask run --release
cargo xtask check
cargo xtask build --release
```

Chip-specific commands:

```bash
cargo xtask run --chip c6 --release
cargo xtask check --chip s3
cargo xtask build --chip c3 --release
```

Other chip/board combinations:

```bash
cargo xtask run --chip c2 --release
cargo xtask run --chip h2 --release
```

## Toolchain

Install Rust targets:

```bash
rustup target add riscv32imac-unknown-none-elf
rustup target add riscv32imc-unknown-none-elf
```

Xtensa chips (`esp32`, `s2`, `s3`) need ESP toolchain setup:

```bash
cargo install espup
espup install
```

Enable espup environment before building xtensa chips:

```bash
# Linux/macOS
source "$HOME/export-esp.sh"
```

```powershell
# Windows PowerShell
& "$HOME/export-esp.ps1"
```

Equivalent raw cargo commands (advanced):

```bash
cargo +esp run --release --target xtensa-esp32s3-none-elf --no-default-features --features esp32s3 --example blinky_s3_generic -Zbuild-std=core,alloc
```

Install flashing tool:

```bash
cargo install espflash
```

Optional helper (`just`)

```bash
just run
just check
just build
```

`just` is a convenience wrapper around the `cargo` commands above.

## License

Licensed under either:

- MIT license (see LICENSE-MIT)
- Apache License, Version 2.0 (see LICENSE-APACHE)

at your option.
