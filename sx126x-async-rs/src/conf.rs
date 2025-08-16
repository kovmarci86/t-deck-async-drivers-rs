//! Wrapper for modem configuration parameters.

use super::op::*;

/// Configuration parameters used to initialize the SX126x modem.
pub struct Config {
    /// The packet type to be used (LoRa or GFSK).
    pub packet_type: PacketType,
    /// The LoRa sync word. Use `0x3444` for public networks (like TTN) and `0x1424` for private networks.
    pub sync_word: u16,
    /// Calibration parameters for the device.
    pub calib_param: CalibParam,
    /// Modulation parameters (e.g., spreading factor, bandwidth).
    pub mod_params: ModParams,
    /// Power-amplifier configuration.
    pub pa_config: PaConfig,
    /// Packet parameters (e.g., preamble length, header type).
    /// Set to `None` to configure these later.
    pub packet_params: Option<PacketParams>,
    /// TX power and ramp time configuration.
    pub tx_params: TxParams,
    /// Interrupt mask for the DIO1 pin.
    pub dio1_irq_mask: IrqMask,
    /// Interrupt mask for the DIO2 pin.
    pub dio2_irq_mask: IrqMask,
    /// Interrupt mask for the DIO3 pin.
    pub dio3_irq_mask: IrqMask,
    /// The RF frequency value to be set in the register, calculated using `calc_rf_freq`.
    pub rf_freq: u32,
    /// The desired RF frequency in Hz (e.g., `868_000_000` for 868 MHz).
    pub rf_frequency: u32,
    /// TCXO (Temperature-Compensated Crystal Oscillator) options.
    /// Set to `None` if not using a TCXO.
    pub tcxo_opts: Option<(TcxoVoltage, TcxoDelay)>,
}
