//! Type definitions for DSY-RS servo drive controller
//!
//! Contains error types, enums, and configuration structs based on
//! DSY-RS Series Low Voltage Servo Drive User Manual - Chapter 7 Parameters.

use thiserror::Error;
use tokio_modbus::ExceptionCode;

/// Error types for DSY-RS operations
#[derive(Error, Debug)]
pub enum DsyrsError {
    #[error("Modbus communication error: {0}")]
    Modbus(#[from] std::io::Error),

    #[error("Modbus protocol error: {0}")]
    ModbusProtocol(#[from] tokio_modbus::Error),

    #[error("Modbus exception: {0:?}")]
    ModbusException(#[from] ExceptionCode),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Invalid segment ID: {0}. Must be 1-16")]
    InvalidSegment(u8),

    #[error("Invalid digital input: {0}. Must be 1-3")]
    InvalidDigitalInput(u8),

    #[error("Invalid digital output: {0}. Must be 1-2")]
    InvalidDigitalOutput(u8),

    #[error("Operation failed: {0}")]
    OperationFailed(String),

    #[error("Servo not ready")]
    ServoNotReady,

    #[error("Timeout waiting for operation")]
    Timeout,

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Serial port error: {0}")]
    SerialError(String),
}

pub type Result<T> = std::result::Result<T, DsyrsError>;

// ============================================================================
// P00 - Basic Control Enums
// ============================================================================

/// Control mode selection (P00.00)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum ControlMode {
    /// Position control mode
    #[default]
    Position = 0,
    /// Speed control mode
    Speed = 1,
    /// Torque control mode
    Torque = 2,
}

impl From<ControlMode> for u16 {
    fn from(mode: ControlMode) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for ControlMode {
    type Error = DsyrsError;
    fn try_from(value: u16) -> Result<Self> {
        match value {
            0 => Ok(ControlMode::Position),
            1 => Ok(ControlMode::Speed),
            2 => Ok(ControlMode::Torque),
            _ => Err(DsyrsError::InvalidParameter(format!(
                "Invalid control mode: {}",
                value
            ))),
        }
    }
}

/// Motor rotation direction (P00.01)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum Direction {
    /// Counter-clockwise is forward
    #[default]
    CcwForward = 0,
    /// Clockwise is forward
    CwForward = 1,
}

impl From<Direction> for u16 {
    fn from(dir: Direction) -> Self {
        dir as u16
    }
}

/// Absolute value system selection (P00.06)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum AbsoluteSystem {
    /// Incremental position
    #[default]
    Incremental = 0,
    /// Absolute position (linear)
    AbsoluteLinear = 1,
    /// Absolute position (rotation)
    AbsoluteRotation = 2,
}

impl From<AbsoluteSystem> for u16 {
    fn from(sys: AbsoluteSystem) -> Self {
        sys as u16
    }
}

/// Servo OFF stop mode (P00.10)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum ServoOffStopMode {
    /// Freewheel stop
    Freewheel = 0,
    /// Stop at zero speed (deceleration by P05.06)
    #[default]
    ZeroSpeed = 1,
}

impl From<ServoOffStopMode> for u16 {
    fn from(mode: ServoOffStopMode) -> Self {
        mode as u16
    }
}

/// Overtravel stop mode (P00.13)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum OvertravelStopMode {
    /// Freewheel
    Freewheel = 0,
    /// Decelerate then servo-lock
    #[default]
    DecelThenLock = 1,
    /// Decelerate then freewheel
    DecelThenFreewheel = 2,
}

impl From<OvertravelStopMode> for u16 {
    fn from(mode: OvertravelStopMode) -> Self {
        mode as u16
    }
}

/// Energy consumption resistor setting (P00.18)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum EnergyResistor {
    /// Built-in resistor
    #[default]
    BuiltIn = 0,
    /// External, natural cooling
    ExternalNatural = 1,
    /// External, forced air cooling
    ExternalForced = 2,
    /// None (capacitance absorption)
    None = 3,
}

impl From<EnergyResistor> for u16 {
    fn from(res: EnergyResistor) -> Self {
        res as u16
    }
}

