# device-envoy-esp-blinky

A minimal blinky app built with `device-envoy-esp` using the board's built-in smart LED (WS2812).

No external LED wiring is required.

## Prerequisites

- Rust installed
- `espflash` installed:

```bash
cargo install espflash
```

### ESP toolchain setup

This project is pinned to the `esp` toolchain (`rust-toolchain.toml`).

```bash
cargo install espup
espup install
source "$HOME/export-esp.sh"
```

## Built-in LED pin defaults

Typical built-in smart LED (WS2812) pins are:

- ESP32-C6 dev boards: usually `GPIO8`
- ESP32-S3 dev boards: often `GPIO48` (some boards use other pins)

This project defaults to `GPIO8` (C6) and `GPIO48` (S3).

If your board uses a different built-in LED pin, just update `src/main.rs`:

- `LED_PIN_NUM`
- The S3 or C6 `BuiltinBlinky::new(...)` pin argument
- The matching `led_strip! { ... pin: ... }` definition

It is a small, straightforward change.

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

- C6 and S3 aliases use `-Z build-std=core,alloc`.
- Runner is `espflash flash --monitor`, so `cargo blinky*` flashes and opens serial monitor.
- Logging uses `log::info!` through `esp-println`.
