//! An asynchronous, `no_std` driver for the T-Deck's keyboard.
//!
//! This driver provides a `KeyboardController` to interact with the TCA8418 I2C
//! keyboard scanner IC. It allows for initializing the keyboard and reading key
//! press and release events.
//!
//! # Usage
//!
//! To use this driver, you need an I2C peripheral implementation that satisfies the
//! `embedded-hal-async::i2c::I2c` trait, along with the interrupt and reset GPIO pins.
//!
//! ```no_run
//! # #![no_std]
//! # #![no_main]
//! # use esp_hal::prelude::*;
//! # use esp_hal::i2c::master::I2c;
//! # use esp_hal::gpio::{Input, InputConfig, Output, Level, OutputConfig};
//! # use esp_hal::Config;
//! # use esp_hal::clock::CpuClock;
//! # use esp_hal::time::Rate;
//! # use embassy_executor::Spawner;
//! use t_deck_pro_keyboard_async::keyboard::KeyboardController;
//!
//! #[esp_hal_embassy::main]
//! async fn main(spawner: Spawner) {
//!     // Initialize peripherals and I2C
//!     let peripherals = esp_hal::init(Config::default().with_cpu_clock(CpuClock::max()));
//!     let keyboard_scl = peripherals.GPIO14;
//!     let keyboard_sda = peripherals.GPIO13;
//!     let keyboard_int = Input::new(peripherals.GPIO15, InputConfig::default());
//!     let config = esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100));
//!     let i2c = I2c::new(peripherals.I2C0, config)
//!         .unwrap()
//!         .with_sda(keyboard_sda)
//!         .with_scl(keyboard_scl)
//!         .into_async();
//!
//!     // Create and initialize the keyboard controller
//!     let mut keyboard_controller = KeyboardController::new(i2c, keyboard_int, None);
//!     keyboard_controller.init().await.unwrap();
//!
//!     // Spawn a task to read key events
//!     spawner.spawn(read_keys(keyboard_controller)).unwrap();
//! }
//!
//! #[embassy_executor::task]
//! async fn read_keys(mut keyboard_controller: KeyboardController<'static, I2c<'static, esp_hal::Async>, esp_hal::i2c::master::Error>) {
//!     loop {
//!         if let Ok(events) = keyboard_controller.read_key_events().await {
//!             for event in events {
//!                 // log::info!("Key Event: {:?}", event);
//!             }
//!         }
//!     }
//! }
//! ```

#![no_std]

pub mod keyboard;