// ============================================================================
// P01 - Servo Motor Parameter Enums
// ============================================================================

/// Encoder selection (P01.18)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum EncoderType {
    /// 2500-line encoder
    Line2500 = 0,
    /// 17-bit incremental encoder
    #[default]
    Bit17Incremental = 1,
    /// 17-bit absolute encoder
    Bit17Absolute = 2,
    /// 23-bit incremental encoder
    Bit23Incremental = 3,
    /// 23-bit absolute encoder
    Bit23Absolute = 4,
}

impl From<EncoderType> for u16 {
    fn from(enc: EncoderType) -> Self {
        enc as u16
    }
}

// ============================================================================
// P02 - Digital I/O Parameter Enums
// ============================================================================

/// Digital input function selection (P02.01-P02.03)
/// Values 1-45 correspond to FunIN.1-45
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum DiFunction {
    /// No function assigned
    #[default]
    None = 0,
    /// Servo enable (FunIN.1)
    ServoEnable = 1,
    /// Alarm reset signal (edge valid function) (FunIN.2)
    AlarmResetSignal = 2,
    /// Proportional action switching/gain switching (FunIN.3)
    ProportionalActionSwitch = 3,
    /// Main/auxiliary running command switching (FunIN.4)
    MainAuxiliaryCommandSwitch = 4,
    /// Pulse deviation clearing (FunIN.5)
    PulseDeviationClear = 5,
    /// Multi-segment running command switch CMD1 (FunIN.6)
    MultiSegCommandSwitch1 = 6,
    /// Multi-segment running command switch CMD2 (FunIN.7)
    MultiSegCommandSwitch2 = 7,
    /// Multi-segment running command switch CMD3 (FunIN.8)
    MultiSegCommandSwitch3 = 8,
    /// Multi-segment running command switch CMD4 (FunIN.9)
    MultiSegCommandSwitch4 = 9,
    /// P-Mode switching (FunIN.10)
    PModeSwitch = 10,
    /// Zero fixed function enable signal (FunIN.11)
    ZeroFixedEnable = 11,
    /// Pulse prohibition (FunIN.12)
    PulseProhibition = 12,
    /// Forward overtravel (FunIN.13)
    ForwardOvertravel = 13,
    /// Backward overtravel (FunIN.14)
    BackwardOvertravel = 14,
    /// Forward external torque limit ON (FunIN.15)
    ForwardExternalTorqueLimit = 15,
    /// Backward external torque limit ON (FunIN.16)
    BackwardExternalTorqueLimit = 16,
    /// Forward jog (FunIN.17)
    ForwardJog2 = 17,
    /// Backward jog (FunIN.18)
    BackwardJog = 18,
    /// Position step input DI variable (FunIN.19)
    PositionStepInputDI = 19,
    /// Handwheel magnification signal 1 (reserved) (FunIN.20)
    HandwheelMagnification1 = 20,
    /// Handwheel magnification signal 2 (reserved) (FunIN.21)
    HandwheelMagnification2 = 21,
    /// Handwheel enable signal (reserved) (FunIN.22)
    HandwheelEnable = 22,
    /// Electronic gear selection (FunIN.23)
    ElectronicGearSelection = 23,
    /// Position instruction reverse (FunIN.24)
    PositionInstructionReverse = 24,
    /// Speed command reverse (FunIN.25)
    SpeedCommandReverse = 25,
    /// Torque command reverse (FunIN.26)
    TorqueCommandReverse = 26,
    /// Handwheel A signal (reserved) (FunIN.27)
    HandwheelSignalA = 27,
    /// Handwheel B signal (reserved) (FunIN.28)
    HandwheelSignalB = 28,
    /// Internal multi-segment position enable signal (FunIN.29)
    InternalMultiSegmentPositionEnable = 29,
    /// Interrupt fixed length completion external confirmation signal (FunIN.30)
    InterruptFixedLengthCompletionExtConfirm = 30,
    /// Interrupt fixed length prohibition (FunIN.31)
    InterruptFixedLengthProhibition = 31,
    /// Home switch signal (FunIN.32)
    HomeSwitchSignal = 32,
    /// Homing enable signal (FunIN.33)
    HomingEnableSignal = 33,
    /// Emergency stop (FunIN.34)
    EmergencyStop = 34,
    /// Position loop constant speed running (FunIN.35)
    PositionLoopConstantSpeedRunning = 35,
    /// Interrupt fixed length reset (FunIN.36)
    InterruptFixedLengthReset = 36,
    /// Interrupt fixed length operation pause (FunIN.37)
    InterruptFixedLengthOperationPause = 37,
    /// Multi-segment torque running command switching 1 (FunIN.38)
    MultiSegmentTorqueCommandSwitch1 = 38,
    /// Multi-step torque running command switching 1 (FunIN.39)
    MultiStepTorqueCommandSwitch1 = 39,
    /// Speed Mode A1 command direction switching 1 (reserved) (FunIN.40)
    SpeedModeA1SW1 = 40,
    /// Speed Mode A1 command direction switching 2 (reserved) (FunIN.41)
    SpeedModeA1SW2 = 41,
}

