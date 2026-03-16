# device-envoy-esp-blinky

A minimal blinky app built with `device-envoy-esp` using the board's built-in smart LED (WS2812).

No external LED wiring is required.

## Prerequisites

- Rust installed
- `espflash` installed:

```bash
cargo install espflash
```

- Targets installed:

```bash
rustup target add riscv32imac-unknown-none-elf
rustup target add xtensa-esp32s3-none-elf
```

### ESP32-S3 toolchain note

This project is pinned to the `esp` toolchain (`rust-toolchain.toml`).

If S3 commands fail with target/toolchain errors:

```bash
cargo install espup
espup install
source "$HOME/export-esp.sh"
cargo blinky-s3-check
```

## Build and run

Default (`cargo blinky`) targets ESP32-C6.

### ESP32-C6 (default)

```bash
cargo blinky
```

### ESP32-S3

```bash
cargo blinky-s3
```

Check/build aliases:

- C6 default: `cargo blinky-check`, `cargo blinky-build`
- S3: `cargo blinky-s3-check`, `cargo blinky-s3-build`

## Notes

- Runner is `espflash flash --monitor`, so `cargo blinky*` flashes and opens serial monitor.
- Logging uses `log::info!` through `esp-println`.
