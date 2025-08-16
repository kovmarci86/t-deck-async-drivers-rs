//! # I2C Scanner Application
//!
//! This application initializes the I2C bus on a T-Deck device and scans for connected
//! I2C peripherals. It periodically prints the addresses of any detected devices to
//! the console. This is useful for debugging and verifying that I2C devices are
//! correctly connected and responsive.

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::I2c;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

/// The main entry point of the application.
///
/// This function initializes the necessary hardware, including the I2C bus,
/// and then enters an infinite loop to scan for I2C devices.
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0
    esp_println::logger::init_logger(log::LevelFilter::Debug);
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let i2c_scl = peripherals.GPIO14;
    let i2c_sda = peripherals.GPIO13;

    let config = esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100));

    let mut rst = Output::new(peripherals.GPIO45, Level::High, OutputConfig::default());
    rst.set_low();
    Timer::after(Duration::from_millis(20)).await;
    rst.set_high();
    Timer::after(Duration::from_millis(300)).await;

    let mut i2c = I2c::new(peripherals.I2C0, config)
        .expect("Failed to create i2c.")
        .with_sda(i2c_sda)
        .with_scl(i2c_scl)
        .into_async();

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        log::debug!("i2c scan starting.");
        // Iterate through all possible 7-bit I2C addresses
        for address in 0x08..=0x77 {
            // Attempt a zero-byte write to the address.
            // The `write` method sends the address and checks for an ACK.
            // An empty slice `&[]` means we are not sending any data, just probing.
            let result = i2c.write_async(address, &[]).await;

            match result {
                // If the write succeeds (Ok), a device is present at this address.
                Ok(_) => {
                    log::debug!("   -> Found device at address 0x{address:02X}");
                }
                // If it fails (Err), no device responded. We can ignore the error.
                Err(_err) => {
                    // No device at this address
                }
            }
        }
        log::debug!("i2c scan complete.");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