impl From<DiFunction> for u16 {
    fn from(func: DiFunction) -> Self {
        func as u16
    }
}

/// Digital input logic selection (P02.11-P02.13)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum DiLogic {
    /// Low level active
    #[default]
    LowActive = 0,
    /// High level active
    HighActive = 1,
    /// Rising edge triggered
    RisingEdge = 2,
    /// Falling edge triggered
    FallingEdge = 3,
    /// Both edges triggered
    BothEdges = 4,
}

impl From<DiLogic> for u16 {
    fn from(logic: DiLogic) -> Self {
        logic as u16
    }
}

/// Digital output function selection (P02.21-P02.22)
/// Values 1-25 correspond to FunOUT.1-25
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum DoFunction {
    /// No function assigned
    #[default]
    None = 0,
    /// Servo ready (FunOUT.1)
    ServoReady = 1,
    /// Fault output signal (FunOUT.2)
    FaultOutputSignal = 2,
    /// Warning output signal (FunOUT.3)
    WarningOutputSignal = 3,
    /// Motor rotation output signal (FunOUT.4)
    MotorRotationOutputSignal = 4,
    /// Zero speed signal (FunOUT.5)
    ZeroSpeedSignal = 5,
    /// Speed consistent (FunOUT.6)
    SpeedConsistent = 6,
    /// Position completed (FunOUT.7)
    PositionCompleted = 7,
    /// Positioning approach signal (FunOUT.8)
    PositioningApproachSignal = 8,
    /// Torque limit signal (FunOUT.9)
    TorqueLimitSignal = 9,
    /// Speed limit signal (FunOUT.10)
    SpeedLimitSignal = 10,
    /// Brake release signal output (FunOUT.11)
    BrakeReleaseSignalOutput = 11,
    /// Torque feedback reaches specified range (FunOUT.12)
    TorqueFeedbackReachesRange = 12,
    /// Speed feedback reaches specified range (FunOUT.13)
    SpeedFeedbackReachesRange = 13,
    /// Angle recognition completed (FunOUT.14)
    AngleRecognitionCompleted = 14,
    /// Output 3-bit alarm code (reserved) (FunOUT.15)
    OutputAlarmCode1 = 15,
    /// Output 3-bit alarm code (reserved) (FunOUT.16)
    OutputAlarmCode2 = 16,
    /// Output 3-bit alarm code (reserved) (FunOUT.17)
    OutputAlarmCode3 = 17,
    /// Interrupt fixed length completion signal (FunOUT.18)
    InterruptFixedLengthCompletionSignal = 18,
    /// Homing completion signal (FunOUT.19)
    HomingCompletionSignal = 19,
    /// Reserved (FunOUT.20)
    Reserved20 = 20,
    /// Multi-segment position completion command 1 output (FunOUT.21)
    MultiSegmentPositionCompletion1 = 21,
    /// Multi-segment position completion command 2 output (FunOUT.22)
    MultiSegmentPositionCompletion2 = 22,
    /// Multi-segment position completion command 3 output (FunOUT.23)
    MultiSegmentPositionCompletion3 = 23,
    /// Multi-segment position completion command 4 output (FunOUT.24)
    MultiSegmentPositionCompletion4 = 24,
}

