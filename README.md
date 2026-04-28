# device-envoy-esp-blinky

[![GitHub](https://img.shields.io/badge/github-device--envoy--esp--blinky-8da0cb?style=flat&labelColor=555555&logo=github)](https://github.com/CarlKCarlK/device-envoy-esp-blinky)
[![crates.io](https://img.shields.io/crates/v/device-envoy-esp?style=flat&color=fc8d62&logo=rust)](https://crates.io/crates/device-envoy-esp)

Minimal blinky project using [`device-envoy-esp`](https://crates.io/crates/device-envoy-esp) and a built-in NeoPixel-style (WS2812) LED (where available).

## What This Repo Contains

- `src/main.rs` is the default quick start. It assumes a built-in NeoPixel-style (WS2812) LED on `GPIO8`.
- `examples/<chip>/<board>/blinky.rs` contains chip- and board-specific variants.

What if your board is different from the default setup?

1. If your built-in NeoPixel LED is on a different GPIO, edit `src/main.rs` to reflect its location.
2. Alternatively, find your board at `examples/<chip>/<board>/blinky.rs`. Copy that `blinky.rs` into `src/main.rs`, and adjust wiring as needed (including external plain LED wiring if required).

**Feedback**: If you try this crate, I’d love to hear how it goes, whether it works well, fails to build, needs clearer docs, or does not fit your hardware. Please send feedback to carlk AT msn.com.
## Prerequisites

Before **Quick Start**, install the toolchain in [Toolchain](#toolchain), then return here.

## Quick Start

```bash
git clone https://github.com/CarlKCarlK/device-envoy-esp-blinky.git
cd device-envoy-esp-blinky
git remote remove origin
```

The default code in `src/main.rs` assumes a built-in NeoPixel-style LED on `GPIO8`, which matches common ESP32-C3 and ESP32-C6 Espressif dev boards. For an ESP32-S3, change it to `GPIO38` or `GPIO48`, depending on the board revision. For other chips, or if that doesn't work, see [Running Examples Directly](#running-examples-directly).

Next, run:

```bash
cargo xtask run --chip YOUR_CHIP
```

Change the `YOUR_CHIP` to your chip. Supported options: `c2`, `c3`, `c5`, `c6`, `c61`, `h2`, `esp32`, `s2`, `s3`.

*This `xtask run` always adds the `--release` option*.

## Setting the Default Chip

If you do not want to repeat `--chip`, set your project default once in `xtask/src/main.rs` by changing:

```rust
const DEFAULT_CHIP: Chip = Chip::C6;
```

## Main Commands

*This assumes you've set the default to your chip; otherwise append `--chip YOUR_CHIP`.*

```bash
cargo xtask run
cargo xtask check
cargo xtask build
```

## Running Examples Directly

```bash
cargo xtask run --example blinky_c6_devkitc1_n8
cargo xtask check --example blinky_c6_devkitc1_n8
cargo xtask build --example blinky_c6_devkitc1_n8
```

Where examples are defined in `examples` and (if subfolders are used) in `Cargo.toml`.

## Toolchain

Install Rust from [rustup.rs](https://rustup.rs) if you haven't already.

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
# Linux/macOS/WSL
source "$HOME/export-esp.sh"
# To make this permanent, add this line to your
# shell profile (e.g. ~/.bashrc or ~/.zshrc):
# source "$HOME/export-esp.sh"
```

```powershell
# Windows PowerShell
& "$env:USERPROFILE\export-esp.ps1"
# To make this permanent, run:
# if (!(Test-Path $PROFILE)) { New-Item -Type File -Path $PROFILE -Force }; Add-Content $PROFILE '& "$env:USERPROFILE\export-esp.ps1"'
```

Install flashing tool:

```bash
cargo install espflash
```

## License

Licensed under either:

- MIT license (see LICENSE-MIT)
- Apache License, Version 2.0 (see LICENSE-APACHE)

at your option.
