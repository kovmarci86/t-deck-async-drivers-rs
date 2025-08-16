#![no_std]
#![doc = "Asynchronous shared bus implementations for embedded-hal."]

// Note: This custom implementation was created to navigate challenges with dependency
// version conflicts and API churn in the async embedded ecosystem.
//
// For the official Embassy implementation, see:
// - https://github.com/embassy-rs/embassy/tree/main/embassy-embedded-hal/src/shared_bus

extern crate alloc;

pub mod i2c;
pub mod spi;
