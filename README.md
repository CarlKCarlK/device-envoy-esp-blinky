# device-envoy-esp-blinky

[![GitHub](https://img.shields.io/badge/github-device--envoy--esp--blinky-8da0cb?style=flat&labelColor=555555&logo=github)](https://github.com/CarlKCarlK/device-envoy-esp-blinky)
[![crates.io](https://img.shields.io/crates/v/device-envoy-esp?style=flat&color=fc8d62&logo=rust)](https://crates.io/crates/device-envoy-esp)

Minimal blinky project using [`device-envoy-esp`](https://crates.io/crates/device-envoy-esp) and a built-in NeoPixel-style (WS2812) LED.

## Quick Start

```bash
git clone https://github.com/CarlKCarlK/device-envoy-esp-blinky.git
cd device-envoy-esp-blinky
just run
```

Also accepted:

```bash
just run blinky
just check blinky
just build blinky
```

## RISC-V Chips

Suffixes: `c3`, `c6`, `h2`

Commands:

- `just run [blinky] [chip]`
- `just check [blinky] [chip]`
- `just build [blinky] [chip]`

Examples:

```bash
just run c3
just run blinky c6
just check h2
just build c3
```

## Xtensa Chips

Suffixes: `esp32`, `s2`, `s3`

Commands are the same:

- `just run [blinky] [chip]`
- `just check [blinky] [chip]`
- `just build [blinky] [chip]`

Examples:

```bash
just run esp32
just run blinky s3
just check s2
just build esp32
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

Install flashing tool:

```bash
cargo install espflash
```

Pin defaults in this workspace copy:

- `c6`: `GPIO8`
- `s3`: `GPIO48`
- `c3`, `h2`, `esp32`, `s2`: `GPIO2`

If your board uses a different built-in LED pin, update `src/main.rs`.

Note: this workspace copy is pinned to your local `device-envoy` checkout for chip-feature development. Before publishing template changes, switch `Cargo.toml` back to crates.io after the next release.

## License

Licensed under either:

- MIT license (see LICENSE-MIT)
- Apache License, Version 2.0 (see LICENSE-APACHE)

at your option.
