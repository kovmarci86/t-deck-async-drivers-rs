# sx126x-async-rs

This is an async port of the [sx126x-rs](https://github.com/tweedegolf/sx126x-rs) driver crate for Semtech's SX1261/62 family of LoRa trancievers. This version has been heavily modified to support async operations using `embedded-hal-async`.

[Original Documentation on docs.rs](https://docs.rs/sx126x)

## Purpose

This crate provides a `no_std` async driver for the Semtech SX126x series of LoRa transceivers. It is designed to be used in embedded systems and requires an implementation of the `embedded-hal-async` traits.

## Features

*   Async, non-blocking API for all operations.
*   Supports LoRa packet types.
*   Configuration of all major modem parameters.
*   High-level methods for sending and receiving data.

## Status

This crate is currently under development and should be considered a work in progress. The API may change in the future.

## Dependencies

This driver relies on the `embedded-hal-async` traits for SPI and GPIO communication. You will need to use a HAL (Hardware Abstraction Layer) crate for your target hardware that implements these traits.

## Usage

Here is a conceptual example of how to initialize the driver and send a LoRa message. You will need to replace the placeholder types with the actual types provided by your HAL.

```rust
# #![no_std]
# use sx126x_async_rs::{SX126x, Config, op::PacketType, op::LoRaPacketParams, op::LoRaHeaderType, op::LoRaCrcType, op::RxTxTimeout};
# use embedded_hal_async::spi::SpiDevice;
# use embedded_hal_async::digital::Wait;
# use embedded_hal::digital::{OutputPin, InputPin};
#
# struct MySpi;
# impl SpiDevice for MySpi {
#     type Error = ();
#     async fn transaction(&mut self, operations: &mut [embedded_hal_async::spi::Operation<'_, u8>]) -> Result<(), Self::Error> { Ok(()) }
# }
#
# struct MyPin;
# impl OutputPin for MyPin {
#     type Error = ();
#     fn set_low(&mut self) -> Result<(), Self::Error> { Ok(()) }
#     fn set_high(&mut self) -> Result<(), Self::Error> { Ok(()) }
# }
# impl InputPin for MyPin {
#     type Error = ();
#     fn is_high(&self) -> Result<bool, Self::Error> { Ok(false) }
#     fn is_low(&self) -> Result<bool, Self::Error> { Ok(true) }
# }
# impl Wait for MyPin {
#     async fn wait_for_high(&mut self) -> Result<(), Self::Error> { Ok(()) }
#     async fn wait_for_low(&mut self) -> Result<(), Self::Error> { Ok(()) }
# }
#
async fn main() {
    // Initialize your SPI device and GPIO pins here.
    // These will be specific to your hardware and HAL.
    let spi: MySpi = MySpi; // Your SPI device
    let nrst_pin: MyPin = MyPin; // Reset pin
    let busy_pin: MyPin = MyPin; // Busy pin
    let ant_pin: MyPin = MyPin; // Antenna switch pin
    let dio1_pin: MyPin = MyPin; // DIO1 pin

    // Create a new SX126x instance
    let mut lora = SX126x::new(spi, (nrst_pin, busy_pin, ant_pin, dio1_pin));

    // Create a configuration for the modem
    let mut conf = Config::default();
    conf.packet_type = PacketType::LoRa;
    // ... set other configuration parameters as needed ...

    // Initialize the modem
    lora.init(conf).await.unwrap();

    // The data to send
    let message = b"Hello, LoRa!";

    // Send the message
    lora.write_bytes(
        message,
        RxTxTimeout::from_ms(5000), // Timeout
        2, // Preamble length
        LoRaCrcType::On,
    )
    .await
    .unwrap();
}
```