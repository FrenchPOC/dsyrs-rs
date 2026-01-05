# DSY-RS Servo Drive Controller Library

A Rust library for controlling DSY-RS series low voltage servo drives via Modbus RTU protocol.

Based on **DSY-RS Series Low Voltage Servo Drive User Manual - Chapter 7 Parameters**.

## Features

- **Async and Sync APIs** - Choose between tokio-based async or blocking synchronous interfaces
- **Complete Parameter Access** - All P00-P18 parameter groups with proper addressing
- **Control Modes** - Position, Speed, and Torque control
- **Multi-Segment Positioning** - Configure up to 16 positioning segments
- **Homing Operations** - 18 different homing modes
- **Digital I/O** - Configure DI1-DI3 inputs and DO1-DO2 outputs
- **Real-time Status** - Read speed, position, torque, current, voltage
- **Multi-Servo Support** - Control multiple servos on the same RS-485 bus

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dsyrs = { path = "path/to/dsyrs-rs" }
tokio = { version = "1", features = ["full"] }
tokio-serial = "5.4"
tokio-modbus = "0.17"
```

## Quick Start

### Async Example

```rust
use dsyrs::{DsyrsClient, ServoConfig, ControlMode, Direction};
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open serial port
    let builder = tokio_serial::new("/dev/ttyUSB0", 115200)
        .timeout(Duration::from_millis(100));
    let port = SerialStream::open(&builder)?;
    
    // Create Modbus RTU context
    let ctx = rtu::attach_slave(port, Slave::from(1));
    
    // Configure servo
    let config = ServoConfig::new(1)
        .with_control_mode(ControlMode::Position)
        .with_direction(Direction::Forward)
        .with_max_speed(3000);
    
    // Create and initialize client
    let mut servo = DsyrsClient::new(ctx, config);
    servo.init().await?;
    
    // Read status
    let status = servo.get_status().await?;
    println!("Speed: {} rpm, Position: {}", status.speed, status.position);
    
    Ok(())
}
```

### Sync Example

```rust
use dsyrs::{DsyrsSyncClient, ServoConfig, ControlMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServoConfig::new(1)
        .with_control_mode(ControlMode::Speed);
    
    let mut servo = DsyrsSyncClient::connect("/dev/ttyUSB0", 115200, config)?;
    servo.init()?;
    
    // Set speed and read feedback
    servo.set_speed_command(1000)?;
    println!("Current speed: {} rpm", servo.get_speed()?);
    
    Ok(())
}
```

## Parameter Groups

| Group | Description | Address Range |
|-------|-------------|---------------|
| P00 | Basic control parameters | 0x0000-0x00FF |
| P01 | Servo motor parameters | 0x0100-0x01FF |
| P02 | Digital I/O configuration | 0x0200-0x02FF |
| P04 | Position control | 0x0400-0x04FF |
| P05 | Speed control | 0x0500-0x05FF |
| P06 | Torque control | 0x0600-0x06FF |
| P07 | Gain parameters | 0x0700-0x07FF |
| P08 | Advanced parameters | 0x0800-0x08FF |
| P09 | Protection parameters | 0x0900-0x09FF |
| P10 | Communication parameters | 0x0A00-0x0AFF |
| P11 | Auxiliary functions | 0x0B00-0x0BFF |
| P12 | Display parameters | 0x0C00-0x0CFF |
| P13 | Multi-segment position | 0x0D00-0x0DFF |
| P14 | Multi-speed control | 0x0E00-0x0EFF |
| P16 | Special functions (homing) | 0x1000-0x10FF |
| P18 | Status monitoring (read-only) | 0x1200-0x12FF |

## Register Addressing

Parameters are addressed as **PXX.YY** where:
- **XX** = Parameter group (00-18)
- **YY** = Parameter number within group
- **Modbus address** = XX × 256 + YY

Example: P18.01 (speed feedback) = 18 × 256 + 1 = 0x1201

## Control Modes

```rust
use dsyrs::ControlMode;

// Position control with pulse input
servo.set_control_mode(ControlMode::Position).await?;

// Speed control via Modbus
servo.set_control_mode(ControlMode::Speed).await?;

// Torque control
servo.set_control_mode(ControlMode::Torque).await?;
```

## Multi-Segment Positioning

Configure up to 16 position segments for automated motion sequences:

```rust
use dsyrs::{SegmentConfig, MultiSegOperationMode, MultiSegPositionMode};

