//! Register addresses for DSY-RS low voltage servo drive controller
//!
//! Based on DSY-RS Series Low Voltage Servo Drive User Manual - Chapter 7 Parameters
//!
//! # Address Calculation
//! Address (PXX.YY) = XX * 256 + YY
//! E.g.: P18.01 = 0x1201
//!
//! # Data Format
//! For U16 sending: [value & 0x00ff, (value & 0xff00) >> 8]

/// Calculate register address from parameter code (PXX.YY)
pub const fn param_addr(group: u8, param: u8) -> u16 {
    (group as u16) * 256 + (param as u16)
}

// ============================================================================
// P00 – Basic Control Parameters
// ============================================================================

/// P00.00: Control mode selection
/// - 0 = Position mode
/// - 1 = Speed mode
/// - 2 = Torque mode
pub const P00_CONTROL_MODE: u16 = param_addr(0, 0);

/// P00.01: Direction of rotation
/// - 0 = CCW is forward
/// - 1 = CW is forward
pub const P00_DIRECTION: u16 = param_addr(0, 1);

/// P00.02: Pulse output forward-direction definition
/// - 0 = CCW forward (OA leads OB)
/// - 1 = CW forward (OA lags OB)
pub const P00_PULSE_DIRECTION: u16 = param_addr(0, 2);

/// P00.04: Rigidity level setting (0-31)
pub const P00_RIGIDITY: u16 = param_addr(0, 4);

/// P00.05: Inertia ratio (0-3000, unit: 0.01)
pub const P00_INERTIA_RATIO: u16 = param_addr(0, 5);

/// P00.06: Absolute value system selection
/// - 0 = Incremental position
/// - 1 = Absolute position (linear)
/// - 2 = Absolute position (rotation)
pub const P00_ABSOLUTE_SYSTEM: u16 = param_addr(0, 6);

/// P00.07: System maximum speed (0-10000 rpm)
pub const P00_MAX_SPEED: u16 = param_addr(0, 7);

/// P00.10: Servo OFF stop mode
/// - 0 = Freewheel stop
/// - 1 = Stop at zero speed (decel by P05.06)
pub const P00_SERVO_OFF_STOP_MODE: u16 = param_addr(0, 10);

/// P00.11: Fault No.1 stop mode selection
/// - 0 = Freewheel stop
/// - 1 = Reserved
pub const P00_FAULT1_STOP_MODE: u16 = param_addr(0, 11);

/// P00.12: Fault No.2 stop mode selection
/// - 0 = Freewheel stop
/// - 1 = Stop at zero speed (decel by P05.06)
pub const P00_FAULT2_STOP_MODE: u16 = param_addr(0, 12);

/// P00.13: Stop mode when overtravel
/// - 0 = Freewheel
/// - 1 = Decel then servo-lock
/// - 2 = Decel then freewheel
pub const P00_OVERTRAVEL_STOP_MODE: u16 = param_addr(0, 13);

/// P00.14: Brake output ON delay after command (0-10000 ms)
pub const P00_BRAKE_ON_DELAY: u16 = param_addr(0, 14);

/// P00.15: Brake output OFF delay (10-10000 ms)
pub const P00_BRAKE_OFF_DELAY: u16 = param_addr(0, 15);

/// P00.16: Speed threshold for brake output OFF (0-1000 rpm)
pub const P00_BRAKE_SPEED_THRESHOLD: u16 = param_addr(0, 16);

/// P00.17: No.1 fault in running: delay between servo OFF and brake OFF (0-10000 ms)
pub const P00_FAULT_BRAKE_DELAY: u16 = param_addr(0, 17);

/// P00.18: Energy consumption resistor setting
/// - 0 = Built-in
/// - 1 = External, natural cooling
/// - 2 = External, forced air
/// - 3 = None (capacitance absorption, brake tube closed)
pub const P00_ENERGY_RESISTOR: u16 = param_addr(0, 18);

/// P00.19: External resistor power capacity (1-65535 W)
pub const P00_EXT_RESISTOR_POWER: u16 = param_addr(0, 19);

/// P00.20: External resistance value (1-1000 Ω)
pub const P00_EXT_RESISTANCE: u16 = param_addr(0, 20);

/// P00.21: External resistance heating time constant (1000-65535 ms)
pub const P00_EXT_RESISTANCE_TIME: u16 = param_addr(0, 21);

/// P00.22: Braking start voltage (0-1000 V)
pub const P00_BRAKE_VOLTAGE: u16 = param_addr(0, 22);

/// P00.37: Pulse increment threshold (0-200)
pub const P00_PULSE_INCREMENT_THRESHOLD: u16 = param_addr(0, 37);

/// P00.38: Continuous pulseless reception cycle number (1-200)
pub const P00_PULSELESS_CYCLE: u16 = param_addr(0, 38);

// ============================================================================
// P01 – Servo Motor Parameters
// ============================================================================

/// P01.00: Motor model code (0-65535)
pub const P01_MOTOR_MODEL: u16 = param_addr(1, 0);

/// P01.01: Motor power line phase sequence direction
/// - 0 = CCW
/// - 1 = CW
pub const P01_PHASE_SEQUENCE: u16 = param_addr(1, 1);

/// P01.02: Rated voltage (1-1000 V)
pub const P01_RATED_VOLTAGE: u16 = param_addr(1, 2);

/// P01.03: Rated power (0-65535, unit: 0.01 kW)
pub const P01_RATED_POWER: u16 = param_addr(1, 3);

/// P01.04: Rated current (1-10000, unit: 0.01 A)
pub const P01_RATED_CURRENT: u16 = param_addr(1, 4);

/// P01.05: Rated torque (0-65535, unit: 0.01 Nm)
pub const P01_RATED_TORQUE: u16 = param_addr(1, 5);

/// P01.08: Max speed (0-9000 rpm)
pub const P01_MAX_SPEED: u16 = param_addr(1, 8);

/// P01.09: Rotor inertia (0-10000, unit: 0.01 kg·cm²)
pub const P01_ROTOR_INERTIA: u16 = param_addr(1, 9);

/// P01.10: Pole pairs PMSM (1-50)
pub const P01_POLE_PAIRS: u16 = param_addr(1, 10);

