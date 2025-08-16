#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those     holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{with_timeout, Duration, Timer};
use esp_hal::i2c::master::I2c;
use esp_hal::{
    clock::CpuClock,
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    time::Rate,
    timer::systimer::SystemTimer,
};
use esp_println::println;
use log::{debug, error, info, warn};
use t_deck_pro_keyboard_async::keyboard::KeyboardController;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

/// The main entry point of the application.
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // Init logging
    esp_println::logger::init_logger(log::LevelFilter::Debug);

    info!("Logger initialized");
    debug!("This is a debug message");
    warn!("This is a warning");
    error!("This is an error");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    info!("Peripherals initialized");

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let shared_rst = Output::new(peripherals.GPIO45, Level::High, OutputConfig::default());

    let keyboard_scl = peripherals.GPIO14;
    let keyboard_sda = peripherals.GPIO13;
    let keyboard_int = Input::new(peripherals.GPIO15, InputConfig::default());

    let config = esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100));

    let keyboard_i2c = I2c::new(peripherals.I2C0, config)
        .unwrap()
        .with_sda(keyboard_sda)
        .with_scl(keyboard_scl)
        .into_async();
    let mut keyboard_controller =
        KeyboardController::new(keyboard_i2c, keyboard_int, Some(shared_rst));
    match keyboard_controller.init().await {
        Ok(_) => log::debug!("Keyboard controller initialized."),
        Err(_) => log::warn!("Error initializing keyboard controller."),
    };

    spawner.spawn(read_keys(keyboard_controller)).unwrap();

    info!("Drawing complete. Entering idle loop.");
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

/// A task that continuously reads key events and logs them.
#[embassy_executor::task]
async fn read_keys(
    mut keyboard_controller: KeyboardController<
        'static,
        I2c<'static, esp_hal::Async>,
        esp_hal::i2c::master::Error,
    >,
) {
    loop {
        if let Ok(Ok(keys)) = with_timeout(
            Duration::from_secs(5),
            keyboard_controller.read_key_events(),
        )
        .await
        {
            if !keys.is_empty() {
                info!("Key events detected {keys:?}");
            }
        } else {
            log::debug!("No keys.");
        }
    }
}
