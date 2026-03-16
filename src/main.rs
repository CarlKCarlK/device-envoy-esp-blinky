#![no_std]
#![no_main]

use core::convert::Infallible;

use embassy_executor::Spawner;
use embassy_time::Duration;
use esp_backtrace as _;
use log::info;

use device_envoy_esp::{
    init_and_start, led_strip,
    led_strip::{Current, Frame1d, LedStrip as _, colors},
};

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(target_arch = "riscv32")]
const LED_PIN_NUM: u8 = 8;
#[cfg(target_arch = "xtensa")]
const LED_PIN_NUM: u8 = 38;

const DOT_MS: u64 = 200;
const DASH_MS: u64 = DOT_MS * 3;
const SYMBOL_GAP_MS: u64 = DOT_MS;
const LETTER_GAP_MS: u64 = DOT_MS * 3;
const WORD_GAP_MS: u64 = DOT_MS * 7;

const DOT_DURATION: Duration = Duration::from_millis(DOT_MS);
const DASH_DURATION: Duration = Duration::from_millis(DASH_MS);
const SYMBOL_GAP_DURATION: Duration = Duration::from_millis(SYMBOL_GAP_MS);
const LETTER_GAP_DURATION: Duration = Duration::from_millis(LETTER_GAP_MS);
const WORD_GAP_DURATION: Duration = Duration::from_millis(WORD_GAP_MS);

const ON_COLOR: Frame1d<1> = Frame1d([colors::WHITE]);
const OFF_COLOR: Frame1d<1> = Frame1d([colors::BLACK]);

const SOS_PATTERN: [(Frame1d<1>, Duration); 18] = [
    (ON_COLOR, DOT_DURATION),
    (OFF_COLOR, SYMBOL_GAP_DURATION),
    (ON_COLOR, DOT_DURATION),
    (OFF_COLOR, SYMBOL_GAP_DURATION),
    (ON_COLOR, DOT_DURATION),
    (OFF_COLOR, LETTER_GAP_DURATION),
    (ON_COLOR, DASH_DURATION),
    (OFF_COLOR, SYMBOL_GAP_DURATION),
    (ON_COLOR, DASH_DURATION),
    (OFF_COLOR, SYMBOL_GAP_DURATION),
    (ON_COLOR, DASH_DURATION),
    (OFF_COLOR, LETTER_GAP_DURATION),
    (ON_COLOR, DOT_DURATION),
    (OFF_COLOR, SYMBOL_GAP_DURATION),
    (ON_COLOR, DOT_DURATION),
    (OFF_COLOR, SYMBOL_GAP_DURATION),
    (ON_COLOR, DOT_DURATION),
    (OFF_COLOR, WORD_GAP_DURATION),
];

#[cfg(target_arch = "riscv32")]
led_strip! {
    BuiltinBlinky {
        pin: GPIO8,
        len: 1,
        max_current: Current::Milliamps(10),
        max_frames: 20,
    }
}

#[cfg(target_arch = "xtensa")]
led_strip! {
    BuiltinBlinky {
        pin: GPIO38,
        len: 1,
        max_current: Current::Milliamps(10),
        max_frames: 20,
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let err = inner_main(spawner).await.unwrap_err();
    panic!("{err:?}");
}

async fn inner_main(spawner: Spawner) -> device_envoy_esp::Result<Infallible> {
    init_and_start!(p, rmt80: rmt80, mode: rmt_mode::Blocking);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    info!("device-envoy-esp-blinky boot");
    info!("built-in smart LED on GPIO{}", LED_PIN_NUM);

    #[cfg(target_arch = "riscv32")]
    let builtin_blinky = BuiltinBlinky::new(p.GPIO8, rmt80.channel0, spawner)?;
    #[cfg(target_arch = "xtensa")]
    let builtin_blinky = BuiltinBlinky::new(p.GPIO38, rmt80.channel0, spawner)?;

    builtin_blinky.animate(&SOS_PATTERN);
    info!("SOS animation started");

    core::future::pending().await
}
