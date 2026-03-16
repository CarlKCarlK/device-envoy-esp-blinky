# device-envoy-esp-blinky

A minimal blinky app built with `device-envoy-esp` using the board's built-in smart LED (WS2812).

No external LED wiring is required.

Jump to the [ESP32-C6 directions](#esp32-c6-directions) or the [ESP32-S3 directions](#esp32-s3-directions).

## ESP32-C6 Directions

### C6 Prerequisites

ESP32-C6 uses the RISC-V target.

- Rust installed
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

If your board uses a different built-in smart LED pin, update `src/main.rs`:

- `LED_PIN_NUM`
- The C6 `BuiltinBlinky::new(...)` pin argument
- The C6 `led_strip! { ... pin: ... }` definition

### C6 Build and run

```bash
cargo blinky
```

`cargo blinky` is an alias for:

```bash
cargo run --release --target riscv32imac-unknown-none-elf --no-default-features
```

The `runner = "espflash flash --monitor"` setting in `.cargo/config.toml`
handles flashing and opening the serial monitor, so `cargo blinky` does both.

Logging uses `log::info!` through `esp-println`.

### Extra C6 commands

- Check: `cargo blinky-check`
- Build only: `cargo blinky-build`

## ESP32-S3 Directions

### S3 Prerequisites

ESP32-S3 uses Xtensa and requires ESP toolchain setup.

```bash
cargo install espup
espup install
source "$HOME/export-esp.sh"
```

You'll either need to run `source "$HOME/export-esp.sh"` every time
or add `. "$HOME/export-esp.sh"` to your shell profile such as
`~/.bashrc`, `~/.zshrc`, or `~/.profile`.

For example, to add it to `~/.bashrc`:

```bash
echo '. "$HOME/export-esp.sh"' >> ~/.bashrc
source ~/.bashrc
```

On Windows, `espup` injects the required environment variables automatically,
so there is no `source` step.

### S3 built-in LED pin

This project defaults to `GPIO48` for ESP32-S3 dev boards.

If your board uses a different built-in smart LED pin, update `src/main.rs`:

- `LED_PIN_NUM`
- The S3 `BuiltinBlinky::new(...)` pin argument
- The S3 `led_strip! { ... pin: ... }` definition

### S3 Build and run

```bash
cargo +esp blinky-s3
```

This runs the `blinky-s3` cargo alias through the `esp` toolchain.

`cargo +esp blinky-s3` expands to:

```bash
cargo +esp run -Z build-std=core,alloc --release --target xtensa-esp32s3-none-elf --no-default-features
```

The `runner = "espflash flash --monitor"` setting in `.cargo/config.toml`
handles flashing and opening the serial monitor, so `cargo +esp blinky-s3` does both.

Logging uses `log::info!` through `esp-println`.

### Extra S3 commands

- Check: `cargo +esp blinky-s3-check`
- Build only: `cargo +esp blinky-s3-build`
