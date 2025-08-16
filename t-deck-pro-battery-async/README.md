# t-deck-pro-battery-async

An asynchronous, `no_std` Rust driver for the BQ25896 battery charger IC on the T-Deck, designed for the Embassy framework.

## Features

*   Asynchronous reading of battery and charging status.
*   Provides data on voltage, current, temperature, and fault conditions.
*   Control over the ADC for power saving.
*   Designed for the `xtensa-esp32s3-none-elf` target.

## Prerequisites

Ensure you have the ESP-RS toolchain installed. Instructions can be found at [The Rust on ESP Book](https://esp-rs.github.io/book/).

You will need:
*   `espup`
*   `cargo-espflash`

## Building the Example

The project is configured to build for the `xtensa-esp32s3-none-elf` target by default.

To build the included `simple_battery` example:

```bash
cargo build --example simple_battery
```

### Flashing the Example

To flash the example to your T-Deck device:

```bash
cargo flash --example simple_battery
```

You can monitor the serial output to see the battery status printed periodically.

## Usage

Here is a minimal example of how to initialize the battery service and read data within an Embassy `#[main]` task.

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
use t_deck_pro_battery_async::BatteryService;
use embassy_time::{Duration, Timer};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // Initialize peripherals and I2C
    let peripherals = esp_hal::init(Config::default().with_cpu_clock(CpuClock::max()));
    let battery_scl = peripherals.GPIO14;
    let battery_sda = peripherals.GPIO13;
    let config = esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100));
    let i2c = I2c::new(peripherals.I2C0, config)
        .unwrap()
        .with_sda(battery_sda)
        .with_scl(battery_scl)
        .into_async();

    // Create the battery service
    let battery_service = BatteryService::new(i2c);

    // Spawn a task to read battery data
    spawner.spawn(read_battery_task(battery_service)).unwrap();
}

#[embassy_executor::task]
async fn read_battery_task(mut battery_service: BatteryService<'static, I2c<'static, esp_hal::Async>, esp_hal::i2c::master::Error>) {
    loop {
        match battery_service.measure().await {
            Ok(data) => {
                // log::info!("Voltage: {:.3} V", data.voltage);
                // log::info!("Charge Current: {:.3} A", data.charge_current);
                // log::info!("Battery Temp Percent: {:.1}%", data.battery_temp_percent);
                // log::info!("Charge Status: {:?}", data.charging_status);
            }
            Err(_) => { /* Handle error */ }
        }
        Timer::after(Duration::from_secs(5)).await;
    }
}
```

## Acknowledgements

This driver was developed with reference to the examples in the [Xinyuan-LilyGO/T-Deck-Pro](https://github.com/Xinyuan-LilyGO/T-Deck-Pro.git) repository.

## License

This project is licensed under the Apache 2.0 License. See the `LICENSE` file for details.
