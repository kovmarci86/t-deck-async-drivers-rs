//! The core LoRa radio driver implementation.

use embassy_time::{Duration, Timer};
use embedded_hal_async::spi::SpiDevice;
use esp_hal::gpio::{Input, Output};
use sx126x_async::op::*;
use sx126x_async::SX126x as Device;

/// Configuration for the LoRa radio.
#[derive(Debug, Clone)]
pub struct LoraConfig {
    /// Spreading factor
    pub spreading_factor: LoRaSpreadFactor,
    /// Bandwidth
    pub bandwidth: LoRaBandWidth,
    /// Coding rate
    pub coding_rate: LoraCodingRate,
    /// PA Duty cycle
    pub pa_duty_cycle: u8,
    /// HP Max
    pub hp_max: u8,
    /// Sync word
    pub sync_word: u16,
}

impl Default for LoraConfig {
    fn default() -> Self {
        Self {
            spreading_factor: LoRaSpreadFactor::SF10,
            bandwidth: LoRaBandWidth::BW125,
            coding_rate: LoraCodingRate::CR4_6,
            pa_duty_cycle: 0x04,
            hp_max: 0x07,
            sync_word: 0x1424, // private network
        }
    }
}

type Error = ();

/// A high-level interface for the SX1262 LoRa radio.
///
/// This struct encapsulates the `sx126x_async` device and provides methods
/// for initialization, sending, and receiving data.
pub struct LoraRadio<'a, SPI>
where
    SPI: SpiDevice,
{
    /// The underlying `sx126x_async` device instance.
    pub device: sx126x_async::SX126x<SPI, Output<'a>, Input<'a>, Output<'a>, Input<'a>>,
}