/// P01.11: Stator resistance Rs (1-65535, unit: 0.001 Ω)
pub const P01_STATOR_RESISTANCE: u16 = param_addr(1, 11);

/// P01.12: Q-axis inductance Lq (1-65535, unit: 0.01 mH)
pub const P01_Q_INDUCTANCE: u16 = param_addr(1, 12);

/// P01.13: D-axis inductance Ld (1-65535, unit: 0.01 mH)
pub const P01_D_INDUCTANCE: u16 = param_addr(1, 13);

/// P01.14: Back EMF (1-65535, unit: 0.01 mV/rpm)
pub const P01_BACK_EMF: u16 = param_addr(1, 14);

/// P01.15: Torque factor (1-65535, unit: 0.001 Nm/A)
pub const P01_TORQUE_FACTOR: u16 = param_addr(1, 15);

/// P01.18: Encoder selection
/// - 0 = 2500-line
/// - 1 = 17-bit incremental
/// - 2 = 17-bit absolute
/// - 3 = 23-bit incremental
/// - 4 = 23-bit absolute
pub const P01_ENCODER_SELECTION: u16 = param_addr(1, 18);

/// P01.20: Encoder resolution (1-1,073,741,824)
pub const P01_ENCODER_RESOLUTION: u16 = param_addr(1, 20);

/// P01.22: Z electrical angle (0-3600, unit: 0.1°)
pub const P01_Z_ELECTRICAL_ANGLE: u16 = param_addr(1, 22);

/// P01.23: U rising-edge electrical angle (0-3600, unit: 0.1°)
pub const P01_U_ELECTRICAL_ANGLE: u16 = param_addr(1, 23);

/// P01.24: FPGA upload motor model (read-only)
pub const P01_FPGA_MOTOR_MODEL: u16 = param_addr(1, 24);

// ============================================================================
// P02 – Digital Terminal I/O Parameters
// ============================================================================

/// P02.00: FunINL unassigned state (HEX) - Bit0=FunIN.1 … Bit15=FunIN.16
pub const P02_FUNINL_STATE: u16 = param_addr(2, 0);

/// P02.01: DI1 terminal function selection (0=None, 1-45=FunIN.1-45)
pub const P02_DI1_FUNCTION: u16 = param_addr(2, 1);

/// P02.02: DI2 terminal function selection
pub const P02_DI2_FUNCTION: u16 = param_addr(2, 2);

/// P02.03: DI3 terminal function selection
pub const P02_DI3_FUNCTION: u16 = param_addr(2, 3);

/// P02.10: FunINH unassigned state (HEX) - Bit0=FunIN.17 … Bit15=FunIN.32
pub const P02_FUNINH_STATE: u16 = param_addr(2, 10);

/// P02.11: DI1 terminal logic selection
/// - 0 = Low active
/// - 1 = High active
/// - 2 = Rising edge
/// - 3 = Falling edge
/// - 4 = Both edges
pub const P02_DI1_LOGIC: u16 = param_addr(2, 11);

/// P02.12: DI2 terminal logic selection
pub const P02_DI2_LOGIC: u16 = param_addr(2, 12);

/// P02.13: DI3 terminal logic selection
pub const P02_DI3_LOGIC: u16 = param_addr(2, 13);

/// P02.21: DO1 terminal function selection (0=None, 1-25=FunOUT.1-25)
pub const P02_DO1_FUNCTION: u16 = param_addr(2, 21);

/// P02.22: DO2 terminal function selection (must be 11=Lock release signal output)
pub const P02_DO2_FUNCTION: u16 = param_addr(2, 22);

/// P02.31: DO1 terminal logic (polarity invert)
/// - 0 = NO (conduct when active)
/// - 1 = NC (open when active)
pub const P02_DO1_LOGIC: u16 = param_addr(2, 31);

/// P02.32: DO2 terminal logic
pub const P02_DO2_LOGIC: u16 = param_addr(2, 32);

// ============================================================================
// P04 – Position Control Parameters
// ============================================================================

/// P04.00: Main position command A source
/// - 0 = Low-speed pulse
/// - 1 = High-speed pulse
/// - 2 = Step amount
/// - 4 = Multi-segment position
/// - 5 = Communication
pub const P04_POSITION_CMD_SOURCE: u16 = param_addr(4, 0);

/// P04.02: Step amount (-9999 to 9999)
pub const P04_STEP_AMOUNT: u16 = param_addr(4, 2);

/// P04.03: Position command smoothing filter (0-65535, unit: 0.1 ms)
pub const P04_POSITION_FILTER: u16 = param_addr(4, 3);

/// P04.04: Position command FIR filter (0-1280, unit: 0.1 ms)
pub const P04_POSITION_FIR_FILTER: u16 = param_addr(4, 4);

/// P04.05: Units required for one revolution (32-bit, PTP only)
pub const P04_UNITS_PER_REV: u16 = param_addr(4, 5);

/// P04.07: Electronic gear 1 numerator (32-bit)
pub const P04_GEAR1_NUMERATOR: u16 = param_addr(4, 7);

/// P04.09: Electronic gear 1 denominator (32-bit)
pub const P04_GEAR1_DENOMINATOR: u16 = param_addr(4, 9);

/// P04.11: Electronic gear 2 numerator (32-bit)
pub const P04_GEAR2_NUMERATOR: u16 = param_addr(4, 11);

/// P04.13: Electronic gear 2 denominator (32-bit)
pub const P04_GEAR2_DENOMINATOR: u16 = param_addr(4, 13);

/// P04.21: Pulse shape
/// - 0 = Pulse+Dir, +logic
/// - 1 = Dir+Pulse, −logic
/// - 2 = A/B quad, +logic
/// - 3 = A/B quad, −logic
/// - 4 = CCW/CW, +logic
/// - 5 = CCW/CW, −logic
pub const P04_PULSE_SHAPE: u16 = param_addr(4, 21);

/// P04.22: Position deviation clear
/// - 0 = Clear on fault or servo OFF
/// - 1 = Clear only on fault
/// - 2 = Clear by DI (PERR-CLR)
pub const P04_DEVIATION_CLEAR: u16 = param_addr(4, 22);

/// P04.23: COIN output condition
/// - 0 = |dev|<range
/// - 1 = |dev|<range AND filtered cmd=0
/// - 2 = |dev|<range AND cmd=0
pub const P04_COIN_CONDITION: u16 = param_addr(4, 23);

