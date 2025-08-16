//! An asynchronous, `no_std` driver for the T-Deck's CST328 touch controller.
//!
//! This driver provides a `TouchController` to interact with the CST328 I2C
//! touch controller. It allows for initializing the controller and reading
//! touch events.
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
//! use t_deck_pro_touch_async::touch::TouchController;
//!
//! #[esp_hal_embassy::main]
//! async fn main(spawner: Spawner) {
//!     // Initialize peripherals and I2C
//!     let peripherals = esp_hal::init(Config::default().with_cpu_clock(CpuClock::max()));
//!     let touch_scl = peripherals.GPIO14;
//!     let touch_sda = peripherals.GPIO13;
//!     let touch_int = Input::new(peripherals.GPIO12, InputConfig::default());
//!     let config = esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100));
//!     let i2c = I2c::new(peripherals.I2C0, config)
//!         .unwrap()
//!         .with_sda(touch_sda)
//!         .with_scl(touch_scl)
//!         .into_async();
//!
//!     // Create and initialize the touch controller
//!     let mut touch_controller = TouchController::new(i2c, touch_int, None);
//!     touch_controller.init().await.unwrap();
//!
//!     // Spawn a task to read touch events
//!     spawner.spawn(read_touch(touch_controller)).unwrap();
//! }
//!
//! #[embassy_executor::task]
//! async fn read_touch(mut touch_controller: TouchController<'static>) {
//!     loop {
//!         if let Ok(touches) = touch_controller.read_touches().await {
//!             for touch in touches.iter().flatten() {
//!                 // log::info!("Touch Event: {:?}", touch);
//!             }
//!         }
//!     }
//! }
//! ```

#![no_std]

pub mod touch;
