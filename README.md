# T-Deck Async Rust Drivers

This repository contains a collection of asynchronous Rust drivers for the LilyGo T-Deck device, built on the Embassy framework. Each driver is in its own crate within the workspace.

## Crates

*   `t-deck-pro-battery-async`: Driver for the BQ25896 battery charger IC.
*   `t-deck-pro-epd-async`: Driver for the e-paper display.
*   `t-deck-pro-gps-async`: Driver for the GPS module.
*   `t-deck-pro-keyboard-async`: Driver for the keyboard controller.
*   `t-deck-pro-lora-async`: Driver for the LoRa module.
*   `t-deck-pro-touch-async`: Driver for the touch controller.
*   `sx126x-async-rs`: A low-level async driver for the SX126x LoRa transceiver.

## Contributions

Contributions are welcome! These drivers implement the most essential features to be functional, but there is much room for improvement and expansion.

A key goal is to generalize these libraries beyond the T-Deck and test them on a wider range of devices. If you have other hardware and are interested in helping, your contributions would be highly valuable.

Feel free to open an issue or submit a pull request.