/// P04.24: Positioning completion range (1-65535 pulse)
pub const P04_POSITIONING_RANGE: u16 = param_addr(4, 24);

/// P04.25: Positioning close range (1-65535 pulse)
pub const P04_POSITIONING_CLOSE_RANGE: u16 = param_addr(4, 25);

// ============================================================================
// P05 – Speed Control Parameters
// ============================================================================

/// P05.00: Main speed command A source
/// - 0 = Digit value (P05.03)
/// - 1 = Reserved
/// - 2 = Reserved
pub const P05_SPEED_CMD_SOURCE: u16 = param_addr(5, 0);

/// P05.01: Auxiliary speed command B source
/// - 0 = Digit value (P05.03)
/// - 1 = Reserved
/// - 2 = Reserved
/// - 3 = Multi-speed command
pub const P05_AUX_SPEED_SOURCE: u16 = param_addr(5, 1);

/// P05.02: Speed command selection
/// - 0 = Main A
/// - 2 = Aux B
/// - 3 = A/B switching
pub const P05_SPEED_CMD_SELECT: u16 = param_addr(5, 2);

/// P05.03: Speed command keyboard setting (-9000 to 9000 rpm)
pub const P05_SPEED_COMMAND: u16 = param_addr(5, 3);

/// P05.04: Jog speed setting (0-9000 rpm)
pub const P05_JOG_SPEED: u16 = param_addr(5, 4);

/// P05.05: Acceleration time (0-10000 ms)
pub const P05_ACCEL_TIME: u16 = param_addr(5, 5);

/// P05.06: Deceleration time (0-10000 ms)
pub const P05_DECEL_TIME: u16 = param_addr(5, 6);

/// P05.07: Speed limit selection (0=Use P05.08/P05.09)
pub const P05_SPEED_LIMIT_SELECT: u16 = param_addr(5, 7);

/// P05.08: Forward speed limit (0-9000 rpm)
pub const P05_FORWARD_SPEED_LIMIT: u16 = param_addr(5, 8);

/// P05.09: Backward speed limit (0-9000 rpm)
pub const P05_BACKWARD_SPEED_LIMIT: u16 = param_addr(5, 9);

/// P05.14: Speed direction selection
/// - 0 = Unchanged
/// - 1 = Reversed
/// - 2 = By DI func 25
/// - 3 = By DI func 40/41
pub const P05_SPEED_DIRECTION: u16 = param_addr(5, 14);

/// P05.15: Zero fixed speed value (0-6000 rpm)
pub const P05_ZERO_SPEED_VALUE: u16 = param_addr(5, 15);

/// P05.16: Motor running signal speed threshold (0-1000 rpm)
pub const P05_RUNNING_THRESHOLD: u16 = param_addr(5, 16);

/// P05.17: Speed uniform signal width (0-100 rpm)
pub const P05_SPEED_UNIFORM_WIDTH: u16 = param_addr(5, 17);

/// P05.18: Speed reaches specified value (0-6000 rpm)
pub const P05_SPEED_REACHED_VALUE: u16 = param_addr(5, 18);

/// P05.20: Zero-speed judgment threshold (0-6000 rpm)
pub const P05_ZERO_SPEED_THRESHOLD: u16 = param_addr(5, 20);

// ============================================================================
// P06 – Torque Control Parameters
// ============================================================================

/// P06.00: Main torque command A source
/// - 0 = Digit (P06.05)
/// - 1 = Reserved
pub const P06_TORQUE_CMD_SOURCE: u16 = param_addr(6, 0);

/// P06.02: Torque command selection
/// - 0 = A
/// - 1 = B
/// - 2 = A+B
/// - 3 = A/B switch
pub const P06_TORQUE_CMD_SELECT: u16 = param_addr(6, 2);

/// P06.04: Torque command filter time (-3000 to 3000, unit: 0.01 ms)
pub const P06_TORQUE_FILTER: u16 = param_addr(6, 4);

/// P06.05: Torque command keyboard setting (-3000 to 3000, unit: 0.1% of rated)
pub const P06_TORQUE_COMMAND: u16 = param_addr(6, 5);

/// P06.06: Torque limit source
/// - 0 = Internal ± limit
/// - 1 = External ± limit (P_CL/N_CL)
pub const P06_TORQUE_LIMIT_SOURCE: u16 = param_addr(6, 6);

/// P06.08: Forward internal torque limit (0-5000, unit: 0.1%)
pub const P06_FORWARD_TORQUE_LIMIT: u16 = param_addr(6, 8);

/// P06.09: Backward internal torque limit (0-5000, unit: 0.1%)
pub const P06_BACKWARD_TORQUE_LIMIT: u16 = param_addr(6, 9);

/// P06.10: Forward external torque limit (0-5000, unit: 0.1%)
pub const P06_FORWARD_EXT_TORQUE_LIMIT: u16 = param_addr(6, 10);

/// P06.11: Backward external torque limit (0-5000, unit: 0.1%)
pub const P06_BACKWARD_EXT_TORQUE_LIMIT: u16 = param_addr(6, 11);

/// P06.13: Speed limit source (torque control)
/// - 0 = Internal (P06.15/P06.16)
/// - 1 = Reserved
pub const P06_SPEED_LIMIT_SOURCE: u16 = param_addr(6, 13);

/// P06.15: Positive speed limit in torque mode (0-9000 rpm)
pub const P06_POSITIVE_SPEED_LIMIT: u16 = param_addr(6, 15);

/// P06.16: Negative speed limit in torque mode (0-9000 rpm)
pub const P06_NEGATIVE_SPEED_LIMIT: u16 = param_addr(6, 16);

/// P06.21: Multi-segment torque command 1 (-3000 to 3000, unit: 0.1%)
pub const P06_TORQUE_SEGMENT1: u16 = param_addr(6, 21);

/// P06.22: Multi-segment torque command 2
pub const P06_TORQUE_SEGMENT2: u16 = param_addr(6, 22);

/// P06.23: Multi-segment torque command 3
pub const P06_TORQUE_SEGMENT3: u16 = param_addr(6, 23);

// ============================================================================
// P07 – Gain Parameters
// ============================================================================

/// P07.00: Position loop gain 1 (10-20000, unit: 0.1 Hz)
pub const P07_POSITION_GAIN1: u16 = param_addr(7, 0);

