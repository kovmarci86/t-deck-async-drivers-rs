//! An asynchronous, `no_std` driver for the T-Deck's LoRa module (SX1262).
//!
//! This driver provides a `LoraRadio` struct to interact with the SX1262 LoRa transceiver.
//! It is built upon the `sx126x-async-rs` crate and is designed to be used with a shared SPI bus,
//! such as the one provided by the `embedded-bus-async` crate.
//!
//! # Usage
//!
//! ```no_run
//! # #![no_std]
//! # #![no_main]
//! # use esp_hal::prelude::*;
//! # use esp_hal::spi::master::{Spi, Config};
//! # use esp_hal::dma_buffers;
//! # use esp_hal::gpio::{Input, InputConfig, Output, Level, OutputConfig};
//! # use esp_hal::Config as EspConfig;
//! # use esp_hal::clock::CpuClock;
//! # use esp_hal::time::Rate;
//! # use esp_hal::spi::Mode;
//! # use embassy_executor::Spawner;
//! # use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, rwlock::RwLock};
//! # use embassy_time::Delay;
//! # use core::cell::RefCell;
//! use alloc::rc::Rc;
//! use t_deck_pro_lora_async::lora::LoraRadio;
//! use embedded_bus_async::spi::RwLockDevice;
//!
//! #[esp_hal_embassy::main]
//! async fn main(_spawner: Spawner) {
//!     // Initialize peripherals, clocks, and SPI
//!     let peripherals = esp_hal::init(EspConfig::default().with_cpu_clock(CpuClock::max()));
//!     let lora_cs = Output::new(peripherals.GPIO3, Level::High, OutputConfig::default());
//!     let lora_busy = Input::new(peripherals.GPIO6, InputConfig::default());
//!     let lora_rst = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
//!     let lora_int = Input::new(peripherals.GPIO5, InputConfig::default());
//!     let lora_en = Output::new(peripherals.GPIO46, Level::High, OutputConfig::default());
//!     let sclk = peripherals.GPIO36;
//!     let mosi = peripherals.GPIO33;
//!     let miso = peripherals.GPIO47;
//!     let dma_channel = peripherals.DMA_CH0;
//!     let (rx_buffer, _, tx_buffer, _) = dma_buffers!(32000);
//!     let spi = Spi::new(
//!         peripherals.SPI2,
//!         Config::default().with_frequency(Rate::from_mhz(8)).with_mode(Mode::_0),
//!     )
//!     .unwrap()
//!     .with_sck(sclk)
//!     .with_mosi(mosi)
//!     .with_miso(miso)
//!     .with_dma(dma_channel.configure_for_async(false))
//!     .with_buffers(rx_buffer, tx_buffer)
//!     .into_async();
//!
//!     // Create a shared SPI bus and a device interface for the LoRa radio
//!     let spi_bus = Rc::new(RwLock::new(spi));
//!     let lora_spi = RwLockDevice::new(spi_bus, lora_cs, Delay);
//!
//!     // Initialize the LoRa radio
//!     let mut lora = LoraRadio::new(lora_spi, lora_rst, lora_int, lora_busy, lora_en);
//!     lora.init().await.unwrap();
//!
//!     // Send a message
//!     lora.send(b"Hello LoRa!").await.unwrap();
//! }
//! ```

#![no_std]
#![deny(missing_docs)]

//! A basic, asynchronous driver for the T-Deck LoRa module.
extern crate alloc;

/// The LoRa module driver.
pub mod lora;