// Configure segment 1
let segment = SegmentConfig {
    segment: 1,
    displacement: 10000,     // 10000 pulses
    speed: 1000,             // 1000 rpm
    accel_decel_time: 100,   // 100 ms
    wait_time: 500,          // Wait 500 ms after completion
};
servo.configure_segment(&segment).await?;

// Set operation mode
servo.set_multi_seg_mode(MultiSegOperationMode::ContinuousCycle).await?;
servo.set_multi_seg_position_mode(MultiSegPositionMode::Relative).await?;
servo.set_multi_seg_start(1).await?;
servo.set_multi_seg_end(4).await?;
```

## Homing Operations

18 different homing modes available:

```rust
use dsyrs::{HomingConfig, HomingMode};

let homing = HomingConfig {
    mode: HomingMode::ForwardLimit,   // Mode 1: Forward to limit switch
    high_speed: 500,                  // Search speed (rpm)
    low_speed: 100,                   // Creep speed (rpm)
    accel_limit: 200,                 // Acceleration time (ms)
    timeout: 30000,                   // Timeout (ms)
    offset: 0,                        // Offset after homing
};
servo.apply_homing_config(&homing).await?;
```

### Homing Modes

| Mode | Description |
|------|-------------|
| 0 | No homing |
| 1 | Forward limit switch |
| 2 | Backward limit switch |
| 3 | Z-pulse forward |
| 4 | Z-pulse backward |
| 5-17 | Various combinations of limit + Z-pulse |

## Digital I/O

### Digital Inputs (DI1-DI3)

```rust
use dsyrs::{DiFunction, DiLogic};

// Configure DI1 as servo enable
servo.set_di_function(1, DiFunction::ServoEnable).await?;
servo.set_di_logic(1, DiLogic::NormallyOpen).await?;
```

### Digital Outputs (DO1-DO2)

```rust
use dsyrs::{DoFunction, DoLogic};

// Configure DO1 as servo ready signal
servo.set_do_function(1, DoFunction::ServoReady).await?;
servo.set_do_logic(1, DoLogic::NormallyOpen).await?;
```

## Status Monitoring

Read real-time servo status from P18 registers:

```rust
let status = servo.get_status().await?;
println!("State: {:?}", status.state);
println!("Speed: {} rpm", status.speed);
println!("Position: {} pulses", status.position);
println!("Torque: {}% of rated", status.torque as f32 * 0.1);
println!("Current: {} A", status.current as f32 * 0.01);
println!("Bus Voltage: {} V", status.bus_voltage as f32 * 0.1);
```

## Communication Settings

Default Modbus RTU settings:
- **Baud rate**: 115200 (configurable: 2400-115200)
- **Data format**: 8N1 (8 data bits, no parity, 1 stop bit)
- **Slave ID**: 1-247

```rust
use dsyrs::{CommConfig, BaudRate, DataFormat, AddressSource};

let comm_config = CommConfig {
    address: 1,
    baud_rate: BaudRate::Baud115200,
    data_format: DataFormat::N81,  // 8N1
    address_source: AddressSource::FromP10_00,
};
servo.apply_comm_config(&comm_config).await?;
```

## Error Handling

```rust
use dsyrs::{DsyrsError, ServoState};

match servo.get_servo_state().await? {
    ServoState::Fault => {
        println!("Servo fault detected!");
        servo.reset_fault().await?;
    }
    ServoState::Ready => println!("Servo ready"),
    ServoState::Running => println!("Servo running"),
    _ => {}
}
```

## Examples

Run examples with:

```bash
# Async example
cargo run --example async_example

# Sync example
cargo run --example sync_example

# Multiple servos
cargo run --example multiple_servos
```

## Auxiliary Functions

```rust
// Reset fault
servo.reset_fault().await?;

// Emergency stop
servo.emergency_stop().await?;

// Clear emergency stop
servo.clear_emergency_stop().await?;

// Save to EEPROM
servo.save_to_eeprom().await?;

// Factory reset
servo.factory_reset().await?;
```

## Dependencies

- `tokio` - Async runtime
- `tokio-modbus` - Modbus RTU client
- `tokio-serial` - Serial port handling
- `thiserror` - Error handling

## License

See LICENSE file.

## References

- DSY-RS Series Low Voltage Servo Drive User Manual - Chapter 7 Parameters
- Modbus RTU Protocol Specification