/// P07.01: Speed loop gain 1 (10-20000, unit: 0.1 Hz)
pub const P07_SPEED_GAIN1: u16 = param_addr(7, 1);

/// P07.02: Speed loop integral time 1 (15-512, unit: 0.01 ms)
pub const P07_SPEED_INTEGRAL1: u16 = param_addr(7, 2);

/// P07.03: Speed detection filter 1 (0-200, unit: 0.01 ms)
pub const P07_SPEED_FILTER1: u16 = param_addr(7, 3);

/// P07.05: Position loop gain 2 (10-20000, unit: 0.1 Hz)
pub const P07_POSITION_GAIN2: u16 = param_addr(7, 5);

/// P07.06: Speed loop gain 2 (10-20000, unit: 0.1 Hz)
pub const P07_SPEED_GAIN2: u16 = param_addr(7, 6);

/// P07.10: GAINSWITCH action select
/// - 0 = PI/P switch (gain fixed group 1)
/// - 1 = Gain1/Gain2 switching
pub const P07_GAINSWITCH_ACTION: u16 = param_addr(7, 10);

/// P07.11: Gain switching mode (0-13, see manual for details)
pub const P07_GAIN_SWITCH_MODE: u16 = param_addr(7, 11);

// ============================================================================
// P08 – Advanced Adjustment Parameters
// ============================================================================

/// P08.00: Adaptive filter mode (0-5)
pub const P08_ADAPTIVE_FILTER_MODE: u16 = param_addr(8, 0);

/// P08.02: 1st notch filter frequency (10-4000 Hz)
pub const P08_NOTCH1_FREQUENCY: u16 = param_addr(8, 2);

/// P08.03: 1st notch filter width (0-8)
pub const P08_NOTCH1_WIDTH: u16 = param_addr(8, 3);

/// P08.04: 1st notch filter depth (0-100)
pub const P08_NOTCH1_DEPTH: u16 = param_addr(8, 4);

/// P08.15: Damping filter switch (0=OFF, 1=ON)
pub const P08_DAMPING_FILTER: u16 = param_addr(8, 15);

/// P08.17: Damping filter selection (0=Filter A, 1=Filter B)
pub const P08_DAMPING_FILTER_SELECT: u16 = param_addr(8, 17);

/// P08.23: Inertia identification mode
/// - 0 = Offline triangle (+/−)
/// - 1 = Offline JOG
pub const P08_INERTIA_ID_MODE: u16 = param_addr(8, 23);

/// P08.26: HF vibration suppression switch (0=OFF, 1=ON)
pub const P08_HF_VIBRATION_SUPPRESS: u16 = param_addr(8, 26);

/// P08.33: Anti-disturbance compensation (0=OFF, 1=ON)
pub const P08_ANTI_DISTURBANCE: u16 = param_addr(8, 33);

/// P08.39: Momentary speed compensation (0=OFF, 1=ON)
pub const P08_SPEED_COMPENSATION: u16 = param_addr(8, 39);

/// P08.45: Model compensation switch
/// - 0 = Off
/// - 1 = Rigid model
/// - 2 = 2nd-order vector model
pub const P08_MODEL_COMPENSATION: u16 = param_addr(8, 45);

// ============================================================================
// P09 – Failure & Protection
// ============================================================================

/// P09.02: Undervoltage detection delay (100-20000, unit: 0.1 ms)
pub const P09_UNDERVOLTAGE_DELAY: u16 = param_addr(9, 2);

/// P09.04: Out-of-control protection (0=Open, 1=Off)
pub const P09_RUNAWAY_PROTECTION: u16 = param_addr(9, 4);

/// P09.05: Overload warning value (1-100 %)
pub const P09_OVERLOAD_WARNING: u16 = param_addr(9, 5);

/// P09.06: Motor overload factor (10-300 %)
pub const P09_MOTOR_OVERLOAD_FACTOR: u16 = param_addr(9, 6);

/// P09.07: Undervoltage protection point (50-100, 100=default %)
pub const P09_UNDERVOLTAGE_POINT: u16 = param_addr(9, 7);

/// P09.08: Overspeed fault point (50-120, 100=max speed %)
pub const P09_OVERSPEED_POINT: u16 = param_addr(9, 8);

/// P09.09: Position deviation excessive threshold (32-bit, 1-1,073,741,824 pulse)
pub const P09_POSITION_DEVIATION_THRESHOLD: u16 = param_addr(9, 9);

/// P09.24: Locked-rotor over-temp enable (0=Disable, 1=Enable)
pub const P09_LOCKED_ROTOR_TEMP: u16 = param_addr(9, 24);

/// P09.25: Overload protection enable
/// - 0 = Motor overload + avg-load overload
/// - 1 = Motor overload only
/// - 2 = Avg-load only
/// - 3 = Disable both
pub const P09_OVERLOAD_PROTECTION: u16 = param_addr(9, 25);

// ============================================================================
// P10 – Communication Parameters
// ============================================================================

/// P10.00: Communication address (0-247, 0=broadcast)
pub const P10_COMM_ADDRESS: u16 = param_addr(10, 0);

/// P10.02: Modbus baud rate setting
/// - 0 = 2400
/// - 1 = 4800
/// - 2 = 9600
/// - 3 = 19200
/// - 4 = 38400
/// - 5 = 57600
/// - 6 = 115200
pub const P10_MODBUS_BAUDRATE: u16 = param_addr(10, 2);

/// P10.03: Modbus data format
/// - 0 = No parity, 2 stop
/// - 1 = Even parity, 1 stop
/// - 2 = Odd parity, 1 stop
/// - 3 = No parity, 1 stop
pub const P10_MODBUS_FORMAT: u16 = param_addr(10, 3);

/// P10.04: Write comm params to EEPROM (0=No, 1=Yes except P11 & P18)
pub const P10_WRITE_EEPROM: u16 = param_addr(10, 4);

/// P10.05: RS232 baud rate setting (same values as P10.02)
pub const P10_RS232_BAUDRATE: u16 = param_addr(10, 5);

/// P10.06: RS485 address source
/// - 0 = DIP switch
/// - 1 = Host setting
pub const P10_RS485_ADDRESS_SOURCE: u16 = param_addr(10, 6);

