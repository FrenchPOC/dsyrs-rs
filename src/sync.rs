//! Synchronous client for DSY-RS servo drive controller
//!
//! This module provides a blocking wrapper around the async client,
//! using a dedicated tokio runtime for synchronous operation.

use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::Mutex;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

use crate::client::DsyrsClient;
use crate::types::*;

/// Synchronous DSY-RS servo drive controller client
/// 
/// This client wraps the async DsyrsClient with a dedicated tokio runtime
/// to provide a blocking API. Useful for simple applications or when
/// async is not needed.
/// 
/// # Example
/// ```no_run
/// use dsyrs::{DsyrsSyncClient, ServoConfig, ControlMode};
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = ServoConfig::new(1)
///         .with_control_mode(ControlMode::Position);
///     let mut servo = DsyrsSyncClient::connect("/dev/ttyUSB0", 115200, config)?;
///     servo.init()?;
///     
///     let status = servo.get_status()?;
///     println!("Current speed: {} rpm", status.speed);
///     
///     Ok(())
/// }
/// ```
pub struct DsyrsSyncClient {
    client: Arc<Mutex<DsyrsClient>>,
    runtime: Runtime,
}

impl DsyrsSyncClient {
    /// Create a new synchronous client by connecting to a serial port
    pub fn connect(port: &str, baud_rate: u32, config: ServoConfig) -> Result<Self> {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| DsyrsError::IoError(e.to_string()))?;

