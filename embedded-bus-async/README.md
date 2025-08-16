# embedded-bus-async

Asynchronous shared bus implementations for `embedded-hal-async`.

This crate provides `RwLock`-based shared bus implementations for SPI and I2C, allowing a single bus peripheral to be safely shared by multiple device drivers in an asynchronous context.

## Overview

In embedded systems, it's common for multiple devices (e.g., a screen, a touch controller, and a sensor) to share a single communication bus like I2C or SPI. The `embedded-hal` ecosystem provides traits for peripherals, but managing shared access can be complex, especially in an async environment.

This crate offers convenient wrappers that implement the `SpiDevice` and `I2c` traits, handling the locking and resource management internally.

## Implementations

-   **`spi::RwLockDevice`**: An async `SpiDevice` implementation that wraps a shared `SpiBus`. It manages its own Chip Select (CS) pin, ensuring exclusive bus access during transactions.
-   **`i2c::RwLockI2cDevice`**: An async `I2c` implementation that wraps a shared `I2c` bus.

## Usage

### Shared SPI Bus

```rust
# #![no_std]
# use esp_hal::prelude::*;
# use esp_hal::spi::master::{Spi, Config};
# use esp_hal::gpio::{Output, Level, OutputConfig};
# use esp_hal::Config as EspConfig;
# use esp_hal::clock::CpuClock;
# use esp_hal::time::Rate;
# use esp_hal::spi::Mode;
# use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, rwlock::RwLock};
# use embassy_time::Delay;
# use core::cell::RefCell;
# use alloc::rc::Rc;
use embedded_bus_async::spi::RwLockDevice;

// 1. Create your SpiBus instance
// let spi_bus = Spi::new(...).into_async();

// 2. Wrap the bus in a RwLock and an Rc for sharing
// let shared_spi_bus = Rc::new(RwLock::new(spi_bus));

// 3. Create devices for each peripheral on the bus
// let lora_cs = Output::new(...);
// let lora_spi = RwLockDevice::new(shared_spi_bus.clone(), lora_cs, Delay);
//
// let display_cs = Output::new(...);
// let display_spi = RwLockDevice::new(shared_spi_bus.clone(), display_cs, Delay);

// 4. You can now use `lora_spi` and `display_spi` as if they were dedicated SPI peripherals.
// lora.init(&mut lora_spi).await;
// display.init(&mut display_spi).await;
```

### Shared I2C Bus

```rust
# #![no_std]
# use esp_hal::prelude::*;
# use esp_hal::i2c::master::I2c;
# use esp_hal::Config;
# use esp_hal::clock::CpuClock;
# use esp_hal::time::Rate;
# use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, rwlock::RwLock};
# use core::cell::RefCell;
# use alloc::rc::Rc;
use embedded_bus_async::i2c::RwLockI2cDevice;

// 1. Create your I2c bus instance
// let i2c_bus = I2c::new(...).into_async();

// 2. Wrap the bus in a RwLock and an Rc for sharing
// let shared_i2c_bus = Rc::new(RwLock::new(i2c_bus));

// 3. Create devices for each peripheral on the bus
// let keyboard_i2c = RwLockI2cDevice::new(shared_i2c_bus.clone());
// let touch_i2c = RwLockI2cDevice::new(shared_i2c_bus.clone());

// 4. You can now use `keyboard_i2c` and `touch_i2c` as if they were dedicated I2C peripherals.
// keyboard.init(&mut keyboard_i2c).await;
// touch.init(&mut touch_i2c).await;
```

## Design Notes

This custom shared bus implementation was created to navigate the complexities of the rapidly evolving async embedded ecosystem. Challenges such as conflicting dependency versions, frequent API changes, and similarly named traits can make integration difficult. This crate offers a stable, lightweight alternative tailored to the project's needs.

For reference, the official Embassy shared bus implementation can be found here:
-   [embassy-embedded-hal/src/shared_bus](https://github.com/embassy-rs/embassy/tree/main/embassy-embedded-hal/src/shared_bus)

## Acknowledgements & References

This crate provides implementations for the asynchronous traits defined in `embedded-hal-async`:
-   [`spi.rs`](https://github.com/rust-embedded/embedded-hal/blob/master/embedded-hal-async/src/spi.rs)
-   [`i2c.rs`](https://github.com/rust-embedded/embedded-hal/blob/master/embedded-hal-async/src/i2c.rs)

The implementation approach is a direct port of the patterns used in the synchronous [`embedded-hal-bus`](https://github.com/rust-embedded/embedded-hal/tree/master/embedded-hal-bus) crate, updated to use `async` traits and modern dependencies.
