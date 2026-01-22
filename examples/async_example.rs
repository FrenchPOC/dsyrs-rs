//! Async example for DSY-RS servo drive controller
//!
//! This example demonstrates:
//! - Connecting to a servo via Modbus RTU
//! - Configuring basic parameters
//! - Setting control mode and direction
//! - Reading status feedback
//! - Homing operation
//!
//! Run with: cargo run --example async_example

use dsyrs::{
    DsyrsClient, ServoConfig, ControlMode, Direction, HomingMode, HomingConfig, ServoState,
};
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("DSY-RS Async Example");
    println!("====================\n");

    // Serial port configuration
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115200;
    let slave_id = 1;

    println!("Connecting to {} at {} baud, slave ID {}...", port_name, baud_rate, slave_id);

    // Open serial port with timeout
    let builder = tokio_serial::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100));
    let port = SerialStream::open(&builder)?;

    // Create Modbus RTU context
    let ctx = rtu::attach_slave(port, Slave::from(slave_id));

    // Create servo configuration
    // - Position control mode
    // - CW forward rotation direction
    // - Max speed 3000 rpm
    // - Rated current 3.0 A
    // Note: rated_current, encoder_type, encoder_resolution, and motor_model_code are optional.
    // If not specified, they will be read from the servo during init().
    let config = ServoConfig::new(slave_id)
        .with_control_mode(ControlMode::Position)
        .with_direction(Direction::CwForward)
        .with_max_speed(3000)
        .with_rated_current(3.0);

    // Create and initialize client
    let mut servo = DsyrsClient::new(ctx, config);
    println!("Initializing servo drive...");
    servo.init().await?;

    // Read and display servo status
    println!("\n--- Servo Status ---");
    let status = servo.get_status().await?;
    println!("State: {:?}", status.state);
    println!("Speed: {} rpm", status.speed);
    println!("Position: {} pulses", status.position);
    println!("Torque: {}% of rated", status.torque as f32 * 0.1);
    println!("Current: {} A", status.current as f32 * 0.01);
    println!("Bus Voltage: {} V", status.bus_voltage as f32 * 0.1);

    // Read firmware version
    let sw_version = servo.get_software_version().await?;
    let fpga_version = servo.get_fpga_version().await?;
    println!("\nFirmware Version: {}", sw_version);
    println!("FPGA Version: {}", fpga_version);

    // Example: Configure homing
    println!("\n--- Homing Configuration ---");
    let homing_config = HomingConfig {
        mode: HomingMode::Mode4,          // Mode 4: Forward + limit switch
        high_speed: 500,                  // Search speed: 500 rpm
        low_speed: 100,                   // Creep speed: 100 rpm
        accel_limit: 200,                 // Acceleration: 200 ms
        timeout: 30000,                   // Timeout: 30 seconds
        offset: 0,                        // No offset after homing
    };
    servo.apply_homing_config(&homing_config).await?;
    println!("Homing configured: mode={:?}, high_speed={} rpm", 
             homing_config.mode, homing_config.high_speed);

    // Example: Set acceleration/deceleration times for speed control
    println!("\n--- Speed Control Parameters ---");
    servo.set_accel_time(500).await?;  // 500 ms acceleration
    servo.set_decel_time(500).await?;  // 500 ms deceleration
    servo.set_forward_speed_limit(3000).await?;  // Forward limit: 3000 rpm
    servo.set_backward_speed_limit(3000).await?; // Backward limit: 3000 rpm
    println!("Accel/Decel: 500 ms, Speed limits: ±3000 rpm");

    // Check if servo is ready
    let state = servo.get_servo_state().await?;
    match state {
        ServoState::Ready => println!("\n✓ Servo is ready for operation"),
        ServoState::Running => println!("\n⚡ Servo is currently running"),
        ServoState::Error | ServoState::Alarm => {
            println!("\n✗ Servo has error/alarm!");
            // Try to reset fault
            println!("Attempting fault reset...");
            servo.reset_fault().await?;
        }
        _ => println!("\nServo state: {:?}", state),
    }

    // Example: Read current control mode
    let mode = servo.get_control_mode().await?;
    println!("\nCurrent control mode: {:?}", mode);

    // Example: Speed control demonstration
    if mode == ControlMode::Speed {
        println!("\n--- Speed Control Demo ---");
        println!("Setting speed to 100 rpm...");
        servo.set_speed_command(100).await?;
        
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        let current_speed = servo.get_speed().await?;
        println!("Current speed: {} rpm", current_speed);
        
        println!("Stopping...");
        servo.set_speed_command(0).await?;
    }

    // Save parameters to EEPROM (optional)
    // println!("\nSaving parameters to EEPROM...");
    // servo.save_to_eeprom().await?;
    // println!("Parameters saved!");

    println!("\n✓ Async example completed successfully!");
    Ok(())
}