        let client: DsyrsClient = runtime.block_on(async {
            let builder = tokio_serial::new(port, baud_rate)
                .timeout(Duration::from_millis(100));
            let port = SerialStream::open(&builder)
                .map_err(|e| DsyrsError::SerialError(e.to_string()))?;
            let ctx = rtu::attach_slave(port, Slave::from(config.slave_id));
            Ok::<DsyrsClient, DsyrsError>(DsyrsClient::new(ctx, config))
        })?;

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            runtime,
        })
    }

    /// Create from an existing async client and runtime
    pub fn from_async(client: DsyrsClient, runtime: Runtime) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            runtime,
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> ServoConfig {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            client.config().clone()
        })
    }

    /// Get the slave ID
    pub fn slave_id(&self) -> u8 {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            client.slave_id()
        })
    }

    /// Initialize the servo drive with configured parameters
    pub fn init(&mut self) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.init().await
        })
    }

    // ========================================================================
    // LOW-LEVEL MODBUS OPERATIONS
    // ========================================================================

    /// Write a single holding register
    pub fn write_register(&mut self, addr: u16, value: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.write_register(addr, value).await
        })
    }

    /// Write multiple holding registers
    pub fn write_registers(&mut self, addr: u16, values: &[u16]) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.write_registers(addr, values).await
        })
    }

    /// Read holding registers
    pub fn read_registers(&mut self, addr: u16, count: u16) -> Result<Vec<u16>> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.read_registers(addr, count).await
        })
    }

    /// Write a 32-bit value as two consecutive registers
    pub fn write_u32(&mut self, addr: u16, value: u32) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.write_u32(addr, value).await
        })
    }

    /// Write a signed 32-bit value as two consecutive registers
    pub fn write_i32(&mut self, addr: u16, value: i32) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.write_i32(addr, value).await
        })
    }

    /// Read a 32-bit value from two consecutive registers
    pub fn read_u32(&mut self, addr: u16) -> Result<u32> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.read_u32(addr).await
        })
    }

    /// Read a signed 32-bit value from two consecutive registers
    pub fn read_i32(&mut self, addr: u16) -> Result<i32> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.read_i32(addr).await
        })
    }

    // ========================================================================
    // P00 - BASIC CONTROL OPERATIONS
    // ========================================================================

    /// Set control mode (P00.00)
    pub fn set_control_mode(&mut self, mode: ControlMode) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_control_mode(mode).await
        })
    }

    /// Get control mode (P00.00)
    pub fn get_control_mode(&mut self) -> Result<ControlMode> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_control_mode().await
        })
    }

    /// Set direction (P00.01)
    pub fn set_direction(&mut self, direction: Direction) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_direction(direction).await
        })
    }

    /// Set rigidity level (P00.04, 0-31)
    pub fn set_rigidity(&mut self, level: u8) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_rigidity(level).await
        })
    }

    /// Set inertia ratio (P00.05, 0-3000, unit: 0.01)
    pub fn set_inertia_ratio(&mut self, ratio: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_inertia_ratio(ratio).await
        })
    }

    /// Set maximum speed (P00.07, 0-10000 rpm)
    pub fn set_max_speed(&mut self, rpm: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_max_speed(rpm).await
        })
    }

    /// Set brake ON delay (P00.14, 0-10000 ms)
    pub fn set_brake_on_delay(&mut self, ms: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_brake_on_delay(ms).await
        })
    }

    /// Set brake OFF delay (P00.15, 10-10000 ms)
    pub fn set_brake_off_delay(&mut self, ms: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_brake_off_delay(ms).await
        })
    }

    // ========================================================================
    // P01 - SERVO MOTOR PARAMETERS
    // ========================================================================

    /// Set rated current (P01.04, unit: 0.01 A)
    pub fn set_rated_current(&mut self, current: f32) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_rated_current(current).await
        })
    }

    /// Set rated torque (P01.05, unit: 0.01 Nm)
    pub fn set_rated_torque(&mut self, torque: f32) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_rated_torque(torque).await
        })
    }

    /// Set pole pairs (P01.10, 1-50)
    pub fn set_pole_pairs(&mut self, pairs: u8) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_pole_pairs(pairs).await
        })
    }

    /// Set encoder type (P01.18)
    pub fn set_encoder_type(&mut self, encoder: EncoderType) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_encoder_type(encoder).await
        })
    }

    // ========================================================================
    // P02 - DIGITAL I/O CONFIGURATION
    // ========================================================================

    /// Configure digital input function (DI1-DI3)
    pub fn set_di_function(&mut self, input: u8, function: DiFunction) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_di_function(input, function).await
        })
    }

    /// Configure digital input logic (DI1-DI3)
    pub fn set_di_logic(&mut self, input: u8, logic: DiLogic) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_di_logic(input, logic).await
        })
    }

    /// Configure digital output function (DO1-DO2)
    pub fn set_do_function(&mut self, output: u8, function: DoFunction) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_do_function(output, function).await
        })
    }

    /// Configure digital output logic (DO1-DO2)
    pub fn set_do_logic(&mut self, output: u8, logic: DoLogic) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_do_logic(output, logic).await
        })
    }

    // ========================================================================
    // P04 - POSITION CONTROL
    // ========================================================================

    /// Set position command source (P04.00)
    pub fn set_position_cmd_source(&mut self, source: PositionCmdSource) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_position_cmd_source(source).await
        })
    }

    /// Set step amount (P04.02, -9999 to 9999)
    pub fn set_step_amount(&mut self, amount: i16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_step_amount(amount).await
        })
    }

    /// Set electronic gear ratio (P04.07/P04.09)
    pub fn set_gear_ratio(&mut self, numerator: u32, denominator: u32) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_gear_ratio(numerator, denominator).await
        })
    }

    /// Set pulse shape (P04.21)
    pub fn set_pulse_shape(&mut self, shape: PulseShape) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_pulse_shape(shape).await
        })
    }

    /// Set positioning completion range (P04.24, 1-65535 pulses)
    pub fn set_positioning_range(&mut self, pulses: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_positioning_range(pulses).await
        })
    }

    // ========================================================================
    // P05 - SPEED CONTROL
    // ========================================================================

    /// Set speed command (P05.03, -9000 to 9000 rpm)
    pub fn set_speed_command(&mut self, rpm: i16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_speed_command(rpm).await
        })
    }

    /// Set jog speed (P05.04, 0-9000 rpm)
    pub fn set_jog_speed(&mut self, rpm: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_jog_speed(rpm).await
        })
    }

    /// Set acceleration time (P05.05, 0-10000 ms)
    pub fn set_accel_time(&mut self, ms: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_accel_time(ms).await
        })
    }

    /// Set deceleration time (P05.06, 0-10000 ms)
    pub fn set_decel_time(&mut self, ms: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_decel_time(ms).await
        })
    }

    /// Set forward speed limit (P05.08, 0-9000 rpm)
    pub fn set_forward_speed_limit(&mut self, rpm: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_forward_speed_limit(rpm).await
        })
    }

    /// Set backward speed limit (P05.09, 0-9000 rpm)
    pub fn set_backward_speed_limit(&mut self, rpm: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_backward_speed_limit(rpm).await
        })
    }

    /// Apply jog configuration
    pub fn apply_jog_config(&mut self, config: &JogConfig) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.apply_jog_config(config).await
        })
    }

    // ========================================================================
    // P06 - TORQUE CONTROL
    // ========================================================================

    /// Set torque command (P06.05, -3000 to 3000, unit: 0.1% of rated)
    pub fn set_torque_command(&mut self, torque: i16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_torque_command(torque).await
        })
    }

    /// Set forward torque limit (P06.08, 0-5000, unit: 0.1%)
    pub fn set_forward_torque_limit(&mut self, limit: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_forward_torque_limit(limit).await
        })
    }

    /// Set backward torque limit (P06.09, 0-5000, unit: 0.1%)
    pub fn set_backward_torque_limit(&mut self, limit: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_backward_torque_limit(limit).await
        })
    }

    // ========================================================================
    // P07 - GAIN PARAMETERS
    // ========================================================================

    /// Set position loop gain 1 (P07.00, 10-20000, unit: 0.1 Hz)
    pub fn set_position_gain(&mut self, gain: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_position_gain(gain).await
        })
    }

    /// Set speed loop gain 1 (P07.01, 10-20000, unit: 0.1 Hz)
    pub fn set_speed_gain(&mut self, gain: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_speed_gain(gain).await
        })
    }

    /// Set speed loop integral time 1 (P07.02, 15-512, unit: 0.01 ms)
    pub fn set_speed_integral(&mut self, time: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_speed_integral(time).await
        })
    }

    /// Apply gain parameters
    pub fn apply_gain_params(&mut self, params: &GainParams) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.apply_gain_params(params).await
        })
    }

    // ========================================================================
    // P10 - COMMUNICATION PARAMETERS
    // ========================================================================

    /// Set communication address (P10.00, 0-247)
    pub fn set_comm_address(&mut self, address: u8) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_comm_address(address).await
        })
    }

    /// Set Modbus baud rate (P10.02)
    pub fn set_baud_rate(&mut self, baud: BaudRate) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_baud_rate(baud).await
        })
    }

    /// Set Modbus data format (P10.03)
    pub fn set_data_format(&mut self, format: DataFormat) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_data_format(format).await
        })
    }

    /// Save parameters to EEPROM (P10.04)
    pub fn save_to_eeprom(&mut self) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.save_to_eeprom().await
        })
    }

    /// Apply communication configuration
    pub fn apply_comm_config(&mut self, config: &CommConfig) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.apply_comm_config(config).await
        })
    }

    // ========================================================================
    // P11 - AUXILIARY FUNCTIONS
    // ========================================================================

    /// Reset fault (P11.01)
    pub fn reset_fault(&mut self) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.reset_fault().await
        })
    }

    /// Soft reset (P11.02)
    pub fn soft_reset(&mut self) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.soft_reset().await
        })
    }

    /// Factory reset (P11.09)
    pub fn factory_reset(&mut self) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.factory_reset().await
        })
    }

    /// Clear fault record (P11.09)
    pub fn clear_fault_record(&mut self) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.clear_fault_record().await
        })
    }

    /// Reset absolute encoder (P11.06)
    pub fn reset_encoder(&mut self, reset: EncoderReset) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.reset_encoder(reset).await
        })
    }

    /// Emergency stop (P11.13)
    pub fn emergency_stop(&mut self) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.emergency_stop().await
        })
    }

    /// Clear emergency stop (P11.13)
    pub fn clear_emergency_stop(&mut self) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.clear_emergency_stop().await
        })
    }

    // ========================================================================
    // P13 - MULTI-SEGMENT POSITION
    // ========================================================================

    /// Set multi-segment operation mode (P13.00)
    pub fn set_multi_seg_mode(&mut self, mode: MultiSegOperationMode) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_multi_seg_mode(mode).await
        })
    }

    /// Set multi-segment start segment (P13.01, 1-16)
    pub fn set_multi_seg_start(&mut self, segment: u8) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_multi_seg_start(segment).await
        })
    }

    /// Set multi-segment end segment (P13.02, 1-16)
    pub fn set_multi_seg_end(&mut self, segment: u8) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_multi_seg_end(segment).await
        })
    }

    /// Set multi-segment position mode (P13.05)
    pub fn set_multi_seg_position_mode(&mut self, mode: MultiSegPositionMode) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_multi_seg_position_mode(mode).await
        })
    }

    /// Configure a segment
    pub fn configure_segment(&mut self, config: &SegmentConfig) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.configure_segment(config).await
        })
    }

    // ========================================================================
    // P16 - SPECIAL FUNCTIONS (HOMING)
    // ========================================================================

    /// Set homing mode (P16.09)
    pub fn set_homing_mode(&mut self, mode: HomingMode) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_homing_mode(mode).await
        })
    }

    /// Set homing high speed (P16.10, 10-3000 rpm)
    pub fn set_homing_high_speed(&mut self, rpm: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_homing_high_speed(rpm).await
        })
    }

    /// Set homing low speed (P16.11, 10-1000 rpm)
    pub fn set_homing_low_speed(&mut self, rpm: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_homing_low_speed(rpm).await
        })
    }

    /// Set homing acceleration limit (P16.12, 0-65535 ms)
    pub fn set_homing_accel(&mut self, ms: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_homing_accel(ms).await
        })
    }

    /// Set homing timeout (P16.13, 0-65535 ms)
    pub fn set_homing_timeout(&mut self, ms: u16) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_homing_timeout(ms).await
        })
    }

    /// Set home offset (P16.14)
    pub fn set_home_offset(&mut self, offset: i32) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.set_home_offset(offset).await
        })
    }

    /// Apply homing configuration
    pub fn apply_homing_config(&mut self, config: &HomingConfig) -> Result<()> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.apply_homing_config(config).await
        })
    }

    // ========================================================================
    // P18 - STATUS MONITORING (READ-ONLY)
    // ========================================================================

    /// Get servo status (P18.00)
    pub fn get_servo_state(&mut self) -> Result<ServoState> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_servo_state().await
        })
    }

    /// Get motor speed feedback (P18.01, rpm)
    pub fn get_speed(&mut self) -> Result<i16> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_speed().await
        })
    }

    /// Get average load rate (P18.02, unit: 0.1%)
    pub fn get_load_rate(&mut self) -> Result<f32> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_load_rate().await
        })
    }

    /// Get speed command (P18.03, rpm)
    pub fn get_speed_command(&mut self) -> Result<i16> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_speed_command().await
        })
    }

    /// Get internal torque (P18.04, unit: 0.1% of rated)
    pub fn get_torque(&mut self) -> Result<f32> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_torque().await
        })
    }

    /// Get phase current RMS (P18.05, unit: 0.01 A)
    pub fn get_current(&mut self) -> Result<f32> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_current().await
        })
    }

    /// Get DC bus voltage (P18.06, unit: 0.1 V)
    pub fn get_bus_voltage(&mut self) -> Result<f32> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_bus_voltage().await
        })
    }

    /// Get absolute position (P18.07)
    pub fn get_position(&mut self) -> Result<i32> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_position().await
        })
    }

    /// Get electrical angle (P18.09, unit: 0.1°)
    pub fn get_electrical_angle(&mut self) -> Result<f32> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_electrical_angle().await
        })
    }

    /// Get complete servo status
    pub fn get_status(&mut self) -> Result<ServoStatus> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_status().await
        })
    }

    // ========================================================================
    // VERSION INFORMATION
    // ========================================================================

    /// Get software version (P12.12)
    pub fn get_software_version(&mut self) -> Result<u16> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_software_version().await
        })
    }

    /// Get FPGA version (P12.13)
    pub fn get_fpga_version(&mut self) -> Result<u16> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_fpga_version().await
        })
    }

    /// Get product series code (P12.14)
    pub fn get_product_code(&mut self) -> Result<u16> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            client.get_product_code().await
        })
    }
}
