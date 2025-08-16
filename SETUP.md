# Development Environment Setup

This document outlines the steps required to set up a development environment for building and flashing Rust applications on our target device. See [device.md](./device.md) for hardware details.

## 1. Prerequisites

Ensure you have `rustup` installed. If not, follow the instructions at [rustup.rs](https://rustup.rs/).

## 2. Configure Your Shell

The `rustup` and `espup` installers add necessary directories to your `PATH`. To make these changes available in your current and future shell sessions, you need to source the appropriate environment file.

For `bash`:

```bash
echo 'source "$HOME/.cargo/env"' >> ~/.bash_profile
source ~/.bash_profile
```

For `zsh`:

```bash
echo 'source "$HOME/.cargo/env"' >> ~/.zshrc
source ~/.zshrc
```

## 3. Install Required Tools

Several tools are required for generating, building, and flashing the application.

### esp-generate

This tool is used to generate new Espressif projects.

```bash
cargo install esp-generate
```

### espup

This tool manages the Espressif Rust toolchain.

```bash
cargo install espup
espup install
```

### espflash

This tool is used to flash the application to the device.

```bash
cargo install espflash
```

### probe-rs

This tool is used for debugging and flashing.

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

## 4. Generate the Project

To generate a new project, use the `esp-generate` command. The following command creates a project for the ESP32-S3 with async support using Embassy and WiFi.

```bash
esp-generate --headless --chip esp32s3 --option embassy --option wifi --option unstable-hal --option alloc t-deck-async-app
```

## 5. Set Up the Build Environment

The `espup` installer creates a script at `~/export-esp.sh` that must be sourced to set up the required environment variables for building.

To have this sourced automatically for every new terminal session, add it to your shell's profile file.

For `bash`:
```bash
echo 'source ~/export-esp.sh' >> ~/.bash_profile
```

After running this, open a new terminal for the change to take effect. For the current session, you can run:
```bash
source ~/export-esp.sh
```

## 6. Flashing the Device

To flash the application to the device, navigate to the project directory and run the following command:

```bash
cargo flash
```

You may need to hold down the BOOT button on the device during the flashing process to establish a connection.

## 7. Monitoring Serial Output

To monitor the serial output from the device, you can use the `read_serial.sh` script. This script reads data from the serial port for 10 seconds.

```bash
./read_serial.sh
```