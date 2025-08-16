# t-deck-pro-keyboard-async

An asynchronous, `no_std` Rust driver for the T-Deck's keyboard controller, designed for the Embassy framework. The keyboard is managed by a TCA8418 I2C-based keypad scan IC.

## Features

*   Asynchronous reading of keyboard events.
*   Initialization and handling of the TCA8418 keyboard controller.
*   Designed for the `xtensa-esp32s3-none-elf` target.
*   Licensed under Apache 2.0.

## Prerequisites

Ensure you have the ESP-RS toolchain installed. Instructions can be found at [The Rust on ESP Book](https://esp-rs.github.io/book/).

You will need:
*   `espup`
*   `cargo-espflash`

## Building the Example

The project is configured to build for the `xtensa-esp32s3-none-elf` target by default.

To build the included `simple_keyboard` example:

```bash
cargo build --example simple_keyboard
```

### Flashing the Example

To flash the example to your T-Deck device:

```bash
cargo flash --example simple_keyboard
```

You can monitor the serial output with a tool like `espflash monitor` to see key press events.

## Usage

Here is a minimal example of how to initialize the keyboard driver and read key events within an Embassy `#[main]` task.

```rust
# #![no_std]
# #![no_main]
# use esp_hal::prelude::*;
# use esp_hal::i2c::master::I2c;
# use esp_hal::gpio::{Input, InputConfig, Output, Level, OutputConfig};
# use esp_hal::Config;
# use esp_hal::clock::CpuClock;
# use esp_hal::time::Rate;
# use embassy_executor::Spawner;
use t_deck_pro_keyboard_async::keyboard::KeyboardController;
use embassy_time::{with_timeout, Duration};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // Initialize peripherals and clocks
    let peripherals = esp_hal::init(Config::default().with_cpu_clock(CpuClock::max()));

    // Configure I2C pins
    let keyboard_scl = peripherals.GPIO14;
    let keyboard_sda = peripherals.GPIO13;
    let keyboard_int = Input::new(peripherals.GPIO15, InputConfig::default());

    // Configure I2C peripheral
    let config = esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100));
    let keyboard_i2c = I2c::new(peripherals.I2C0, config)
        .unwrap()
        .with_sda(keyboard_sda)
        .with_scl(keyboard_scl)
        .into_async();

    // Create and initialize the keyboard controller
    let mut keyboard_controller = KeyboardController::new(keyboard_i2c, keyboard_int, None);
    keyboard_controller.init().await.unwrap();

    // Spawn a task to read key events
    spawner.spawn(read_keys(keyboard_controller)).unwrap();
}

#[embassy_executor::task]
async fn read_keys(mut keyboard_controller: KeyboardController<'static, I2c<'static, esp_hal::Async>, esp_hal::i2c::master::Error>) {
    loop {
        if let Ok(Ok(keys)) = with_timeout(
            Duration::from_secs(5),
            keyboard_controller.read_key_events(),
        ).await {
            if !keys.is_empty() {
                // log::info!("Key events detected {keys:?}");
            }
        }
    }
}
```

## Acknowledgements

This driver was developed with reference to the TCA8418 driver in the [Xinyuan-LilyGO/T-Deck-Pro](https://github.com/Xinyuan-LilyGO/T-Deck-Pro.git) repository.

## License

This project is licensed under the Apache 2.0 License. See the `LICENSE` file for details.