impl From<DoFunction> for u16 {
    fn from(func: DoFunction) -> Self {
        func as u16
    }
}

/// Digital output logic (P02.31-P02.32)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum DoLogic {
    /// Normally open (conduct when active)
    #[default]
    NormallyOpen = 0,
    /// Normally closed (open when active)
    NormallyClosed = 1,
}

impl From<DoLogic> for u16 {
    fn from(logic: DoLogic) -> Self {
        logic as u16
    }
}

// ============================================================================
// P04 - Position Control Parameter Enums
// ============================================================================

/// Position command source (P04.00)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum PositionCmdSource {
    /// Low-speed pulse input
    #[default]
    LowSpeedPulse = 0,
    /// High-speed pulse input
    HighSpeedPulse = 1,
    /// Step amount (P04.02)
    StepAmount = 2,
    /// Multi-segment position
    MultiSegment = 4,
    /// Communication command
    Communication = 5,
}

impl From<PositionCmdSource> for u16 {
    fn from(src: PositionCmdSource) -> Self {
        src as u16
    }
}

/// Pulse shape (P04.21)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum PulseShape {
    /// Pulse + Direction, positive logic
    #[default]
    PulseDirPos = 0,
    /// Direction + Pulse, negative logic
    DirPulseNeg = 1,
    /// A/B quadrature, positive logic
    QuadPos = 2,
    /// A/B quadrature, negative logic
    QuadNeg = 3,
    /// CCW/CW pulse, positive logic
    CcwCwPos = 4,
    /// CCW/CW pulse, negative logic
    CcwCwNeg = 5,
}

impl From<PulseShape> for u16 {
    fn from(shape: PulseShape) -> Self {
        shape as u16
    }
}

/// Position deviation clear mode (P04.22)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum DeviationClearMode {
    /// Clear on fault or servo OFF
    #[default]
    OnFaultOrOff = 0,
    /// Clear only on fault
    OnFault = 1,
    /// Clear by DI (PERR-CLR)
    ByDi = 2,
}

impl From<DeviationClearMode> for u16 {
    fn from(mode: DeviationClearMode) -> Self {
        mode as u16
    }
}

// ============================================================================
// P10 - Communication Parameter Enums
// ============================================================================

/// Modbus baud rate setting (P10.02)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum BaudRate {
    /// 2400 bps
    Baud2400 = 0,
    /// 4800 bps
    Baud4800 = 1,
    /// 9600 bps
    Baud9600 = 2,
    /// 19200 bps
    Baud19200 = 3,
    /// 38400 bps
    Baud38400 = 4,
    /// 57600 bps
    Baud57600 = 5,
    /// 115200 bps
    #[default]
    Baud115200 = 6,
}

impl From<BaudRate> for u16 {
    fn from(br: BaudRate) -> Self {
        br as u16
    }
}

impl BaudRate {
    /// Get the actual baud rate value
    pub fn to_bps(self) -> u32 {
        match self {
            BaudRate::Baud2400 => 2400,
            BaudRate::Baud4800 => 4800,
            BaudRate::Baud9600 => 9600,
            BaudRate::Baud19200 => 19200,
            BaudRate::Baud38400 => 38400,
            BaudRate::Baud57600 => 57600,
            BaudRate::Baud115200 => 115200,
        }
    }
}

/// Modbus data format (P10.03)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum DataFormat {
    /// No parity, 2 stop bits
    #[default]
    NoParity2Stop = 0,
    /// Even parity, 1 stop bit
    EvenParity1Stop = 1,
    /// Odd parity, 1 stop bit
    OddParity1Stop = 2,
    /// No parity, 1 stop bit
    NoParity1Stop = 3,
}

impl From<DataFormat> for u16 {
    fn from(fmt: DataFormat) -> Self {
        fmt as u16
    }
}

