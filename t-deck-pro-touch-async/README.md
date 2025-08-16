# t-deck-pro-touch-async

An asynchronous, `no_std` Rust driver for the T-Deck's CST328 touch controller, designed for the Embassy framework.

## Features

*   Asynchronous reading of touch events.
*   Initialization and handling of the CST328 touch controller.
*   Designed for the `xtensa-esp32s3-none-elf` target.
*   Licensed under Apache 2.0.

## Prerequisites

Ensure you have the ESP-RS toolchain installed. Instructions can be found at [The Rust on ESP Book](https://esp-rs.github.io/book/).

You will need:
*   `espup`
*   `cargo-espflash`

## Building the Example

The project is configured to build for the `xtensa-esp32s3-none-elf` target by default.

To build the included `simple_touch` example:

```bash
cargo build --example simple_touch
```

### Flashing the Example

To flash the example to your T-Deck device:

```bash
cargo flash --example simple_touch
```

You can monitor the serial output with a tool like `espflash monitor` to see touch events.

## Usage

Here is a minimal example of how to initialize the touch driver and read touch events within an Embassy `#[main]` task.

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
use t_deck_pro_touch_async::touch::TouchController;
use embassy_time::{Duration, Timer};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // Initialize peripherals and clocks
    let peripherals = esp_hal::init(Config::default().with_cpu_clock(CpuClock::max()));

    // Configure I2C pins
    let touch_scl = peripherals.GPIO14;
    let touch_sda = peripherals.GPIO13;
    let touch_int = Input::new(peripherals.GPIO12, InputConfig::default());

    // Configure I2C peripheral
    let config = esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100));
    let touch_i2c = I2c::new(peripherals.I2C0, config)
        .unwrap()
        .with_sda(touch_sda)
        .with_scl(touch_scl)
        .into_async();

    // Create and initialize the touch controller
    let mut touch_controller = TouchController::new(touch_i2c, touch_int, None);
    touch_controller.init().await.unwrap();

    // Spawn a task to read touch events
    spawner.spawn(read_touch(touch_controller)).unwrap();
}

#[embassy_executor::task]
async fn read_touch(mut touch_controller: TouchController<'static>) {
    loop {
        match touch_controller.read_touches().await {
            Ok(touches) => {
                if !touches.is_empty() {
                    // log::info!("Touches detected {touches:?}");
                }
            }
            Err(_) => { /* Handle error */ }
        }
        Timer::after(Duration::from_millis(20)).await;
    }
}
```

## Acknowledgements

This driver was developed with reference to the following projects:

*   [esphome-CST328-Touch](https://github.com/BluetriX/esphome-CST328-Touch.git)
*   The CST328 driver in the [Xinyuan-LilyGO/T-Deck-Pro](https://github.com/Xinyuan-LilyGO/T-Deck-Pro.git) repository.
*   [CIRCUITSTATE/CSE_CST328](https://github.com/CIRCUITSTATE/CSE_CST328)

## License

This project is licensed under the Apache 2.0 License. See the `LICENSE` file for details.
