//! Synchronous example for DSY-RS servo drive controller
//!
//! This example demonstrates:
//! - Connecting using the synchronous client with tokio-modbus sync API
//! - Reading and displaying status
//! - Speed control operations
//! - Position control with multi-segment
//!
//! Run with: cargo run --example sync_example

use dsyrs::{
    DsyrsSyncClient, ServoConfig, ControlMode, Direction, JogConfig,
    SegmentConfig, MultiSegOperationMode, MultiSegPositionMode, ServoState,
    Slave,
};
use tokio_modbus::prelude::client;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("DSY-RS Synchronous Example");
    println!("==========================\n");

    // Connection parameters
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115200;
    let slave_id = 1;

    println!("Connecting to {} at {} baud, slave ID {}...", port_name, baud_rate, slave_id);

    // Create serial port builder
    let builder = tokio_serial::new(port_name, baud_rate);
    
    // Connect to slave via tokio-modbus sync API
    let ctx = client::sync::rtu::connect_slave(&builder, Slave::from(slave_id))?;

    // Create servo configuration for position control
    let config = ServoConfig::new(slave_id)
        .with_control_mode(ControlMode::Position)
        .with_direction(Direction::CwForward)
        .with_max_speed(3000)
        .with_rated_current(3.0);

    // Create synchronous client with context
    let mut servo = DsyrsSyncClient::new(ctx, config);
    
    // Initialize servo with configuration
    println!("Initializing servo drive...");
    servo.init()?;

    // Read and display status
    println!("\n--- Current Status ---");
    display_status(&mut servo)?;

    // Read version information
    println!("\n--- Version Information ---");
    let sw_ver = servo.get_software_version()?;
    let fpga_ver = servo.get_fpga_version()?;
    let product_code = servo.get_product_code()?;
    println!("Software Version: {}", sw_ver);
    println!("FPGA Version: {}", fpga_ver);
    println!("Product Code: {}", product_code);

    // Example: Configure jog parameters
    println!("\n--- Jog Configuration ---");
    let jog_config = JogConfig {
        speed: 500,       // 500 rpm
        accel_time: 200,  // 200 ms acceleration
        decel_time: 200,  // 200 ms deceleration
    };
    servo.apply_jog_config(&jog_config)?;
    println!("Jog speed: {} rpm, Accel/Decel: {} ms", jog_config.speed, jog_config.accel_time);

    // Example: Configure multi-segment positioning
    println!("\n--- Multi-Segment Position Configuration ---");
    
    // Set up multi-segment mode
    servo.set_multi_seg_mode(MultiSegOperationMode::Cycle)?;
    servo.set_multi_seg_position_mode(MultiSegPositionMode::Incremental)?;
    servo.set_multi_seg_start(1)?;
    servo.set_multi_seg_end(3)?;
    
    // Configure segment 1: Move 10000 pulses at 1000 rpm
    let segment1 = SegmentConfig {
        segment: 1,
        displacement: 10000,
        speed: 1000,
        accel_decel_time: 100,
        wait_time: 500,  // Wait 500 ms after completion
    };
    servo.configure_segment(&segment1)?;
    println!("Segment 1: {} pulses at {} rpm", segment1.displacement, segment1.speed);
    
    // Configure segment 2: Move -5000 pulses (reverse) at 500 rpm
    let segment2 = SegmentConfig {
        segment: 2,
        displacement: -5000,
        speed: 500,
        accel_decel_time: 150,
        wait_time: 500,
    };
    servo.configure_segment(&segment2)?;
    println!("Segment 2: {} pulses at {} rpm", segment2.displacement, segment2.speed);
    
    // Configure segment 3: Return to start at 800 rpm
    let segment3 = SegmentConfig {
        segment: 3,
        displacement: -5000,
        speed: 800,
        accel_decel_time: 100,
        wait_time: 0,
    };
    servo.configure_segment(&segment3)?;
    println!("Segment 3: {} pulses at {} rpm", segment3.displacement, segment3.speed);
    println!("Multi-segment configured: segments 1-3, continuous cycle mode");

    // Check servo state
    let state = servo.get_servo_state()?;
    match state {
        ServoState::Ready => {
            println!("\n✓ Servo is ready");
        }
        ServoState::Error | ServoState::Alarm => {
            println!("\n✗ Servo has error/alarm - resetting...");
            servo.reset_fault()?;
            thread::sleep(Duration::from_millis(500));
        }
        _ => {
            println!("\nServo state: {:?}", state);
        }
    }

    // Speed control demonstration
    println!("\n--- Speed Control Demo ---");
    servo.set_control_mode(ControlMode::Speed)?;
    println!("Switched to speed control mode");

    // Set speed to 200 rpm
    println!("Setting speed to 200 rpm...");
    servo.set_speed_command(200)?;
    
    // Monitor for 2 seconds
    for i in 1..=4 {
        thread::sleep(Duration::from_millis(500));
        let speed = servo.get_speed()?;
        let torque = servo.get_torque()?;
        println!("  {}ms: Speed={} rpm, Torque={:.1}%", i * 500, speed, torque);
    }

    // Stop motor
    println!("Stopping motor...");
    servo.set_speed_command(0)?;
    thread::sleep(Duration::from_millis(500));

    // Final status
    println!("\n--- Final Status ---");
    display_status(&mut servo)?;

    // Switch back to position mode
    servo.set_control_mode(ControlMode::Position)?;
    println!("\nSwitched back to position control mode");

    println!("\n✓ Sync example completed successfully!");
    Ok(())
}

/// Display current servo status
fn display_status(servo: &mut DsyrsSyncClient) -> Result<(), Box<dyn std::error::Error>> {
    let status = servo.get_status()?;
    println!("State: {:?}", status.state);
    println!("Speed: {} rpm", status.speed);
    println!("Position: {} pulses", status.position);
    println!("Torque: {:.1}% of rated", status.torque as f32 * 0.1);
    println!("Current: {:.2} A", status.current as f32 * 0.01);
    println!("Bus Voltage: {:.1} V", status.bus_voltage as f32 * 0.1);
    println!("Electrical Angle: {:.1}°", status.electrical_angle as f32 * 0.1);
    Ok(())
}
