# t-deck-pro-gps-async

An asynchronous, `no_std` driver for the GPS module on the LilyGo T-Deck, designed for the Embassy framework.

## Features

*   Asynchronous reading of GPS data.
*   Parsing of NMEA 0183 sentences.
*   Control over the GPS module's power state.
*   Designed for the `xtensa-esp32s3-none-elf` target.

## Prerequisites

Ensure you have the ESP-RS toolchain installed for `xtensa-esp32s3-none-elf`. Instructions can be found at [The Rust on ESP Book](https://esp-rs.github.io/book/).

You will need:
*   `espup`
*   `cargo-espflash`

## Building the Example

To build the included `simple_gps` example:

```bash
cargo build --example simple_gps --target xtensa-esp32s3-none-elf
```

### Flashing the Example

To flash the example to your T-Deck device:

```bash
cargo flash --example simple_gps --target xtensa-esp32s3-none-elf
```

You can monitor the serial output with a tool like `espflash monitor` or the provided `read_serial.sh` script.

## Usage

Here is a minimal example of how to initialize the GPS driver and read data within an Embassy `#[main]` task.

```rust
# #![no_std]
# #![no_main]
# use esp_hal::prelude::*;
# use esp_hal::gpio::{Output, Level, OutputConfig, Pin};
# use esp_hal::uart::Uart;
# use esp_hal::Config;
# use esp_hal::clock::CpuClock;
# use embassy_executor::Spawner;
use t_deck_pro_gps_async::{Gps, PowerMode};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Initialize peripherals and clocks
    let peripherals = esp_hal::init(Config::default().with_cpu_clock(CpuClock::max()));

    // Configure GPIO pins for UART and GPS power
    let tx = peripherals.GPIO43.degrade();
    let rx = peripherals.GPIO44.degrade();
    let enable_pin = Output::new(peripherals.GPIO39, Level::High, OutputConfig::default());

    // Initialize the GPS driver
    let mut gps = Gps::new(peripherals.UART1, tx, rx, enable_pin);

    // Set the desired power mode
    if let Err(_) = gps.set_power_mode(PowerMode::Interval).await {
        // Handle error
    }

    // Main loop to read GPS messages
    loop {
        match gps.read_message().await {
            Ok(gps_data) => {
                // Process the received GPS data
                // log::info!("Received new GPS data: {:?}", gps_data);
            }
            Err(_) => {
                // Handle read error
            }
        }
    }
}
```

## Acknowledgements

This driver was developed with reference to the examples in the [Xinyuan-LilyGO/T-Deck-Pro](https://github.com/Xinyuan-LilyGO/T-Deck-Pro.git) repository.