// ============================================================================
// P11 – Auxiliary Function Parameters
// ============================================================================

/// P11.01: Fault reset (0=None, 1=Reset)
pub const P11_FAULT_RESET: u16 = param_addr(11, 1);

/// P11.02: Soft reset (0=None, 1=Reset)
pub const P11_SOFT_RESET: u16 = param_addr(11, 2);

/// P11.03: Inertia recognition function (Enter to execute)
pub const P11_INERTIA_RECOGNITION: u16 = param_addr(11, 3);

/// P11.06: Absolute encoder reset
/// - 0 = None
/// - 1 = Clear warnings/errors
/// - 2 = Reset multi-turn data
pub const P11_ENCODER_RESET: u16 = param_addr(11, 6);

/// P11.07: Absolute system soft limit set
/// - 0 = None
/// - 1 = Set current pos as negative limit
/// - 2 = Set current pos as positive limit
pub const P11_SOFT_LIMIT_SET: u16 = param_addr(11, 7);

/// P11.09: System initialization
/// - 0 = None
/// - 1 = Factory reset (except P01 & P17)
/// - 2 = Clear fault record
pub const P11_SYSTEM_INIT: u16 = param_addr(11, 9);

/// P11.10: Forced DIDO enable
/// - 0 = None
/// - 1 = Force DI
/// - 2 = Force DO
/// - 3 = Force DI&DO
pub const P11_FORCED_DIDO: u16 = param_addr(11, 10);

/// P11.11: Set DI forced input (0-0x01FF)
pub const P11_FORCED_DI_VALUE: u16 = param_addr(11, 11);

/// P11.12: Set DO forced output (0-0x001F)
pub const P11_FORCED_DO_VALUE: u16 = param_addr(11, 12);

/// P11.13: Emergency stop settings (0=None, 1=Emergency stop)
pub const P11_EMERGENCY_STOP: u16 = param_addr(11, 13);

// ============================================================================
// P12 – Keyboard Display Parameters
// ============================================================================

/// P12.00: LED warning display selection (0=Show warnings, 1=Do not show)
pub const P12_LED_WARNING: u16 = param_addr(12, 0);

/// P12.01: Default display settings (0-100)
pub const P12_DEFAULT_DISPLAY: u16 = param_addr(12, 1);

/// P12.03: Speed display filter time (0-10000, unit: 0.1 ms)
pub const P12_SPEED_DISPLAY_FILTER: u16 = param_addr(12, 3);

/// P12.11: Non-standard version number (read-only)
pub const P12_NONSTANDARD_VERSION: u16 = param_addr(12, 11);

/// P12.12: Software version number (read-only)
pub const P12_SOFTWARE_VERSION: u16 = param_addr(12, 12);

/// P12.13: FPGA version number (read-only)
pub const P12_FPGA_VERSION: u16 = param_addr(12, 13);

/// P12.14: Product series code (read-only)
pub const P12_PRODUCT_CODE: u16 = param_addr(12, 14);

// ============================================================================
// P13 – Multi-Segment Position Parameters
// ============================================================================

/// P13.00: Operation mode
/// - 0 = Single
/// - 1 = Cycle
/// - 2 = DI switch
pub const P13_OPERATION_MODE: u16 = param_addr(13, 0);

/// P13.01: Start segment (1-16)
pub const P13_START_SEGMENT: u16 = param_addr(13, 1);

/// P13.02: End segment (1-16)
pub const P13_END_SEGMENT: u16 = param_addr(13, 2);

/// P13.03: Interrupt handling (0=Continue, 1=Restart)
pub const P13_INTERRUPT_HANDLING: u16 = param_addr(13, 3);

/// P13.04: Wait time unit (0=ms, 1=s)
pub const P13_WAIT_TIME_UNIT: u16 = param_addr(13, 4);

/// P13.05: Position mode (0=Incremental, 1=Absolute)
pub const P13_POSITION_MODE: u16 = param_addr(13, 5);

// Segment 1
/// P13.08: Segment 1 displacement (32-bit)
pub const P13_SEG1_DISPLACEMENT: u16 = param_addr(13, 8);
/// P13.10: Segment 1 max speed (rpm)
pub const P13_SEG1_SPEED: u16 = param_addr(13, 10);
/// P13.11: Segment 1 accel/decel time (ms)
pub const P13_SEG1_ACCEL_DECEL: u16 = param_addr(13, 11);
/// P13.12: Segment 1 wait time
pub const P13_SEG1_WAIT_TIME: u16 = param_addr(13, 12);

// Segment 2
pub const P13_SEG2_DISPLACEMENT: u16 = param_addr(13, 13);
pub const P13_SEG2_SPEED: u16 = param_addr(13, 15);
pub const P13_SEG2_ACCEL_DECEL: u16 = param_addr(13, 16);
pub const P13_SEG2_WAIT_TIME: u16 = param_addr(13, 17);

// Segment 3
pub const P13_SEG3_DISPLACEMENT: u16 = param_addr(13, 18);
pub const P13_SEG3_SPEED: u16 = param_addr(13, 20);
pub const P13_SEG3_ACCEL_DECEL: u16 = param_addr(13, 21);
pub const P13_SEG3_WAIT_TIME: u16 = param_addr(13, 22);

// Segment 4
pub const P13_SEG4_DISPLACEMENT: u16 = param_addr(13, 23);
pub const P13_SEG4_SPEED: u16 = param_addr(13, 25);
pub const P13_SEG4_ACCEL_DECEL: u16 = param_addr(13, 26);
pub const P13_SEG4_WAIT_TIME: u16 = param_addr(13, 27);

// Segment 5
pub const P13_SEG5_DISPLACEMENT: u16 = param_addr(13, 28);
pub const P13_SEG5_SPEED: u16 = param_addr(13, 30);
pub const P13_SEG5_ACCEL_DECEL: u16 = param_addr(13, 31);
pub const P13_SEG5_WAIT_TIME: u16 = param_addr(13, 32);

// Segment 6
pub const P13_SEG6_DISPLACEMENT: u16 = param_addr(13, 33);
pub const P13_SEG6_SPEED: u16 = param_addr(13, 35);
pub const P13_SEG6_ACCEL_DECEL: u16 = param_addr(13, 36);
pub const P13_SEG6_WAIT_TIME: u16 = param_addr(13, 37);

