# i2c-tester

An asynchronous, `no_std` Rust application for testing I2C communication on the T-Deck, designed for the Embassy framework.

## Features

*   Initializes the I2C bus.
*   Scans for and lists connected I2C devices.
*   Designed for the `xtensa-esp32s3-none-elf` target.

## Prerequisites

Ensure you have the ESP-RS toolchain installed. Instructions can be found at [The Rust on ESP Book](https://esp-rs.github.io/book/).

You will need:
*   `espup`
*   `cargo-espflash`

## Building the Application

The project is configured to build for the `xtensa-esp32s3-none-elf` target by default.

To build the application:

```bash
cargo build --bin i2c-tester
```

### Flashing the Application

To flash the application to your T-Deck device:

```bash
cargo flash --bin i2c-tester
```

You can monitor the serial output to see the list of detected I2C devices.

## License

This project is licensed under the Apache 2.0 License. See the `LICENSE` file for details.
