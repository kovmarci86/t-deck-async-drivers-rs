# t-deck-pro-lora-async

An asynchronous, `no_std` Rust driver for the T-Deck's LoRa module (Semtech SX1262), designed for the Embassy framework.

## Features

*   Asynchronous sending and receiving of LoRa packets.
*   Initialization and configuration of the SX1262 LoRa radio.
*   SPI communication managed via a shared, thread-safe bus.
*   Designed for the `xtensa-esp32s3-none-elf` target.
*   Licensed under Apache 2.0.

## Prerequisites

Ensure you have the ESP-RS toolchain installed. Instructions can be found at [The Rust on ESP Book](https://esp-rs.github.io/book/).

You will need:
*   `espup`
*   `cargo-espflash`

## Building the Examples

The project is configured to build for the `xtensa-esp32s3-none-elf` target by default.

To build the sender example:
```bash
cargo build --example example_sender
```

To build the receiver example:
```bash
cargo build --example example_receiver
```

### Flashing the Examples

To flash the sender example to your T-Deck device:
```bash
cargo flash --example example_sender
```

To flash the receiver example to another T-Deck device:
```bash
cargo flash --example example_receiver
```

You can monitor the serial output on the receiver to see incoming messages.

## Usage

Here is a minimal example of how to initialize the LoRa driver and send or receive data within an Embassy `#[main]` task.

```rust
# #![no_std]
# #![no_main]
# use esp_hal::prelude::*;
# use esp_hal::spi::master::{Spi, Config};
# use esp_hal::dma_buffers;
# use esp_hal::gpio::{Input, InputConfig, Output, Level, OutputConfig};
# use esp_hal::Config as EspConfig;
# use esp_hal::clock::CpuClock;
# use esp_hal::time::Rate;
# use esp_hal::spi::Mode;
# use embassy_executor::Spawner;
# use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, rwlock::RwLock};
# use embassy_time::Delay;
# use core::cell::RefCell;
use alloc::rc::Rc;
use t_deck_pro_lora_async::lora::LoraRadio;
use t_deck_pro_lora_async::spibus::RwLockDevice;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Initialize peripherals and clocks
    let peripherals = esp_hal::init(EspConfig::default().with_cpu_clock(CpuClock::max()));

    // Configure SPI and GPIO pins
    let lora_cs = Output::new(peripherals.GPIO3, Level::High, OutputConfig::default());
    let lora_busy = Input::new(peripherals.GPIO6, InputConfig::default());
    let lora_rst = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let lora_int = Input::new(peripherals.GPIO5, InputConfig::default());
    let lora_en = Output::new(peripherals.GPIO46, Level::High, OutputConfig::default());
    let sclk = peripherals.GPIO36;
    let mosi = peripherals.GPIO33;
    let miso = peripherals.GPIO47;
    let dma_channel = peripherals.DMA_CH0;

    // Configure SPI
    let (rx_buffer, _, tx_buffer, _) = dma_buffers!(32000);
    let spi = Spi::new(
        peripherals.SPI2,
        Config::default().with_frequency(Rate::from_mhz(8)).with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_mosi(mosi)
    .with_miso(miso)
    .with_dma(dma_channel.configure_for_async(false))
    .with_buffers(rx_buffer, tx_buffer)
    .into_async();

    // Create a shared SPI bus and a device interface
    let spi_bus = Rc::new(RwLock::new(spi));
    let lora_spi = RwLockDevice::new(spi_bus, lora_cs, Delay);

    // Initialize the LoRa radio
    let mut lora = LoraRadio::new(lora_spi, lora_rst, lora_int, lora_busy, lora_en);
    lora.init().await.unwrap();

    // --- Sending Example ---
    // lora.send(b"Hello LoRa!").await.unwrap();

    // --- Receiving Example ---
    // let mut buffer = [0u8; 255];
    // match lora.receive(&mut buffer).await {
    //     Ok(len) if len > 0 => {
    //         // log::info!("Received: {:?}", &buffer[..len]);
    //     }
    //     _ => { /* Handle error or no data */ }
    // }
}
```

## Acknowledgements

This driver was developed with reference to the examples in the [Xinyuan-LilyGO/T-Deck-Pro](https://github.com/Xinyuan-LilyGO/T-Deck-Pro.git) repository.

## License

This project is licensed under the Apache 2.0 License. See the `LICENSE` file for details.