/// RS485 address source (P10.06)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum AddressSource {
    /// Use DIP switch setting
    #[default]
    DipSwitch = 0,
    /// Use host setting (P10.00)
    HostSetting = 1,
}

impl From<AddressSource> for u16 {
    fn from(src: AddressSource) -> Self {
        src as u16
    }
}

// ============================================================================
// P11 - Auxiliary Function Parameter Enums
// ============================================================================

/// System initialization command (P11.09)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum SystemInit {
    /// No action
    #[default]
    None = 0,
    /// Factory reset (except P01 & P17)
    FactoryReset = 1,
    /// Clear fault record
    ClearFaultRecord = 2,
}

impl From<SystemInit> for u16 {
    fn from(init: SystemInit) -> Self {
        init as u16
    }
}

/// Absolute encoder reset command (P11.06)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum EncoderReset {
    /// No action
    #[default]
    None = 0,
    /// Clear warnings/errors
    ClearWarnings = 1,
    /// Reset multi-turn data
    ResetMultiTurn = 2,
}

impl From<EncoderReset> for u16 {
    fn from(reset: EncoderReset) -> Self {
        reset as u16
    }
}

// ============================================================================
// P13 - Multi-Segment Position Parameter Enums
// ============================================================================

/// Multi-segment operation mode (P13.00)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum MultiSegOperationMode {
    /// Single execution
    Single = 0,
    /// Cycle execution
    #[default]
    Cycle = 1,
    /// DI switch selection
    DiSwitch = 2,
}

impl From<MultiSegOperationMode> for u16 {
    fn from(mode: MultiSegOperationMode) -> Self {
        mode as u16
    }
}

/// Multi-segment position mode (P13.05)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum MultiSegPositionMode {
    /// Incremental positioning
    #[default]
    Incremental = 0,
    /// Absolute positioning
    Absolute = 1,
}

impl From<MultiSegPositionMode> for u16 {
    fn from(mode: MultiSegPositionMode) -> Self {
        mode as u16
    }
}

/// Wait time unit (P13.04)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum WaitTimeUnit {
    /// Milliseconds
    #[default]
    Milliseconds = 0,
    /// Seconds
    Seconds = 1,
}

impl From<WaitTimeUnit> for u16 {
    fn from(unit: WaitTimeUnit) -> Self {
        unit as u16
    }
}

// ============================================================================
// P16 - Special Function Parameter Enums
// ============================================================================

/// Homing mode (P16.09)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum HomingMode {
    /// Mode 0: Forward + limit switch + Z pulse
    #[default]
    Mode0 = 0,
    /// Mode 1: Reverse + limit switch + Z pulse
    Mode1 = 1,
    /// Mode 2: Forward + home switch + Z pulse
    Mode2 = 2,
    /// Mode 3: Reverse + home switch + Z pulse
    Mode3 = 3,
    /// Mode 4: Forward + limit switch
    Mode4 = 4,
    /// Mode 5: Reverse + limit switch
    Mode5 = 5,
    /// Mode 6: Forward + home switch
    Mode6 = 6,
    /// Mode 7: Reverse + home switch
    Mode7 = 7,
    /// Mode 8: Z pulse only (forward)
    Mode8 = 8,
    /// Mode 9: Z pulse only (reverse)
    Mode9 = 9,
    /// Mode 10: Current position as home
    Mode10 = 10,
    /// Mode 11-17: Additional modes (see manual)
    Mode11 = 11,
    Mode12 = 12,
    Mode13 = 13,
    Mode14 = 14,
    Mode15 = 15,
    Mode16 = 16,
    Mode17 = 17,
}

impl From<HomingMode> for u16 {
    fn from(mode: HomingMode) -> Self {
        mode as u16
    }
}

// ============================================================================
// P18 - Status Enums
// ============================================================================

/// Servo status (P18.00)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServoState {
    /// Servo ready
    Ready,
    /// Servo running
    Running,
    /// Servo error
    Error,
    /// Servo alarm
    Alarm,
    /// Unknown state
    Unknown(u16),
}

