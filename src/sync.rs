//! Synchronous client for DSY-RS servo drive controller
//!
//! This module provides a native synchronous Modbus RTU client,
//! compatible with em2rs library for shared bus operation.

use crate::registers;
use crate::types::*;
#[cfg(feature = "modbus-delay")]
use std::thread;
#[cfg(feature = "modbus-delay")]
use std::time::Duration;
use tokio_modbus::prelude::*;

/// Default delay after modbus requests (1ms)
#[cfg(feature = "modbus-delay")]
const MODBUS_DELAY: Duration = Duration::from_millis(1);

/// Synchronous DSY-RS servo drive controller client
///
/// This client uses tokio-modbus sync API for blocking Modbus RTU communication.
/// It is designed to be compatible with em2rs for shared bus operation.
///
/// # Example
/// ```no_run
/// use dsyrs::{DsyrsSyncClient, ServoConfig, ControlMode, Slave};
/// use tokio_modbus::prelude::client;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create serial port builder
///     let builder = tokio_serial::new("/dev/ttyUSB0", 115200);
///     
///     // Connect to slave ID 1
///     let ctx = client::sync::rtu::connect_slave(&builder, Slave::from(1))?;
///     
///     // Create servo configuration
///     let config = ServoConfig::new(1)
///         .with_control_mode(ControlMode::Position);
///     
///     // Create and initialize client
///     let mut servo = DsyrsSyncClient::new(ctx, config);
///     servo.init()?;
///     
///     // Read status
///     let status = servo.get_status()?;
///     println!("Current speed: {} rpm", status.speed);
///     
///     Ok(())
/// }
/// ```
pub struct DsyrsSyncClient {
    ctx: client::sync::Context,
    slave_id: u8,
    config: ServoConfig,
}