// Segment 7
pub const P13_SEG7_DISPLACEMENT: u16 = param_addr(13, 38);
pub const P13_SEG7_SPEED: u16 = param_addr(13, 40);
pub const P13_SEG7_ACCEL_DECEL: u16 = param_addr(13, 41);
pub const P13_SEG7_WAIT_TIME: u16 = param_addr(13, 42);

// Segment 8
pub const P13_SEG8_DISPLACEMENT: u16 = param_addr(13, 43);
pub const P13_SEG8_SPEED: u16 = param_addr(13, 45);
pub const P13_SEG8_ACCEL_DECEL: u16 = param_addr(13, 46);
pub const P13_SEG8_WAIT_TIME: u16 = param_addr(13, 47);

// Segment 9
pub const P13_SEG9_DISPLACEMENT: u16 = param_addr(13, 48);
pub const P13_SEG9_SPEED: u16 = param_addr(13, 50);
pub const P13_SEG9_ACCEL_DECEL: u16 = param_addr(13, 51);
pub const P13_SEG9_WAIT_TIME: u16 = param_addr(13, 52);

// Segment 10
pub const P13_SEG10_DISPLACEMENT: u16 = param_addr(13, 53);
pub const P13_SEG10_SPEED: u16 = param_addr(13, 55);
pub const P13_SEG10_ACCEL_DECEL: u16 = param_addr(13, 56);
pub const P13_SEG10_WAIT_TIME: u16 = param_addr(13, 57);

// Segment 11
pub const P13_SEG11_DISPLACEMENT: u16 = param_addr(13, 58);
pub const P13_SEG11_SPEED: u16 = param_addr(13, 60);
pub const P13_SEG11_ACCEL_DECEL: u16 = param_addr(13, 61);
pub const P13_SEG11_WAIT_TIME: u16 = param_addr(13, 62);

// Segment 12
pub const P13_SEG12_DISPLACEMENT: u16 = param_addr(13, 63);
pub const P13_SEG12_SPEED: u16 = param_addr(13, 65);
pub const P13_SEG12_ACCEL_DECEL: u16 = param_addr(13, 66);
pub const P13_SEG12_WAIT_TIME: u16 = param_addr(13, 67);

// Segment 13
pub const P13_SEG13_DISPLACEMENT: u16 = param_addr(13, 68);
pub const P13_SEG13_SPEED: u16 = param_addr(13, 70);
pub const P13_SEG13_ACCEL_DECEL: u16 = param_addr(13, 71);
pub const P13_SEG13_WAIT_TIME: u16 = param_addr(13, 72);

// Segment 14
pub const P13_SEG14_DISPLACEMENT: u16 = param_addr(13, 73);
pub const P13_SEG14_SPEED: u16 = param_addr(13, 75);
pub const P13_SEG14_ACCEL_DECEL: u16 = param_addr(13, 76);
pub const P13_SEG14_WAIT_TIME: u16 = param_addr(13, 77);

// Segment 15
pub const P13_SEG15_DISPLACEMENT: u16 = param_addr(13, 78);
pub const P13_SEG15_SPEED: u16 = param_addr(13, 80);
pub const P13_SEG15_ACCEL_DECEL: u16 = param_addr(13, 81);
pub const P13_SEG15_WAIT_TIME: u16 = param_addr(13, 82);

// Segment 16
pub const P13_SEG16_DISPLACEMENT: u16 = param_addr(13, 83);
pub const P13_SEG16_SPEED: u16 = param_addr(13, 85);
pub const P13_SEG16_ACCEL_DECEL: u16 = param_addr(13, 86);
pub const P13_SEG16_WAIT_TIME: u16 = param_addr(13, 87);

// ============================================================================
// P14 – Multi-Speed Parameters
// ============================================================================

/// P14.00: Operation mode (0=Single, 1=Cycle, 2=DI)
pub const P14_OPERATION_MODE: u16 = param_addr(14, 0);

/// P14.01: End segment (1-16)
pub const P14_END_SEGMENT: u16 = param_addr(14, 1);

/// P14.02: Time unit (0=s, 1=min)
pub const P14_TIME_UNIT: u16 = param_addr(14, 2);

/// P14.03: Accel/Decel time 1 (0-10000 ms)
pub const P14_ACCEL_DECEL_TIME1: u16 = param_addr(14, 3);

/// P14.04: Accel/Decel time 2
pub const P14_ACCEL_DECEL_TIME2: u16 = param_addr(14, 4);

/// P14.05: Accel/Decel time 3
pub const P14_ACCEL_DECEL_TIME3: u16 = param_addr(14, 5);

/// P14.06: Accel/Decel time 4
pub const P14_ACCEL_DECEL_TIME4: u16 = param_addr(14, 6);

// Speed segments 1-16
pub const P14_SEG1_SPEED: u16 = param_addr(14, 7);
pub const P14_SEG1_TIME: u16 = param_addr(14, 8);
pub const P14_SEG1_ACCEL_SELECT: u16 = param_addr(14, 9);

pub const P14_SEG2_SPEED: u16 = param_addr(14, 10);
pub const P14_SEG2_TIME: u16 = param_addr(14, 11);
pub const P14_SEG2_ACCEL_SELECT: u16 = param_addr(14, 12);

pub const P14_SEG3_SPEED: u16 = param_addr(14, 13);
pub const P14_SEG3_TIME: u16 = param_addr(14, 14);
pub const P14_SEG3_ACCEL_SELECT: u16 = param_addr(14, 15);

pub const P14_SEG4_SPEED: u16 = param_addr(14, 16);
pub const P14_SEG4_TIME: u16 = param_addr(14, 17);
pub const P14_SEG4_ACCEL_SELECT: u16 = param_addr(14, 18);

pub const P14_SEG5_SPEED: u16 = param_addr(14, 19);
pub const P14_SEG5_TIME: u16 = param_addr(14, 20);
pub const P14_SEG5_ACCEL_SELECT: u16 = param_addr(14, 21);

pub const P14_SEG6_SPEED: u16 = param_addr(14, 22);
pub const P14_SEG6_TIME: u16 = param_addr(14, 23);
pub const P14_SEG6_ACCEL_SELECT: u16 = param_addr(14, 24);

