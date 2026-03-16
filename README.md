# device-envoy-esp-blinky

A minimal blinky app built with `device-envoy-esp` using the board's built-in smart LED (WS2812).

No external LED wiring is required.

## ESP32-C6 (default)

### Prerequisites

- Rust installed
- `espflash` installed:

```bash
cargo install espflash
```

- C6 target installed:

```bash
rustup target add riscv32imac-unknown-none-elf
```

### Build and run

```bash
cargo blinky
```

### Extra C6 commands

- Check: `cargo blinky-check`
- Build only: `cargo blinky-build`

## ESP32-S3 (optional)

### Prerequisites

S3 uses Xtensa and requires ESP toolchain setup.

```bash
cargo install espup
espup install
source "$HOME/export-esp.sh"
```

### Build and run

```bash
cargo blinky-s3
```

### Extra S3 commands

- Check: `cargo blinky-s3-check`
- Build only: `cargo blinky-s3-build`

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

## Notes

- C6 aliases use stable-friendly commands (no `-Z build-std`).
- S3 aliases use `-Z build-std=core,alloc`.
- Runner is `espflash flash --monitor`, so `cargo blinky*` flashes and opens serial monitor.
- Logging uses `log::info!` through `esp-println`.