impl DsyrsSyncClient {
    /// Create a new synchronous DSY-RS client with an existing tokio-modbus sync context
    ///
    /// # Arguments
    /// * `ctx` - Tokio-modbus sync context (already initialized for RTU communication)
    /// * `config` - Servo configuration including slave ID
    ///
    /// # Example
    /// ```no_run
    /// use dsyrs::{DsyrsSyncClient, ServoConfig, Slave};
    /// use tokio_modbus::prelude::client;
    ///
    /// let builder = tokio_serial::new("/dev/ttyUSB0", 115200);
    /// let ctx = client::sync::rtu::connect_slave(&builder, Slave::from(1))?;
    /// let config = ServoConfig::new(1);
    /// let mut servo = DsyrsSyncClient::new(ctx, config);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(ctx: client::sync::Context, config: ServoConfig) -> Self {
        Self {
            ctx,
            slave_id: config.slave_id,
            config,
        }
    }

    /// Consume the client and return the underlying Modbus context
    ///
    /// This is useful when you want to reuse the same physical connection
    /// for multiple devices on the same RS485 bus (e.g., with em2rs stepper motors).
    ///
    /// # Example
    /// ```no_run
    /// use dsyrs::{DsyrsSyncClient, ServoConfig, Slave, SlaveContext};
    /// use tokio_modbus::prelude::client;
    ///
    /// // Use servo on slave 1
    /// let builder = tokio_serial::new("/dev/ttyUSB0", 115200);
    /// let ctx = client::sync::rtu::connect_slave(&builder, Slave::from(1))?;
    /// let mut servo = DsyrsSyncClient::new(ctx, ServoConfig::new(1));
    /// servo.init()?;
    ///
    /// // Extract context to use with em2rs stepper on slave 2
    /// let mut ctx = servo.into_context();
    /// ctx.set_slave(Slave::from(2));
    /// // Now use ctx with em2rs: Em2rsSyncClient::new(ctx, stepper_config)
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_context(self) -> client::sync::Context {
        self.ctx
    }

    /// Get a mutable reference to the Modbus context
    pub fn context_mut(&mut self) -> &mut client::sync::Context {
        &mut self.ctx
    }

    /// Get the current configuration
    pub fn config(&self) -> &ServoConfig {
        &self.config
    }

    /// Get the slave ID
    pub fn slave_id(&self) -> u8 {
        self.slave_id
    }

    /// Initialize the servo drive with configured parameters
    pub fn init(&mut self) -> Result<()> {
        self.ctx.set_slave(Slave::from(self.slave_id));

        // Set control mode (P00.00)
        self.write_register(registers::P00_CONTROL_MODE, self.config.control_mode.into())?;

        // Set direction (P00.01)
        self.write_register(registers::P00_DIRECTION, self.config.direction.into())?;

        // Set max speed (P00.07)
        self.write_register(registers::P00_MAX_SPEED, self.config.max_speed)?;

        // Read P01 parameters (all P01 parameters are not writable)
        // Read motor model code (P01.00)
        let motor_model = self.read_register(registers::P01_MOTOR_MODEL)?;
        if let Some(expected_model) = self.config.motor_model_code {
            if motor_model != expected_model {
                log::warn!(
                    "Motor model mismatch: expected {}, read {}",
                    expected_model,
                    motor_model
                );
            }
        }

        // Read rated current (P01.04) - unit is 0.01 A
        let rated_current_raw = self.read_register(registers::P01_RATED_CURRENT)?;
        let rated_current = rated_current_raw as f32 / 100.0;
        if let Some(expected_current) = self.config.rated_current {
            if (rated_current - expected_current).abs() > 0.01 {
                log::warn!(
                    "Rated current mismatch: expected {} A, read {} A",
                    expected_current,
                    rated_current
                );
            }
        }

        // Read encoder type (P01.18)
        let encoder_type_raw = self.read_register(registers::P01_ENCODER_SELECTION)?;
        if let Some(expected_encoder) = self.config.encoder_type {
            let expected_value: u16 = expected_encoder.into();
            if encoder_type_raw != expected_value {
                log::warn!(
                    "Encoder type mismatch: expected {:?}, read {}",
                    expected_encoder,
                    encoder_type_raw
                );
            }
        }

        // Read encoder resolution (P01.20) - stored as two 16-bit registers
        let resolution_regs = self.read_registers(registers::P01_ENCODER_RESOLUTION, 2)?;
        let encoder_resolution = ((resolution_regs[0] as u32) << 16) | (resolution_regs[1] as u32);
        if let Some(expected_resolution) = self.config.encoder_resolution {
            if encoder_resolution != expected_resolution {
                log::warn!(
                    "Encoder resolution mismatch: expected {}, read {}",
                    expected_resolution,
                    encoder_resolution
                );
            }
        }

        Ok(())
    }

    // ========================================================================
    // LOW-LEVEL MODBUS OPERATIONS
    // ========================================================================

    /// Write a single holding register
    pub fn write_register(&mut self, addr: u16, value: u16) -> Result<()> {
        self.ctx.write_single_register(addr, value)??;
        #[cfg(feature = "modbus-delay")]
        thread::sleep(MODBUS_DELAY);
        Ok(())
    }

    /// Write multiple holding registers
    pub fn write_registers(&mut self, addr: u16, values: &[u16]) -> Result<()> {
        self.ctx.write_multiple_registers(addr, values)??;
        #[cfg(feature = "modbus-delay")]
        thread::sleep(MODBUS_DELAY);
        Ok(())
    }

    /// Read holding registers
    pub fn read_registers(&mut self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let data = self.ctx.read_holding_registers(addr, count)??;
        #[cfg(feature = "modbus-delay")]
        thread::sleep(MODBUS_DELAY);
        Ok(data)
    }

    /// Read a single holding register
    pub fn read_register(&mut self, addr: u16) -> Result<u16> {
        let data = self.read_registers(addr, 1)?;
        Ok(data[0])
    }

    /// Write a 32-bit value as two consecutive registers
    pub fn write_u32(&mut self, addr: u16, value: u32) -> Result<()> {
        let high = (value >> 16) as u16;
        let low = (value & 0xFFFF) as u16;
        self.write_registers(addr, &[high, low])
    }

    /// Write a signed 32-bit value as two consecutive registers
    pub fn write_i32(&mut self, addr: u16, value: i32) -> Result<()> {
        self.write_u32(addr, value as u32)
    }

    /// Read a 32-bit value from two consecutive registers
    pub fn read_u32(&mut self, addr: u16) -> Result<u32> {
        let data = self.read_registers(addr, 2)?;
        Ok(((data[0] as u32) << 16) | (data[1] as u32))
    }

    /// Read a signed 32-bit value from two consecutive registers
    pub fn read_i32(&mut self, addr: u16) -> Result<i32> {
        Ok(self.read_u32(addr)? as i32)
    }

    // ========================================================================
    // P00 - BASIC CONTROL OPERATIONS
    // ========================================================================

    /// Set control mode (P00.00)
    pub fn set_control_mode(&mut self, mode: ControlMode) -> Result<()> {
        self.write_register(registers::P00_CONTROL_MODE, mode.into())
    }

    /// Get control mode (P00.00)
    pub fn get_control_mode(&mut self) -> Result<ControlMode> {
        let data = self.read_registers(registers::P00_CONTROL_MODE, 1)?;
        ControlMode::try_from(data[0])
    }

    /// Set direction (P00.01)
    pub fn set_direction(&mut self, direction: Direction) -> Result<()> {
        self.write_register(registers::P00_DIRECTION, direction.into())
    }

    /// Set rigidity level (P00.04, 0-31)
    pub fn set_rigidity(&mut self, level: u8) -> Result<()> {
        if level > 31 {
            return Err(DsyrsError::InvalidParameter("Rigidity must be 0-31".into()));
        }
        self.write_register(registers::P00_RIGIDITY, level as u16)
    }

    /// Set inertia ratio (P00.05, 0-3000, unit: 0.01)
    pub fn set_inertia_ratio(&mut self, ratio: u16) -> Result<()> {
        if ratio > 3000 {
            return Err(DsyrsError::InvalidParameter(
                "Inertia ratio must be 0-3000".into(),
            ));
        }
        self.write_register(registers::P00_INERTIA_RATIO, ratio)
    }

    /// Set maximum speed (P00.07, 0-10000 rpm)
    pub fn set_max_speed(&mut self, rpm: u16) -> Result<()> {
        if rpm > 10000 {
            return Err(DsyrsError::InvalidParameter(
                "Max speed must be 0-10000 rpm".into(),
            ));
        }
        self.write_register(registers::P00_MAX_SPEED, rpm)
    }

    /// Set brake ON delay (P00.14, 0-10000 ms)
    pub fn set_brake_on_delay(&mut self, ms: u16) -> Result<()> {
        self.write_register(registers::P00_BRAKE_ON_DELAY, ms)
    }

    /// Set brake OFF delay (P00.15, 10-10000 ms)
    pub fn set_brake_off_delay(&mut self, ms: u16) -> Result<()> {
        self.write_register(registers::P00_BRAKE_OFF_DELAY, ms)
    }

    // ========================================================================
    // P01 - SERVO MOTOR PARAMETERS
    // ========================================================================

    /// Set rated current (P01.04, unit: 0.01 A)
    pub fn set_rated_current(&mut self, current: f32) -> Result<()> {
        let value = (current * 100.0) as u16;
        self.write_register(registers::P01_RATED_CURRENT, value)
    }

    /// Set rated torque (P01.05, unit: 0.01 Nm)
    pub fn set_rated_torque(&mut self, torque: f32) -> Result<()> {
        let value = (torque * 100.0) as u16;
        self.write_register(registers::P01_RATED_TORQUE, value)
    }

    /// Set pole pairs (P01.10, 1-50)
    pub fn set_pole_pairs(&mut self, pairs: u8) -> Result<()> {
        if pairs < 1 || pairs > 50 {
            return Err(DsyrsError::InvalidParameter(
                "Pole pairs must be 1-50".into(),
            ));
        }
        self.write_register(registers::P01_POLE_PAIRS, pairs as u16)
    }

    /// Set encoder type (P01.18)
    pub fn set_encoder_type(&mut self, encoder: EncoderType) -> Result<()> {
        self.write_register(registers::P01_ENCODER_SELECTION, encoder.into())
    }

    // ========================================================================
    // P02 - DIGITAL I/O CONFIGURATION
    // ========================================================================

    /// Configure digital input function (DI1-DI3)
    pub fn set_di_function(&mut self, input: u8, function: DiFunction) -> Result<()> {
        let register = registers::get_di_function_register(input)
            .ok_or(DsyrsError::InvalidDigitalInput(input))?;
        self.write_register(register, function.into())
    }

    /// Configure digital input logic (DI1-DI3)
    pub fn set_di_logic(&mut self, input: u8, logic: DiLogic) -> Result<()> {
        let register = registers::get_di_logic_register(input)
            .ok_or(DsyrsError::InvalidDigitalInput(input))?;
        self.write_register(register, logic.into())
    }

    /// Configure digital output function (DO1-DO2)
    pub fn set_do_function(&mut self, output: u8, function: DoFunction) -> Result<()> {
        let register = registers::get_do_function_register(output)
            .ok_or(DsyrsError::InvalidDigitalOutput(output))?;
        self.write_register(register, function.into())
    }

    /// Configure digital output logic (DO1-DO2)
    pub fn set_do_logic(&mut self, output: u8, logic: DoLogic) -> Result<()> {
        let register = registers::get_do_logic_register(output)
            .ok_or(DsyrsError::InvalidDigitalOutput(output))?;
        self.write_register(register, logic.into())
    }

    // ========================================================================
    // P04 - POSITION CONTROL
    // ========================================================================

    /// Set position command source (P04.00)
    pub fn set_position_cmd_source(&mut self, source: PositionCmdSource) -> Result<()> {
        self.write_register(registers::P04_POSITION_CMD_SOURCE, source.into())
    }

    /// Set step amount (P04.02, -9999 to 9999)
    pub fn set_step_amount(&mut self, amount: i16) -> Result<()> {
        self.write_register(registers::P04_STEP_AMOUNT, amount as u16)
    }

    /// Set electronic gear ratio (P04.07/P04.09)
    pub fn set_gear_ratio(&mut self, numerator: u32, denominator: u32) -> Result<()> {
        self.write_u32(registers::P04_GEAR1_NUMERATOR, numerator)?;
        self.write_u32(registers::P04_GEAR1_DENOMINATOR, denominator)
    }

    /// Set pulse shape (P04.21)
    pub fn set_pulse_shape(&mut self, shape: PulseShape) -> Result<()> {
        self.write_register(registers::P04_PULSE_SHAPE, shape.into())
    }

    /// Set positioning completion range (P04.24, 1-65535 pulses)
    pub fn set_positioning_range(&mut self, pulses: u16) -> Result<()> {
        self.write_register(registers::P04_POSITIONING_RANGE, pulses)
    }

    // ========================================================================
    // P05 - SPEED CONTROL
    // ========================================================================

    /// Set speed command (P05.03, -9000 to 9000 rpm)
    pub fn set_speed_command(&mut self, rpm: i16) -> Result<()> {
        self.write_register(registers::P05_SPEED_COMMAND, rpm as u16)
    }

    /// Set jog speed (P05.04, 0-9000 rpm)
    pub fn set_jog_speed(&mut self, rpm: u16) -> Result<()> {
        if rpm > 9000 {
            return Err(DsyrsError::InvalidParameter(
                "Jog speed must be 0-9000 rpm".into(),
            ));
        }
        self.write_register(registers::P05_JOG_SPEED, rpm)
    }

    /// Set acceleration time (P05.05, 0-10000 ms)
    pub fn set_accel_time(&mut self, ms: u16) -> Result<()> {
        self.write_register(registers::P05_ACCEL_TIME, ms)
    }

    /// Set deceleration time (P05.06, 0-10000 ms)
    pub fn set_decel_time(&mut self, ms: u16) -> Result<()> {
        self.write_register(registers::P05_DECEL_TIME, ms)
    }

    /// Set forward speed limit (P05.08, 0-9000 rpm)
    pub fn set_forward_speed_limit(&mut self, rpm: u16) -> Result<()> {
        self.write_register(registers::P05_FORWARD_SPEED_LIMIT, rpm)
    }

    /// Set backward speed limit (P05.09, 0-9000 rpm)
    pub fn set_backward_speed_limit(&mut self, rpm: u16) -> Result<()> {
        self.write_register(registers::P05_BACKWARD_SPEED_LIMIT, rpm)
    }

    /// Apply jog configuration
    pub fn apply_jog_config(&mut self, config: &JogConfig) -> Result<()> {
        self.set_jog_speed(config.speed)?;
        self.set_accel_time(config.accel_time)?;
        self.set_decel_time(config.decel_time)
    }

    // ========================================================================
    // P06 - TORQUE CONTROL
    // ========================================================================

    /// Set torque command (P06.05, -3000 to 3000, unit: 0.1% of rated)
    pub fn set_torque_command(&mut self, torque: i16) -> Result<()> {
        self.write_register(registers::P06_TORQUE_COMMAND, torque as u16)
    }

    /// Set forward torque limit (P06.08, 0-5000, unit: 0.1%)
    pub fn set_forward_torque_limit(&mut self, limit: u16) -> Result<()> {
        self.write_register(registers::P06_FORWARD_TORQUE_LIMIT, limit)
    }

    /// Set backward torque limit (P06.09, 0-5000, unit: 0.1%)
    pub fn set_backward_torque_limit(&mut self, limit: u16) -> Result<()> {
        self.write_register(registers::P06_BACKWARD_TORQUE_LIMIT, limit)
    }

    // ========================================================================
    // P07 - GAIN PARAMETERS
    // ========================================================================

    /// Set position loop gain 1 (P07.00, 10-20000, unit: 0.1 Hz)
    pub fn set_position_gain(&mut self, gain: u16) -> Result<()> {
        self.write_register(registers::P07_POSITION_GAIN1, gain)
    }

    /// Set speed loop gain 1 (P07.01, 10-20000, unit: 0.1 Hz)
    pub fn set_speed_gain(&mut self, gain: u16) -> Result<()> {
        self.write_register(registers::P07_SPEED_GAIN1, gain)
    }

    /// Set speed loop integral time 1 (P07.02, 15-512, unit: 0.01 ms)
    pub fn set_speed_integral(&mut self, time: u16) -> Result<()> {
        self.write_register(registers::P07_SPEED_INTEGRAL1, time)
    }

    /// Apply gain parameters
    pub fn apply_gain_params(&mut self, params: &GainParams) -> Result<()> {
        self.set_position_gain(params.position_gain)?;
        self.set_speed_gain(params.speed_gain)?;
        self.set_speed_integral(params.speed_integral)?;
        self.write_register(registers::P07_SPEED_FILTER1, params.speed_filter)
    }

    // ========================================================================
    // P10 - COMMUNICATION PARAMETERS
    // ========================================================================

    /// Set communication address (P10.00, 0-247)
    pub fn set_comm_address(&mut self, address: u8) -> Result<()> {
        self.write_register(registers::P10_COMM_ADDRESS, address as u16)
    }

    /// Set Modbus baud rate (P10.02)
    pub fn set_baud_rate(&mut self, baud: BaudRate) -> Result<()> {
        self.write_register(registers::P10_MODBUS_BAUDRATE, baud.into())
    }

    /// Set Modbus data format (P10.03)
    pub fn set_data_format(&mut self, format: DataFormat) -> Result<()> {
        self.write_register(registers::P10_MODBUS_FORMAT, format.into())
    }

    /// Save parameters to EEPROM (P10.04)
    pub fn save_to_eeprom(&mut self) -> Result<()> {
        self.write_register(registers::P10_WRITE_EEPROM, 1)
    }

    /// Apply communication configuration
    pub fn apply_comm_config(&mut self, config: &CommConfig) -> Result<()> {
        self.set_comm_address(config.address)?;
        self.set_baud_rate(config.baud_rate)?;
        self.set_data_format(config.data_format)?;
        self.write_register(
            registers::P10_RS485_ADDRESS_SOURCE,
            config.address_source.into(),
        )
    }

    // ========================================================================
    // P11 - AUXILIARY FUNCTIONS
    // ========================================================================

    /// Reset fault (P11.01)
    pub fn reset_fault(&mut self) -> Result<()> {
        self.write_register(registers::P11_FAULT_RESET, 1)
    }

    /// Soft reset (P11.02)
    pub fn soft_reset(&mut self) -> Result<()> {
        self.write_register(registers::P11_SOFT_RESET, 1)
    }

    /// Factory reset (P11.09)
    pub fn factory_reset(&mut self) -> Result<()> {
        self.write_register(registers::P11_SYSTEM_INIT, SystemInit::FactoryReset.into())
    }

    /// Clear fault record (P11.09)
    pub fn clear_fault_record(&mut self) -> Result<()> {
        self.write_register(
            registers::P11_SYSTEM_INIT,
            SystemInit::ClearFaultRecord.into(),
        )
    }

    /// Reset absolute encoder (P11.06)
    pub fn reset_encoder(&mut self, reset: EncoderReset) -> Result<()> {
        self.write_register(registers::P11_ENCODER_RESET, reset.into())
    }

    /// Emergency stop (P11.13)
    pub fn emergency_stop(&mut self) -> Result<()> {
        self.write_register(registers::P11_EMERGENCY_STOP, 1)
    }

    /// Clear emergency stop (P11.13)
    pub fn clear_emergency_stop(&mut self) -> Result<()> {
        self.write_register(registers::P11_EMERGENCY_STOP, 0)
    }

    // ========================================================================
    // P13 - MULTI-SEGMENT POSITION
    // ========================================================================

    /// Set multi-segment operation mode (P13.00)
    pub fn set_multi_seg_mode(&mut self, mode: MultiSegOperationMode) -> Result<()> {
        self.write_register(registers::P13_OPERATION_MODE, mode.into())
    }

    /// Set multi-segment start segment (P13.01, 1-16)
    pub fn set_multi_seg_start(&mut self, segment: u8) -> Result<()> {
        if segment < 1 || segment > 16 {
            return Err(DsyrsError::InvalidSegment(segment));
        }
        self.write_register(registers::P13_START_SEGMENT, segment as u16)
    }

    /// Set multi-segment end segment (P13.02, 1-16)
    pub fn set_multi_seg_end(&mut self, segment: u8) -> Result<()> {
        if segment < 1 || segment > 16 {
            return Err(DsyrsError::InvalidSegment(segment));
        }
        self.write_register(registers::P13_END_SEGMENT, segment as u16)
    }

    /// Set multi-segment position mode (P13.05)
    pub fn set_multi_seg_position_mode(&mut self, mode: MultiSegPositionMode) -> Result<()> {
        self.write_register(registers::P13_POSITION_MODE, mode.into())
    }

    /// Configure a segment
    pub fn configure_segment(&mut self, config: &SegmentConfig) -> Result<()> {
        let disp_reg = registers::get_segment_displacement_register(config.segment)
            .ok_or(DsyrsError::InvalidSegment(config.segment))?;
        let speed_reg = registers::get_segment_speed_register(config.segment)
            .ok_or(DsyrsError::InvalidSegment(config.segment))?;
        let accel_reg = registers::get_segment_accel_decel_register(config.segment)
            .ok_or(DsyrsError::InvalidSegment(config.segment))?;
        let wait_reg = registers::get_segment_wait_time_register(config.segment)
            .ok_or(DsyrsError::InvalidSegment(config.segment))?;

        // Write displacement as 32-bit value
        self.write_i32(disp_reg, config.displacement)?;
        self.write_register(speed_reg, config.speed)?;
        self.write_register(accel_reg, config.accel_decel_time)?;
        self.write_register(wait_reg, config.wait_time)
    }

    // ========================================================================
    // P16 - SPECIAL FUNCTIONS (HOMING)
    // ========================================================================

    /// Set homing mode (P16.09)
    pub fn set_homing_mode(&mut self, mode: HomingMode) -> Result<()> {
        self.write_register(registers::P16_HOMING_MODE, mode.into())
    }

    /// Set homing high speed (P16.10, 10-3000 rpm)
    pub fn set_homing_high_speed(&mut self, rpm: u16) -> Result<()> {
        self.write_register(registers::P16_HOMING_HIGH_SPEED, rpm)
    }

    /// Set homing low speed (P16.11, 10-1000 rpm)
    pub fn set_homing_low_speed(&mut self, rpm: u16) -> Result<()> {
        self.write_register(registers::P16_HOMING_LOW_SPEED, rpm)
    }

    /// Set homing acceleration limit (P16.12, 0-65535 ms)
    pub fn set_homing_accel(&mut self, ms: u16) -> Result<()> {
        self.write_register(registers::P16_HOMING_ACCEL, ms)
    }

    /// Set homing timeout (P16.13, 0-65535 ms)
    pub fn set_homing_timeout(&mut self, ms: u16) -> Result<()> {
        self.write_register(registers::P16_HOMING_TIMEOUT, ms)
    }

    /// Set home offset (P16.14)
    pub fn set_home_offset(&mut self, offset: i32) -> Result<()> {
        self.write_i32(registers::P16_HOME_OFFSET, offset)
    }

    /// Apply homing configuration
    pub fn apply_homing_config(&mut self, config: &HomingConfig) -> Result<()> {
        self.set_homing_mode(config.mode)?;
        self.set_homing_high_speed(config.high_speed)?;
        self.set_homing_low_speed(config.low_speed)?;
        self.set_homing_accel(config.accel_limit)?;
        self.set_homing_timeout(config.timeout)?;
        self.set_home_offset(config.offset)
    }

    // ========================================================================
    // P18 - STATUS MONITORING (READ-ONLY)
    // ========================================================================

    /// Get servo status (P18.00)
    pub fn get_servo_state(&mut self) -> Result<ServoState> {
        let data = self.read_registers(registers::P18_SERVO_STATUS, 1)?;
        Ok(ServoState::from(data[0]))
    }

    /// Get motor speed feedback (P18.01, rpm)
    pub fn get_speed(&mut self) -> Result<i16> {
        let data = self.read_registers(registers::P18_SPEED_FEEDBACK, 1)?;
        Ok(data[0] as i16)
    }

    /// Get average load rate (P18.02, unit: 0.1%)
    pub fn get_load_rate(&mut self) -> Result<f32> {
        let data = self.read_registers(registers::P18_LOAD_RATE, 1)?;
        Ok(data[0] as f32 * 0.1)
    }

    /// Get speed command (P18.03, rpm)
    pub fn get_speed_command(&mut self) -> Result<i16> {
        let data = self.read_registers(registers::P18_SPEED_COMMAND, 1)?;
        Ok(data[0] as i16)
    }

    /// Get internal torque (P18.04, unit: 0.1% of rated)
    pub fn get_torque(&mut self) -> Result<f32> {
        let data = self.read_registers(registers::P18_INTERNAL_TORQUE, 1)?;
        Ok(data[0] as i16 as f32 * 0.1)
    }

    /// Get phase current RMS (P18.05, unit: 0.01 A)
    pub fn get_current(&mut self) -> Result<f32> {
        let data = self.read_registers(registers::P18_PHASE_CURRENT, 1)?;
        Ok(data[0] as f32 * 0.01)
    }

    /// Get DC bus voltage (P18.06, unit: 0.1 V)
    pub fn get_bus_voltage(&mut self) -> Result<f32> {
        let data = self.read_registers(registers::P18_BUS_VOLTAGE, 1)?;
        Ok(data[0] as f32 * 0.1)
    }

    /// Get absolute position (P18.07)
    pub fn get_position(&mut self) -> Result<i32> {
        self.read_i32(registers::P18_ABSOLUTE_POSITION)
    }

    /// Get electrical angle (P18.09, unit: 0.1°)
    pub fn get_electrical_angle(&mut self) -> Result<f32> {
        let data = self.read_registers(registers::P18_ELECTRICAL_ANGLE, 1)?;
        Ok(data[0] as f32 * 0.1)
    }

    /// Get complete servo status
    pub fn get_status(&mut self) -> Result<ServoStatus> {
        Ok(ServoStatus {
            state: self.get_servo_state()?,
            speed: self.get_speed()?,
            load_rate: self.read_registers(registers::P18_LOAD_RATE, 1)?[0],
            torque: self.read_registers(registers::P18_INTERNAL_TORQUE, 1)?[0] as i16,
            current: self.read_registers(registers::P18_PHASE_CURRENT, 1)?[0],
            bus_voltage: self.read_registers(registers::P18_BUS_VOLTAGE, 1)?[0],
            position: self.get_position()?,
            electrical_angle: self.read_registers(registers::P18_ELECTRICAL_ANGLE, 1)?[0],
        })
    }

    // ========================================================================
    // VERSION INFORMATION
    // ========================================================================

    /// Get software version (P12.12)
    pub fn get_software_version(&mut self) -> Result<u16> {
        let data = self.read_registers(registers::P12_SOFTWARE_VERSION, 1)?;
        Ok(data[0])
    }

    /// Get FPGA version (P12.13)
    pub fn get_fpga_version(&mut self) -> Result<u16> {
        let data = self.read_registers(registers::P12_FPGA_VERSION, 1)?;
        Ok(data[0])
    }

    /// Get product series code (P12.14)
    pub fn get_product_code(&mut self) -> Result<u16> {
        let data = self.read_registers(registers::P12_PRODUCT_CODE, 1)?;
        Ok(data[0])
    }
}
