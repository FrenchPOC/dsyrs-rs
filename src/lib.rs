//! DSY-RS Low Voltage Servo Drive Controller Library
//!
//! This library provides both async and sync interfaces for controlling
//! DSY-RS series low voltage servo drives via Modbus RTU protocol.
//! Based on DSY-RS Series User Manual - Chapter 7 Parameters.
//!
//! # Features
//! - Async API using tokio-modbus
//! - Synchronous wrapper for blocking contexts  
//! - Support for multiple servo instances on the same bus
//! - Complete parameter access based on official documentation
//! - Control modes: Position, Speed, Torque
//! - Multi-segment positioning (16 segments)
//! - Homing routines
//! - Digital I/O configuration
//! - Real-time status monitoring
//!
//! # Register Addressing
//! Parameters are addressed as PXX.YY where:
//! - XX = Parameter group (00-18)
//! - YY = Parameter number within group
//! - Modbus address = XX × 256 + YY (e.g., P18.01 = 0x1201)
//!
//! # Parameter Groups
//! - P00: Basic control parameters
//! - P01: Servo motor parameters
//! - P02: Digital I/O configuration
//! - P04: Position control
//! - P05: Speed control
//! - P06: Torque control
//! - P07: Gain parameters
//! - P08: Advanced parameters
//! - P09: Protection parameters
//! - P10: Communication parameters
//! - P11: Auxiliary functions
//! - P12: Display parameters
//! - P13: Multi-segment position control
//! - P14: Multi-speed control
//! - P16: Special functions (homing)
//! - P18: Status monitoring (read-only)
//!
//! # Examples
//!
//! ## Async Usage
//! ```no_run
//! use dsyrs::{DsyrsClient, ServoConfig, ControlMode, Direction};
//! use tokio_modbus::prelude::*;
//! use tokio_serial::SerialStream;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize serial port at 115200 baud
//!     let builder = tokio_serial::new("/dev/ttyUSB0", 115200);
//!     let port = SerialStream::open(&builder)?;
//!     
//!     // Create Modbus RTU context with slave ID 1
//!     let ctx = rtu::attach_slave(port, Slave::from(1));
//!     
//!     // Create servo configuration
//!     let config = ServoConfig::new(1)
//!         .with_control_mode(ControlMode::Position)
//!         .with_direction(Direction::Forward)
//!         .with_max_speed(3000);
//!     
//!     // Create and initialize client
//!     let mut client = DsyrsClient::new(ctx, config);
//!     client.init().await?;
//!     
//!     // Read status
//!     let status = client.get_status().await?;
//!     println!("Speed: {} rpm", status.speed);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Sync Usage
//! ```no_run
//! use dsyrs::{DsyrsSyncClient, ServoConfig, ControlMode};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ServoConfig::new(1)
//!         .with_control_mode(ControlMode::Speed);
//!     let mut servo = DsyrsSyncClient::connect("/dev/ttyUSB0", 115200, config)?;
//!     servo.init()?;
//!     
//!     // Set speed and read feedback
//!     servo.set_speed_command(1000)?;
//!     println!("Current speed: {} rpm", servo.get_speed()?);
//!     
//!     Ok(())
//! }
//! ```

pub mod registers;
pub mod types;
pub mod client;
pub mod sync;

// Re-export main types
pub use client::DsyrsClient;
pub use sync::DsyrsSyncClient;
pub use types::*;

// Re-export registers for direct access
pub use registers::*;