impl From<u16> for ServoState {
    fn from(value: u16) -> Self {
        // Status interpretation depends on actual hardware implementation
        // This is a simplified mapping
        match value & 0x0F {
            0 => ServoState::Ready,
            1 => ServoState::Running,
            2 => ServoState::Error,
            3 => ServoState::Alarm,
            _ => ServoState::Unknown(value),
        }
    }
}

// ============================================================================
// Configuration Structures
// ============================================================================

/// Servo drive configuration
#[derive(Debug, Clone)]
pub struct ServoConfig {
    /// Modbus slave ID (1-247)
    pub slave_id: u8,
    /// Control mode
    pub control_mode: ControlMode,
    /// Rotation direction
    pub direction: Direction,
    /// Maximum speed (rpm)
    pub max_speed: u16,
    /// Motor model code (P01.00) - read from servo if None
    pub motor_model_code: Option<u16>,
    /// Rated current (A) (P01.04) - read from servo if None
    pub rated_current: Option<f32>,
    /// Encoder type (P01.18) - read from servo if None
    pub encoder_type: Option<EncoderType>,
    /// Encoder resolution (P01.20) - read from servo if None
    pub encoder_resolution: Option<u32>,
}

impl ServoConfig {
    /// Create a new servo configuration with default values
    pub fn new(slave_id: u8) -> Self {
        Self {
            slave_id,
            control_mode: ControlMode::Position,
            direction: Direction::CcwForward,
            max_speed: 4500,
            motor_model_code: None,
            rated_current: None,
            encoder_type: None,
            encoder_resolution: None,
        }
    }

    /// Set control mode
    pub fn with_control_mode(mut self, mode: ControlMode) -> Self {
        self.control_mode = mode;
        self
    }

    /// Set direction
    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Set maximum speed
    pub fn with_max_speed(mut self, rpm: u16) -> Self {
        self.max_speed = rpm;
        self
    }

    /// Set motor model code
    pub fn with_motor_model_code(mut self, code: u16) -> Self {
        self.motor_model_code = Some(code);
        self
    }

    /// Set rated current
    pub fn with_rated_current(mut self, current: f32) -> Self {
        self.rated_current = Some(current);
        self
    }

    /// Set encoder type
    pub fn with_encoder_type(mut self, encoder: EncoderType) -> Self {
        self.encoder_type = Some(encoder);
        self
    }

    /// Set encoder resolution
    pub fn with_encoder_resolution(mut self, resolution: u32) -> Self {
        self.encoder_resolution = Some(resolution);
        self
    }
}

/// Multi-segment position configuration
#[derive(Debug, Clone)]
pub struct SegmentConfig {
    /// Segment number (1-16)
    pub segment: u8,
    /// Displacement (32-bit signed)
    pub displacement: i32,
    /// Maximum speed (rpm)
    pub speed: u16,
    /// Acceleration/deceleration time (ms)
    pub accel_decel_time: u16,
    /// Wait time after motion
    pub wait_time: u16,
}

impl SegmentConfig {
    /// Create a new segment configuration
    pub fn new(segment: u8) -> Result<Self> {
        if segment < 1 || segment > 16 {
            return Err(DsyrsError::InvalidSegment(segment));
        }
        Ok(Self {
            segment,
            displacement: 0,
            speed: 200,
            accel_decel_time: 50,
            wait_time: 0,
        })
    }

    /// Set displacement
    pub fn with_displacement(mut self, displacement: i32) -> Self {
        self.displacement = displacement;
        self
    }

    /// Set speed
    pub fn with_speed(mut self, rpm: u16) -> Self {
        self.speed = rpm;
        self
    }

    /// Set acceleration/deceleration time
    pub fn with_accel_decel(mut self, ms: u16) -> Self {
        self.accel_decel_time = ms;
        self
    }

    /// Set wait time
    pub fn with_wait_time(mut self, time: u16) -> Self {
        self.wait_time = time;
        self
    }
}

