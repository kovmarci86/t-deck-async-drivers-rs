# Device: LILYGO T-Deck Pro (E-Paper Version)

This document details the hardware specifications for the target device.

## Key Specifications

*   **Microcontroller**: Espressif ESP32-S3FN16R8
    *   **CPU**: Dual-core 32-bit Tensilica Xtensa LX7 @ up to 240 MHz
    *   **Memory**: 8MB PSRAM
    *   **Storage**: 16MB SPI Flash
*   **Display**:
    *   **Type**: 3.1-inch E-Paper (GDEQ031T10)
    *   **Resolution**: 320 x 240
    *   **Touch**: Yes (CST328 chip)
*   **Connectivity**:
    *   **Wi-Fi**: 2.4 GHz 802.11n Wi-Fi 4
    *   **Bluetooth**: 5.0 LE
    *   **LoRa**: SX1262 module (868/915 MHz)
    *   **GPS**: U-blox MIA-M10Q module
*   **User Input**:
    *   QWERTY Keyboard
*   **Sensors**:
    *   Bosch BHI260AP AI Smart Sensor (with IMU)
    *   Lite-on LTR-553ALS Light Sensor
*   **Storage**:
    *   MicroSD card slot
*   **Audio**:
    *   3.5mm audio jack, microphone, speaker
*   **Expansion**:
    *   Qwiic connector

## Pinout

| Component       | Function        | Pin  |
| --------------- | --------------- | ---- |
| A7682E (4G)     | RXD             | IO10 |
| A7682E (4G)     | TXD             | IO11 |
| A7682E (4G)     | RI              | IO07 |
| A7682E (4G)     | ITR             | IO08 |
| A7682E (4G)     | PWR             | IO40 |
| A7682E (4G)     | RST             | IO09 |
| PCM5102A        | I2S_BCLK        | IO07 |
| PCM5102A        | I2S_DOUT        | IO08 |
| PCM5102A        | I2S_LRC         | IO09 |
| E-Paper         | EPD_SCK         | IO36 |
| E-Paper         | EPD_MOSI        | IO33 |
| E-Paper         | EPD_DC          | IO35 |
| E-Paper         | EPD_CS          | IO34 |
| E-Paper         | EPD_BUSY        | IO37 |
| Touch           | TOUCH_SCL       | IO13 |
| Touch           | TOUCH_SDA       | IO14 |
| Touch           | TOUCH_INT       | IO12 |
| Touch           | TOUCH_RST       | IO45 |
| Vibration Motor | MOTOR_PIN       | IO02 |
| LoRa            | LORA_SCK        | IO36 |
| LoRa            | LORA_MOSI       | IO33 |
| LoRa            | LORA_MISO       | IO47 |
| LoRa            | LORA_CS         | IO03 |
| LoRa            | LORA_BUSY       | IO06 |
| LoRa            | LORA_RST        | IO04 |
| LoRa            | LORA_INT        | IO05 |
| SD Card         | SD_CS           | IO48 |
| SD Card         | SD_SCK          | IO36 |
| SD Card         | SD_MOSI         | IO33 |
| SD Card         | SD_MISO         | IO47 |
| Gyroscope       | Gyroscope_INT   | IO21 |
| Microphone      | MIC_DATA        | IO17 |
| Microphone      | MIC_CLOCK       | IO18 |
| GPS             | GPS_RXD         | IO44 |
| GPS             | GPS_TXD         | IO43 |
| GPS             | GPS_PPS         | IO01 |
| Keyboard        | KEYBOARD_SCL    | IO13 |
| Keyboard        | KEYBOARD_SDA    | IO14 |
| Keyboard        | KEYBOARD_INT    | IO15 |
| Keyboard        | KEYBOARD_LED    | IO42 |

## E-Ink Display Driver

The 3.1-inch E-Paper display (GDEQ031T10) uses a **UC8253** controller.

The recommended Rust crate for driving this display is `epd-waveshare`. This crate supports a variety of e-paper displays from the same manufacturer and is compatible with `embedded-hal`.

*   **Crate**: [epd-waveshare on crates.io](https://crates.io/crates/epd-waveshare)
*   **Repository**: [epd-waveshare on GitHub](https://github.com/caemor/epd-waveshare)

Integration will require connecting the display to the ESP32-S3's SPI interface and using the `epd-waveshare` crate to send commands and display data.

## Project Relevance

The project `t-deck-async-app` was generated using `esp-generate` with the `--chip esp32s3` flag, which correctly matches the device's microcontroller.
