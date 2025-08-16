//! Device status and statistics structures.

/// Represents the status of the device.
#[derive(Copy, Clone)]
pub struct Status {
    inner: u8,
}

impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("chip_mode", &self.chip_mode())
            .field("command_status", &self.command_status())
            .finish()
    }
}

/// The operating mode of the chip.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChipMode {
    /// Standby with RC 13MHz oscillator.
    StbyRC = 0x02,
    /// Standby with XOSC (crystal oscillator).
    StbyXOSC = 0x03,
    /// Frequency Synthesis.
    FS = 0x04,
    /// Receive mode.
    RX = 0x05,
    /// Transmit mode.
    TX = 0x06,
}

/// The status of the last command executed.
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum CommandStatus {
    /// Data is available to be read.
    DataAvailable = 0x02,
    /// The command timed out.
    CommandTimeout = 0x03,
    /// The command resulted in a processing error.
    CommandProcessingError = 0x04,
    /// The command failed to execute.
    FailureToExecute = 0x05,
    /// The command completed successfully (e.g., TX done).
    CommandTxDone = 0x06,
}

impl From<u8> for Status {
    fn from(b: u8) -> Self {
        Self { inner: b }
    }
}

impl Status {
    /// Returns the current chip mode.
    pub fn chip_mode(&self) -> Option<ChipMode> {
        use ChipMode::*;
        match (self.inner & 0x70) >> 4 {
            0x02 => Some(StbyRC),
            0x03 => Some(StbyXOSC),
            0x04 => Some(FS),
            0x05 => Some(RX),
            0x06 => Some(TX),
            _ => None,
        }
    }

    /// Returns the status of the last command.
    pub fn command_status(self) -> Option<CommandStatus> {
        use CommandStatus::*;
        match (self.inner & 0x0E) >> 1 {
            0x02 => Some(DataAvailable),
            0x03 => Some(CommandTimeout),
            0x04 => Some(CommandProcessingError),
            0x05 => Some(FailureToExecute),
            0x06 => Some(CommandTxDone),
            _ => None,
        }
    }
}

/// Statistics about the device's operation.
#[derive(Copy, Clone, Debug)]
pub struct Stats {
    /// The device status.
    pub status: Status,
    /// The number of packets received.
    pub rx_pkt: u16,
    /// The number of CRC errors encountered.
    pub crc_error: u16,
    /// The number of header errors encountered.
    pub header_error: u16,
}

impl From<[u8; 7]> for Stats {
    fn from(b: [u8; 7]) -> Self {
        Self {
            status: b[0].into(),
            rx_pkt: u16::from_be_bytes([b[1], b[2]]),
            crc_error: u16::from_be_bytes([b[3], b[4]]),
            header_error: u16::from_be_bytes([b[5], b[6]]),
        }
    }
}

/// The status of a received packet.
#[derive(Copy, Clone, Debug)]
pub struct PacketStatus {
    rssi_pkt: u8,
    snr_pkt: i8,
    signal_rssi_pkt: u8,
}

impl From<[u8; 3]> for PacketStatus {
    fn from(b: [u8; 3]) -> Self {
        Self {
            rssi_pkt: b[0],
            snr_pkt: i8::from_be_bytes([b[1]]),
            signal_rssi_pkt: b[2],
        }
    }
}

impl PacketStatus {
    /// Returns the RSSI (Received Signal Strength Indicator) of the packet in dBm.
    pub fn rssi_pkt(&self) -> f32 {
        self.rssi_pkt as f32 / -2.0
    }

    /// Returns the SNR (Signal-to-Noise Ratio) of the packet in dB.
    pub fn snr_pkt(&self) -> f32 {
        self.snr_pkt as f32 / 4.0
    }

    /// Returns the signal RSSI of the packet in dBm.
    pub fn signal_rssi_pkt(&self) -> f32 {
        self.signal_rssi_pkt as f32 / -2.0
    }
}
