# device-envoy-esp-blinky

[![GitHub](https://img.shields.io/badge/github-device--envoy--esp--blinky-8da0cb?style=flat&labelColor=555555&logo=github)](https://github.com/CarlKCarlK/device-envoy-esp-blinky)
[![crates.io](https://img.shields.io/crates/v/device-envoy-esp?style=flat&color=fc8d62&logo=rust)](https://crates.io/crates/device-envoy-esp)

[`device-envoy-esp`](https://crates.io/crates/device-envoy-esp) is a Rust crate for building ESP32 applications with NeoPixel-style (WS2812) LED panels, easy WiFi, and composable device abstractions.

This repository is a minimal blinky example that uses [`device-envoy-esp`](https://crates.io/crates/device-envoy-esp) with the board's built-in smart LED (WS2812).

This template currently runs on and is tested with ESP32-C6 and ESP32-S3.

To use project as the start of your own project:

```bash
git clone https://github.com/CarlKCarlK/device-envoy-esp-blinky.git
cd device-envoy-esp-blinky
git remote remove origin
```

It loops a one-pixel white SOS Morse pattern on the built-in smart LED.

<img src="docs/assets/led2d_sos_1x1.png" alt="Looping 1-pixel SOS preview" width="50" height="50">

No external LED wiring is required.


Jump to the [ESP32-C6 directions](#esp32-c6-directions) or the [ESP32-S3 directions](#esp32-s3-directions).

## ESP32-C6 Directions

### C6 Prerequisites

ESP32-C6 uses the RISC-V target.

- Rust installed
- `just` installed
- `espflash` installed:

```bash
cargo install espflash
```

- C6 target installed:

```bash
rustup target add riscv32imac-unknown-none-elf
```

### C6 built-in LED pin

This project defaults to `GPIO8` for ESP32-C6 dev boards.

If your board uses a different built-in smart LED pin, update `src/main.rs`.

### C6 Build and run

```bash
just run
just run blinky
```

Behind the scenes, `just run` executes:

```bash
cargo run --release --target riscv32imac-unknown-none-elf --no-default-features
```

The `runner = "espflash flash --monitor"` setting in `.cargo/config.toml` handles flashing and opening the serial monitor, so `just run` does both.

### Extra C6 commands

- Check: `just check`
- Build only: `just build`
- Also accepted: `just check blinky`, `just build blinky`

## ESP32-S3 Directions

### S3 Prerequisites

ESP32-S3 uses Xtensa and requires ESP toolchain setup.

```bash
cargo install espup
espup install
source "$HOME/export-esp.sh" # skip if on Windows
```

On Linux-like systems, you'll either need to run `source "$HOME/export-esp.sh"` every time or add `. "$HOME/export-esp.sh"` to your shell profile such as `~/.bashrc`, `~/.zshrc`, or `~/.profile`.

For example, to add it to `~/.bashrc`:

```bash
echo '. "$HOME/export-esp.sh"' >> ~/.bashrc
source ~/.bashrc
```

On Windows, `espup` injects the required environment variables automatically, so there is no `source` step, but you do need
to re-start your command shell or terminal.

### S3 built-in LED pin

This project defaults to `GPIO48` for ESP32-S3 dev boards.

If your board uses a different built-in smart LED pin, update `src/main.rs`. `GPIO38` is the next most common.

### S3 Build and run

```bash
just run s3
just run blinky s3
```

Behind the scenes, `just run s3` expands to:

```bash
cargo +esp run -Z build-std=core,alloc --release --target xtensa-esp32s3-none-elf --no-default-features
```

The `runner = "espflash flash --monitor"` setting in `.cargo/config.toml` handles flashing and opening the serial monitor, so `just run s3` does both.

### Extra S3 commands

- Check: `just check s3`
- Build only: `just build s3`
- Also accepted: `just check blinky s3`, `just build blinky s3`

## License

Licensed under either:

- MIT license (see LICENSE-MIT)
- Apache License, Version 2.0 (see LICENSE-APACHE)

at your option.
