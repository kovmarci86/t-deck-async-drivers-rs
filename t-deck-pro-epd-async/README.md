# t-deck-pro-epd-async

This is a Rust library for the LilyGo T-Deck device, providing an asynchronous driver for the e-paper display (EPD). It uses `esp-hal` for the ESP32-S3 and the Embassy framework for asynchronous operations. The driver implements the `embedded-graphics` `DrawTarget` trait, allowing for easy drawing of shapes, text, and images.

## Features

*   Asynchronous, non-blocking driver for the T-Deck's e-paper display.
*   Implements `embedded-graphics` `DrawTarget`.
*   Designed for the `xtensa-esp32s3-none-elf` target.
*   Licensed under Apache 2.0.

## Prerequisites

Ensure you have the ESP-RS toolchain installed for `xtensa-esp32s3-none-elf`. Instructions can be found at [The Rust on ESP Book](https://esp-rs.github.io/book/).

You will need:
*   `espup`
*   `cargo-espflash`

## Building the Example

To build the included example:

```bash
cargo build --example simple_example --target xtensa-esp32s3-none-elf
```

### Flashing the Example

To flash the example to your T-Deck device:

```bash
cargo flash --example simple_example --target xtensa-esp32s3-none-elf
```

## Usage

Here's a minimal example of how to initialize and use the `EInkDisplay` driver within an Embassy `#[main]` task.

```rust
# #![no_std]
# #![no_main]
# use esp_hal::prelude::*;
# use esp_hal::spi::master::Spi;
# use esp_hal::gpio::{Output, Input, Level, OutputConfig, InputConfig};
# use esp_hal::dma::Dma;
# use esp_hal::dma_buffers;
# use esp_hal::spi::master::prelude::*;
# use esp_hal::Config;
# use esp_hal::clock::CpuClock;
# use esp_hal::spi::Mode;
# use esp_hal::time::Rate;
# use embassy_executor::Spawner;
use t_deck_pro_epd_async::EInkDisplay;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Rectangle, PrimitiveStyle},
};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Initialize peripherals, clocks, and DMA
    let peripherals = esp_hal::init(Config::default().with_cpu_clock(CpuClock::max()));
    let sclk = peripherals.GPIO36;
    let mosi = peripherals.GPIO33;
    let cs = peripherals.GPIO34;
    let dma_channel = peripherals.DMA_CH0;
    let (rx_buffer, _, tx_buffer, _) = dma_buffers!(32000);

    // Initialize SPI
    let mut spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_mosi(mosi)
    .with_cs(cs)
    .with_dma(dma_channel.configure_for_async(false))
    .with_buffers(rx_buffer, tx_buffer)
    .into_async();

    // Initialize EPD control pins
    let epd_dc = Output::new(peripherals.GPIO35, Level::Low, OutputConfig::default());
    let epd_busy = Input::new(peripherals.GPIO37, InputConfig::default());
    let epd_rst = Output::new(peripherals.GPIO45, Level::High, OutputConfig::default());

    // Initialize the display driver
    let mut display = EInkDisplay::new(epd_dc, epd_busy, Some(epd_rst), 240, 320, false);
    display.init(&mut spi).await.ok();

    // Clear the display to white
    display.clear(BinaryColor::Off).unwrap();

    // Draw a rectangle
    Rectangle::new(Point::new(70, 110), Size::new(100, 100))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Write the buffer to the display and refresh
    display.draw_buffer(&mut spi).await.ok();
    display.update_display(&mut spi).await.ok();
}
```

## E-Ink Display Communication Protocol

The **UC8253** controller on the **GDEQ031T10** e-paper display communicates using a **4-wire SPI** interface. This protocol involves a host microcontroller sending commands and data to the display to control its functions.

The key pins involved are:
*   **SCLK (Serial Clock):** Provides the clock signal for SPI communication.
*   **MOSI (Master Out Slave In):** Carries data from the microcontroller to the display.
*   **CS (Chip Select):** Activates the SPI interface on the display. It's typically active low.
*   **DC (Data/Command):** This pin is crucial for distinguishing between command bytes and data bytes. When DC is low, the byte sent on MOSI is interpreted as a command. When DC is high, the byte is interpreted as data for the preceding command.
*   **BUSY:** The display pulls this pin high while it's busy with internal operations, such as refreshing the screen. The microcontroller must wait for the BUSY pin to go low before sending the next command.

The typical communication sequence is as follows:
1.  **Select the device:** Pull the CS pin low.
2.  **Send a command:** Pull the DC pin low and send a 1-byte command over SPI.
3.  **Send data (if required):** Pull the DC pin high and send the required number of data bytes for the command.
4.  **Deselect the device:** Pull the CS pin high.
5.  **Wait for the display to finish:** Poll the BUSY pin until it goes low, indicating the display is ready for the next command.

Common commands include:
*   **Panel Setting (PSR):** Configures display resolution, color settings, and other parameters.
*   **Power On (PON):** Turns on the internal high-voltage generation.
*   **Data Start Transmission (DTM):** Sends image data to the display's internal memory. The image data is typically a bitmap where each bit represents a pixel's color.
*   **Display Refresh (DRF):** Updates the screen to show the image data from memory. This is the step that causes the visible change on the e-paper.
*   **Power Off (POF):** Turns off the high-voltage generation to save power.

## Acknowledgements

This driver was developed with reference to the following projects:

*   [GxEPD2](https://github.com/ZinggJM/GxEPD2.git)
*   The EPD driver in the [Xinyuan-LilyGO/T-Deck-Pro](https://github.com/Xinyuan-LilyGO/T-Deck-Pro.git) repository.

## License

This project is licensed under the Apache 2.0 License. See the `LICENSE` file for details.