impl<'a, SPI> LoraRadio<'a, SPI>
where
    SPI: SpiDevice,
{
    /// Creates a new `LoraRadio`.
    ///
    /// # Arguments
    ///
    /// * `lora_spi` - The SPI device for communicating with the LoRa module.
    /// * `rst` - The reset output pin.
    /// * `dio1` - The DIO1 interrupt input pin.
    /// * `busy` - The busy indicator input pin.
    /// * `en` - The device enable output pin.
    pub fn new(
        lora_spi: SPI,
        rst: Output<'a>,
        dio1: Input<'a>,
        busy: Input<'a>,
        en: Output<'a>,
    ) -> Self {
        let device = Device::new(lora_spi, (rst, busy, en, dio1));
        Self { device }
    }

    /// Resets the LoRa module.
    pub async fn reset(&mut self) -> Result<(), Error> {
        self.device.reset().await.map_err(|err| {
            log::warn!("Error reseting device: {err:?}");
        })
    }

    /// Initializes the LoRa radio with a default configuration.
    ///
    /// This method sets up the radio with a hardcoded configuration for
    /// LoRa modulation, frequency, power, and other parameters.
    pub async fn init(&mut self, config: &LoraConfig) -> Result<(), Error> {
        let lora_modparam = LoraModParams::default()
            .set_spread_factor(config.spreading_factor)
            .set_bandwidth(config.bandwidth)
            .set_coding_rate(config.coding_rate);

        let pa_config = PaConfig::default()
            .set_pa_duty_cycle(config.pa_duty_cycle)
            .set_hp_max(config.hp_max)
            .set_device_sel(DeviceSel::SX1262);

        let dio1_irq_mask = IrqMask::none()
            .combine(IrqMaskBit::TxDone)
            .combine(IrqMaskBit::RxDone)
            .combine(IrqMaskBit::Timeout);

        let conf = sx126x_async::conf::Config {
            packet_type: PacketType::LoRa,
            sync_word: config.sync_word,
            calib_param: CalibParam::all(),
            mod_params: lora_modparam.into(),
            pa_config,
            packet_params: Some(
                LoRaPacketParams {
                    preamble_len: 15,
                    header_type: LoRaHeaderType::VarLen,
                    payload_len: 0xFF,
                    crc_type: LoRaCrcType::CrcOff,
                    invert_iq: LoRaInvertIq::Standard,
                }
                .into(),
            ),
            tx_params: TxParams::default()
                .set_power_dbm(22i8)
                .set_ramp_time(RampTime::Ramp200u),
            dio1_irq_mask,
            dio2_irq_mask: IrqMask::none(),
            dio3_irq_mask: IrqMask::none(),
            rf_freq: 868_000_000,
            rf_frequency: 868_000_000,
            tcxo_opts: Some((TcxoVoltage::Volt2_4, TcxoDelay::from_ms(5))),
        };
        log::trace!(
            "lora::init busy: {}, dio1: {}",
            self.device.is_busy(),
            self.device.is_dio1_high()
        );
        let init_result = self.device.init(conf).await.map_err(|err| {
            log::warn!("Error reseting device: {err:?}");
        });
        log::trace!(
            "lora::init done. busy: {}, dio1: {}",
            self.device.is_busy(),
            self.device.is_dio1_high()
        );

        log::debug!("LoRa radio status: {:?}", self.device.get_status().await);

        self.device.set_ocp(140).await.map_err(|err| {
            log::warn!("Error setting ocp: {err:?}");
        })?;

        init_result
    }

    /// Sends a data packet using the LoRa radio.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice containing the data to send.
    pub async fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        self.device
            .write_bytes(data, RxTxTimeout::from_ms(2000), 15, LoRaCrcType::CrcOff)
            .await
            .map_err(|err| {
                log::warn!("Error sending lora message: {err:?}");
            })?;
        Ok(())
    }

    /// Waits to receive a data packet from the LoRa radio.
    ///
    /// This function puts the radio into receive mode and waits for the DIO1
    /// interrupt pin to signal a received packet. It handles timeouts and
    /// other interrupt flags.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable byte slice to store the received data.
    ///
    /// # Returns
    ///
    /// The number of bytes received, or an error.
    pub async fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        log::trace!("lora::receive waiting for message");
        self.device
            .clear_irq_status(IrqMask::all())
            .await
            .map_err(|_| ())?;
        self.device
            .set_rx(RxTxTimeout::from_ms(5000))
            .await
            .map_err(|err| {
                log::warn!("Error setting rx mode: {err:?}");
            })?;
        log::trace!(
            "lora::receive in rx mode. busy: {}, dio1: {}",
            self.device.is_busy(),
            self.device.is_dio1_high()
        );

        self.device.wait_on_dio1().await.map_err(|err| {
            log::warn!("Error waiting for dio1: {err:?}");
        })?;
        log::trace!(
            "lora::receive DIO1 went high. busy: {}, dio1: {}",
            self.device.is_busy(),
            self.device.is_dio1_high()
        );

        // Add a small delay to allow the chip to settle
        Timer::after(Duration::from_millis(1)).await;
        log::trace!(
            "lora::receive post-delay. busy: {}, dio1: {}",
            self.device.is_busy(),
            self.device.is_dio1_high()
        );

        let irq_status = self.device.get_irq_status().await.map_err(|err| {
            log::warn!("Error getting irq status: {err:?}");
        })?;
        log::trace!("lora::receive irq status: {irq_status:?}");

        // Check for errors first
        if irq_status.timeout() {
            log::trace!("lora::receive timeout");
            self.device
                .clear_irq_status(IrqMask::all())
                .await
                .map_err(|_| ())?;
            return Ok(0); // Return 0 bytes on timeout
        }

        if !irq_status.rx_done() {
            log::warn!("lora::receive unexpected interrupt: {irq_status:?}");
            self.device
                .clear_irq_status(IrqMask::all())
                .await
                .map_err(|_| ())?;
            return Err(());
        }

        let rx_status = self.device.get_rx_buffer_status().await.map_err(|err| {
            log::warn!("Error getting rx buffer status: {err:?}");
        })?;
        let len = rx_status.payload_length_rx() as usize;
        let offset = rx_status.rx_start_buffer_pointer();
        log::trace!("lora::receive rx status: len={len}, offset={offset}");

        if len > buffer.len() {
            log::warn!("lora::receive received payload larger than buffer");
            self.device
                .clear_irq_status(IrqMask::all())
                .await
                .map_err(|_| ())?;
            return Err(());
        }

        self.device
            .read_buffer(offset, &mut buffer[..len])
            .await
            .map_err(|err| {
                log::warn!("Error reading buffer: {err:?}");
            })?;
        log::trace!("lora::receive buffer read");

        self.device
            .clear_irq_status(IrqMask::all())
            .await
            .map_err(|err| {
                log::warn!("Error clearing irq status: {err:?}");
            })?;
        log::trace!("lora::receive IRQ cleared");

        Ok(len)
    }
}