pub const P14_SEG7_SPEED: u16 = param_addr(14, 25);
pub const P14_SEG7_TIME: u16 = param_addr(14, 26);
pub const P14_SEG7_ACCEL_SELECT: u16 = param_addr(14, 27);

pub const P14_SEG8_SPEED: u16 = param_addr(14, 28);
pub const P14_SEG8_TIME: u16 = param_addr(14, 29);
pub const P14_SEG8_ACCEL_SELECT: u16 = param_addr(14, 30);

pub const P14_SEG9_SPEED: u16 = param_addr(14, 31);
pub const P14_SEG9_TIME: u16 = param_addr(14, 32);
pub const P14_SEG9_ACCEL_SELECT: u16 = param_addr(14, 33);

pub const P14_SEG10_SPEED: u16 = param_addr(14, 34);
pub const P14_SEG10_TIME: u16 = param_addr(14, 35);
pub const P14_SEG10_ACCEL_SELECT: u16 = param_addr(14, 36);

pub const P14_SEG11_SPEED: u16 = param_addr(14, 37);
pub const P14_SEG11_TIME: u16 = param_addr(14, 38);
pub const P14_SEG11_ACCEL_SELECT: u16 = param_addr(14, 39);

pub const P14_SEG12_SPEED: u16 = param_addr(14, 40);
pub const P14_SEG12_TIME: u16 = param_addr(14, 41);
pub const P14_SEG12_ACCEL_SELECT: u16 = param_addr(14, 42);

pub const P14_SEG13_SPEED: u16 = param_addr(14, 43);
pub const P14_SEG13_TIME: u16 = param_addr(14, 44);
pub const P14_SEG13_ACCEL_SELECT: u16 = param_addr(14, 45);

pub const P14_SEG14_SPEED: u16 = param_addr(14, 46);
pub const P14_SEG14_TIME: u16 = param_addr(14, 47);
pub const P14_SEG14_ACCEL_SELECT: u16 = param_addr(14, 48);

pub const P14_SEG15_SPEED: u16 = param_addr(14, 49);
pub const P14_SEG15_TIME: u16 = param_addr(14, 50);
pub const P14_SEG15_ACCEL_SELECT: u16 = param_addr(14, 51);

pub const P14_SEG16_SPEED: u16 = param_addr(14, 52);
pub const P14_SEG16_TIME: u16 = param_addr(14, 53);
pub const P14_SEG16_ACCEL_SELECT: u16 = param_addr(14, 54);

// ============================================================================
// P16 – Special Function Parameters
// ============================================================================

/// P16.00: Fixed length interrupt enable (0-1)
pub const P16_FIXED_LENGTH_ENABLE: u16 = param_addr(16, 0);

/// P16.01: Fixed length 1 displacement (0-2^30 unit)
pub const P16_FIXED_LENGTH1_DISP: u16 = param_addr(16, 1);

/// P16.03: Fixed length 1 speed (0-9000 rpm)
pub const P16_FIXED_LENGTH1_SPEED: u16 = param_addr(16, 3);

/// P16.04: Fixed length accel time (0-1000 ms)
pub const P16_FIXED_LENGTH_ACCEL: u16 = param_addr(16, 4);

/// P16.05: Fixed length decel time (0-1000 ms)
pub const P16_FIXED_LENGTH_DECEL: u16 = param_addr(16, 5);

/// P16.06: Lock release enable (0-1)
pub const P16_LOCK_RELEASE_ENABLE: u16 = param_addr(16, 6);

/// P16.08: Homing enable mode (0-6)
pub const P16_HOMING_ENABLE_MODE: u16 = param_addr(16, 8);

/// P16.09: Homing mode (0-17)
pub const P16_HOMING_MODE: u16 = param_addr(16, 9);

/// P16.10: Homing high speed (10-3000 rpm)
pub const P16_HOMING_HIGH_SPEED: u16 = param_addr(16, 10);

/// P16.11: Homing low speed (10-1000 rpm)
pub const P16_HOMING_LOW_SPEED: u16 = param_addr(16, 11);

/// P16.12: Homing accel limit (0-65535 ms)
pub const P16_HOMING_ACCEL: u16 = param_addr(16, 12);

/// P16.13: Homing timeout (0-65535 ms)
pub const P16_HOMING_TIMEOUT: u16 = param_addr(16, 13);

/// P16.14: Mechanical home offset (±2^30 unit)
pub const P16_HOME_OFFSET: u16 = param_addr(16, 14);

/// P16.28: Absolute encoder origin (0-2^32 inc)
pub const P16_ENCODER_ORIGIN: u16 = param_addr(16, 28);

/// P16.30: Encoder turns at origin (0-32767 turn)
pub const P16_ENCODER_TURNS: u16 = param_addr(16, 30);

/// P16.31: Zero wait count (0-65535 ms)
pub const P16_ZERO_WAIT_COUNT: u16 = param_addr(16, 31);

/// P16.37: Fixed length 2 displacement (±2^30 unit)
pub const P16_FIXED_LENGTH2_DISP: u16 = param_addr(16, 37);

/// P16.39: Fixed length 2 speed (0-9000 rpm)
pub const P16_FIXED_LENGTH2_SPEED: u16 = param_addr(16, 39);

// ============================================================================
// P18 – Display Parameters (Read-Only Status Registers)
// ============================================================================

/// P18.00: Servo status (Ready/Run/Err/AL)
pub const P18_SERVO_STATUS: u16 = param_addr(18, 0);

/// P18.01: Motor speed feedback (±9000 rpm)
pub const P18_SPEED_FEEDBACK: u16 = param_addr(18, 1);

/// P18.02: Average load rate (0-3000, unit: 0.1%)
pub const P18_LOAD_RATE: u16 = param_addr(18, 2);

/// P18.03: Speed command (±9000 rpm)
pub const P18_SPEED_COMMAND: u16 = param_addr(18, 3);

/// P18.04: Internal torque (±5000, unit: 0.1%)
pub const P18_INTERNAL_TORQUE: u16 = param_addr(18, 4);

/// P18.05: Phase current RMS (0-10000, unit: 0.01 A)
pub const P18_PHASE_CURRENT: u16 = param_addr(18, 5);

/// P18.06: DC bus voltage (0-10000, unit: 0.1 V)
pub const P18_BUS_VOLTAGE: u16 = param_addr(18, 6);

