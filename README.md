# device-envoy-esp-blinky

[![GitHub](https://img.shields.io/badge/github-device--envoy--esp--blinky-8da0cb?style=flat&labelColor=555555&logo=github)](https://github.com/CarlKCarlK/device-envoy-esp-blinky)
[![crates.io](https://img.shields.io/crates/v/device-envoy-esp?style=flat&color=fc8d62&logo=rust)](https://crates.io/crates/device-envoy-esp)

Minimal blinky project using [`device-envoy-esp`](https://crates.io/crates/device-envoy-esp) and a built-in NeoPixel-style (WS2812) LED (where available).

## What This Repo Contains

- `src/main.rs` is the default quick start. It assumes a built-in NeoPixel-style (WS2812) LED on `GPIO8`.
- `examples/<chip>/<board>/blinky.rs` contains chip- and board-specific variants.

What if your board is different from the default setup?

1. If your built-in NeoPixel LED is on
a different GPIO, just edit `src/main.rs.`.
2. Alternatively, find your board at
`examples/<chip>/<board>/blinky.rs`. Copy  this `blinky.rs` into `src/main.rs`. If needed, follow
instructions on wiring an external plain LED.

## Quick Start

```bash
git clone https://github.com/CarlKCarlK/device-envoy-esp-blinky.git
cd device-envoy-esp-blinky
```

* Xtensa-based targets (`esp32s3`, `esp32s2`, `esp32`) need additional toolchain installation. See below.

```bash
# RISC-V Chips
cargo run --release --no-default-features --features <YOUR_CHIP>
# Xtensa Chips
cargo +esp run --release --no-default-features --features <YOUR_CHIP>
```

where `<YOUR_CHIP>` is `esp32c6`, etc.

## Commands

Default app (C6 devkit):

```bash
cargo run --release --no-default-features --features esp32c6
cargo check --no-default-features --features esp32c6
cargo build --release --no-default-features --features esp32c6
```

Board-specific example:

```bash
cargo run --release --no-default-features --features esp32c6 --example blinky_c6_devkitc1_n8
cargo check --no-default-features --features esp32s3 --example blinky_s3_generic
cargo build --release --no-default-features --features esp32c3 --example blinky_c3_luatos
```

Other chip/board combinations:

```bash
cargo run --release --no-default-features --features esp32c2 --example blinky_c2_generic
cargo run --release --no-default-features --features esp32h2 --example blinky_h2_generic
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
source "$HOME/export-esp.sh" # skip if on Windows
```

Then use `cargo +esp` for xtensa targets:

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