/// Homing configuration
#[derive(Debug, Clone)]
pub struct HomingConfig {
    /// Homing mode
    pub mode: HomingMode,
    /// High speed for searching (rpm)
    pub high_speed: u16,
    /// Low speed for precise positioning (rpm)
    pub low_speed: u16,
    /// Acceleration limit (ms)
    pub accel_limit: u16,
    /// Timeout (ms)
    pub timeout: u16,
    /// Home offset
    pub offset: i32,
}

impl Default for HomingConfig {
    fn default() -> Self {
        Self {
            mode: HomingMode::Mode0,
            high_speed: 100,
            low_speed: 10,
            accel_limit: 1000,
            timeout: 10000,
            offset: 0,
        }
    }
}

impl HomingConfig {
    /// Set homing mode
    pub fn with_mode(mut self, mode: HomingMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set high speed
    pub fn with_high_speed(mut self, rpm: u16) -> Self {
        self.high_speed = rpm;
        self
    }

    /// Set low speed
    pub fn with_low_speed(mut self, rpm: u16) -> Self {
        self.low_speed = rpm;
        self
    }

    /// Set acceleration limit
    pub fn with_accel_limit(mut self, ms: u16) -> Self {
        self.accel_limit = ms;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, ms: u16) -> Self {
        self.timeout = ms;
        self
    }

    /// Set home offset
    pub fn with_offset(mut self, offset: i32) -> Self {
        self.offset = offset;
        self
    }
}

/// Jog configuration
#[derive(Debug, Clone)]
pub struct JogConfig {
    /// Jog speed (rpm)
    pub speed: u16,
    /// Acceleration time (ms)
    pub accel_time: u16,
    /// Deceleration time (ms)
    pub decel_time: u16,
}

impl Default for JogConfig {
    fn default() -> Self {
        Self {
            speed: 200,
            accel_time: 50,
            decel_time: 50,
        }
    }
}

impl JogConfig {
    /// Set jog speed
    pub fn with_speed(mut self, rpm: u16) -> Self {
        self.speed = rpm;
        self
    }

    /// Set acceleration time
    pub fn with_accel(mut self, ms: u16) -> Self {
        self.accel_time = ms;
        self
    }

    /// Set deceleration time
    pub fn with_decel(mut self, ms: u16) -> Self {
        self.decel_time = ms;
        self
    }
}

/// Servo status information
#[derive(Debug, Clone)]
pub struct ServoStatus {
    /// Current servo state
    pub state: ServoState,
    /// Motor speed feedback (rpm)
    pub speed: i16,
    /// Average load rate (0.1%)
    pub load_rate: u16,
    /// Internal torque (0.1% of rated)
    pub torque: i16,
    /// Phase current RMS (0.01 A)
    pub current: u16,
    /// DC bus voltage (0.1 V)
    pub bus_voltage: u16,
    /// Absolute position
    pub position: i32,
    /// Electrical angle (0.1°)
    pub electrical_angle: u16,
}

/// Gain parameters for tuning
#[derive(Debug, Clone)]
pub struct GainParams {
    /// Position loop gain (0.1 Hz)
    pub position_gain: u16,
    /// Speed loop gain (0.1 Hz)
    pub speed_gain: u16,
    /// Speed loop integral time (0.01 ms)
    pub speed_integral: u16,
    /// Speed detection filter (0.01 ms)
    pub speed_filter: u16,
}

impl Default for GainParams {
    fn default() -> Self {
        Self {
            position_gain: 320,   // 32.0 Hz
            speed_gain: 180,      // 18.0 Hz
            speed_integral: 3100, // 31.0 ms
            speed_filter: 20,     // 0.2 ms
        }
    }
}

/// Communication configuration
#[derive(Debug, Clone)]
pub struct CommConfig {
    /// Slave address (0-247, 0=broadcast)
    pub address: u8,
    /// Baud rate
    pub baud_rate: BaudRate,
    /// Data format
    pub data_format: DataFormat,
    /// Address source
    pub address_source: AddressSource,
}

impl Default for CommConfig {
    fn default() -> Self {
        Self {
            address: 1,
            baud_rate: BaudRate::Baud115200,
            data_format: DataFormat::NoParity2Stop,
            address_source: AddressSource::DipSwitch,
        }
    }
}