/// P18.07: Absolute position (±2^30 unit)
pub const P18_ABSOLUTE_POSITION: u16 = param_addr(18, 7);

/// P18.09: Electrical angle (0-3600, unit: 0.1°)
pub const P18_ELECTRICAL_ANGLE: u16 = param_addr(18, 9);

// ============================================================================
// Helper Functions
// ============================================================================

/// Get the segment displacement register for a given segment (1-16)
pub const fn get_segment_displacement_register(segment: u8) -> Option<u16> {
    match segment {
        1 => Some(P13_SEG1_DISPLACEMENT),
        2 => Some(P13_SEG2_DISPLACEMENT),
        3 => Some(P13_SEG3_DISPLACEMENT),
        4 => Some(P13_SEG4_DISPLACEMENT),
        5 => Some(P13_SEG5_DISPLACEMENT),
        6 => Some(P13_SEG6_DISPLACEMENT),
        7 => Some(P13_SEG7_DISPLACEMENT),
        8 => Some(P13_SEG8_DISPLACEMENT),
        9 => Some(P13_SEG9_DISPLACEMENT),
        10 => Some(P13_SEG10_DISPLACEMENT),
        11 => Some(P13_SEG11_DISPLACEMENT),
        12 => Some(P13_SEG12_DISPLACEMENT),
        13 => Some(P13_SEG13_DISPLACEMENT),
        14 => Some(P13_SEG14_DISPLACEMENT),
        15 => Some(P13_SEG15_DISPLACEMENT),
        16 => Some(P13_SEG16_DISPLACEMENT),
        _ => None,
    }
}

/// Get the segment speed register for a given segment (1-16)
pub const fn get_segment_speed_register(segment: u8) -> Option<u16> {
    match segment {
        1 => Some(P13_SEG1_SPEED),
        2 => Some(P13_SEG2_SPEED),
        3 => Some(P13_SEG3_SPEED),
        4 => Some(P13_SEG4_SPEED),
        5 => Some(P13_SEG5_SPEED),
        6 => Some(P13_SEG6_SPEED),
        7 => Some(P13_SEG7_SPEED),
        8 => Some(P13_SEG8_SPEED),
        9 => Some(P13_SEG9_SPEED),
        10 => Some(P13_SEG10_SPEED),
        11 => Some(P13_SEG11_SPEED),
        12 => Some(P13_SEG12_SPEED),
        13 => Some(P13_SEG13_SPEED),
        14 => Some(P13_SEG14_SPEED),
        15 => Some(P13_SEG15_SPEED),
        16 => Some(P13_SEG16_SPEED),
        _ => None,
    }
}

/// Get the segment accel/decel register for a given segment (1-16)
pub const fn get_segment_accel_decel_register(segment: u8) -> Option<u16> {
    match segment {
        1 => Some(P13_SEG1_ACCEL_DECEL),
        2 => Some(P13_SEG2_ACCEL_DECEL),
        3 => Some(P13_SEG3_ACCEL_DECEL),
        4 => Some(P13_SEG4_ACCEL_DECEL),
        5 => Some(P13_SEG5_ACCEL_DECEL),
        6 => Some(P13_SEG6_ACCEL_DECEL),
        7 => Some(P13_SEG7_ACCEL_DECEL),
        8 => Some(P13_SEG8_ACCEL_DECEL),
        9 => Some(P13_SEG9_ACCEL_DECEL),
        10 => Some(P13_SEG10_ACCEL_DECEL),
        11 => Some(P13_SEG11_ACCEL_DECEL),
        12 => Some(P13_SEG12_ACCEL_DECEL),
        13 => Some(P13_SEG13_ACCEL_DECEL),
        14 => Some(P13_SEG14_ACCEL_DECEL),
        15 => Some(P13_SEG15_ACCEL_DECEL),
        16 => Some(P13_SEG16_ACCEL_DECEL),
        _ => None,
    }
}

/// Get the segment wait time register for a given segment (1-16)
pub const fn get_segment_wait_time_register(segment: u8) -> Option<u16> {
    match segment {
        1 => Some(P13_SEG1_WAIT_TIME),
        2 => Some(P13_SEG2_WAIT_TIME),
        3 => Some(P13_SEG3_WAIT_TIME),
        4 => Some(P13_SEG4_WAIT_TIME),
        5 => Some(P13_SEG5_WAIT_TIME),
        6 => Some(P13_SEG6_WAIT_TIME),
        7 => Some(P13_SEG7_WAIT_TIME),
        8 => Some(P13_SEG8_WAIT_TIME),
        9 => Some(P13_SEG9_WAIT_TIME),
        10 => Some(P13_SEG10_WAIT_TIME),
        11 => Some(P13_SEG11_WAIT_TIME),
        12 => Some(P13_SEG12_WAIT_TIME),
        13 => Some(P13_SEG13_WAIT_TIME),
        14 => Some(P13_SEG14_WAIT_TIME),
        15 => Some(P13_SEG15_WAIT_TIME),
        16 => Some(P13_SEG16_WAIT_TIME),
        _ => None,
    }
}

/// Get the DI function register for a given input (1-3)
pub const fn get_di_function_register(input: u8) -> Option<u16> {
    match input {
        1 => Some(P02_DI1_FUNCTION),
        2 => Some(P02_DI2_FUNCTION),
        3 => Some(P02_DI3_FUNCTION),
        _ => None,
    }
}

/// Get the DI logic register for a given input (1-3)
pub const fn get_di_logic_register(input: u8) -> Option<u16> {
    match input {
        1 => Some(P02_DI1_LOGIC),
        2 => Some(P02_DI2_LOGIC),
        3 => Some(P02_DI3_LOGIC),
        _ => None,
    }
}

/// Get the DO function register for a given output (1-2)
pub const fn get_do_function_register(output: u8) -> Option<u16> {
    match output {
        1 => Some(P02_DO1_FUNCTION),
        2 => Some(P02_DO2_FUNCTION),
        _ => None,
    }
}

/// Get the DO logic register for a given output (1-2)
pub const fn get_do_logic_register(output: u8) -> Option<u16> {
    match output {
        1 => Some(P02_DO1_LOGIC),
        2 => Some(P02_DO2_LOGIC),
        _ => None,
    }
}
