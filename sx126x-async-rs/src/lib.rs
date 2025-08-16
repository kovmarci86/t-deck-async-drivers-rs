//! An asynchronous, `no_std` driver for the Semtech SX126x family of LoRa transceivers.
//!
//! This crate provides a low-level async driver for the SX1261 and SX1262 LoRa chips.
//! It is built upon `embedded-hal-async` traits and provides access to the full
//! command set of the SX126x series.
//!
//! The main entry point is the `SX126x` struct, which takes an async SPI peripheral
//! and the necessary GPIO pins to communicate with the modem.
//!
//! # Usage
//!
//! See the `t-deck-pro-lora-async` crate for a higher-level example of how this
//! driver can be used.

#![no_std]

pub mod conf;
pub mod op;
pub mod reg;

mod sx;
pub use sx::*;
