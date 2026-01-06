//! Example: Synchronous usage with multiple devices on the same RS485 bus
//!
//! This example demonstrates how to use the sync API to control both
//! DSY-RS servos and EM2RS steppers on the same RS485 bus, using the
//! same approach as em2rs library.
//!
//! Key concepts:
//! - Using tokio-modbus sync context directly
//! - Sharing context between dsyrs and em2rs clients
//! - Switching between devices on the same bus

use dsyrs::{DsyrsSyncClient, ServoConfig, ControlMode, Slave};
use tokio_modbus::prelude::{client, SlaveContext};

const SERIAL_PORT: &str = "/dev/ttyUSB0";
const BAUD_RATE: u32 = 115200;

/// Example 1: Basic sync servo connection
fn example_sync_servo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Sync Servo Example ===\n");
    
    // Create serial port builder
    let builder = tokio_serial::new(SERIAL_PORT, BAUD_RATE);
    
    // Connect to slave 1 via tokio-modbus sync API
    let ctx = client::sync::rtu::connect_slave(&builder, Slave::from(1))?;
    
    // Create servo configuration
    let config = ServoConfig::new(1)
        .with_control_mode(ControlMode::Position)
        .with_max_speed(3000);
    
    // Create sync client
    let mut servo = DsyrsSyncClient::new(ctx, config);
    
    // Initialize
    servo.init()?;
    
    // Read status
    let status = servo.get_status()?;
    println!("Servo Status:");
    println!("  State: {:?}", status.state);
    println!("  Speed: {} rpm", status.speed);
    println!("  Position: {}", status.position);
    println!("  Load: {}%", status.load_rate as f32 * 0.1);
    
    // Read versions
    let sw_version = servo.get_software_version()?;
    let fpga_version = servo.get_fpga_version()?;
    println!("  SW Version: {:#06X}", sw_version);
    println!("  FPGA Version: {:#06X}", fpga_version);
    
    Ok(())
}

/// Example 2: Multi-device on same RS485 bus
/// 
/// This is the pattern for using dsyrs and em2rs together:
/// 1. Create context with client::sync::rtu::connect_slave()
/// 2. Create dsyrs client
/// 3. Use dsyrs client
/// 4. Extract context with into_context()
/// 5. Switch slave with ctx.set_slave()
/// 6. Create em2rs client with the same context
fn example_multi_device_sync() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Multi-Device Sync Example ===\n");
    
    // Create serial port builder
    let builder = tokio_serial::new(SERIAL_PORT, BAUD_RATE);
    
    // Connect to servo on slave 1
    let ctx = client::sync::rtu::connect_slave(&builder, Slave::from(1))?;
    
    // Create servo client
    let servo_config = ServoConfig::new(1)
        .with_control_mode(ControlMode::Position);
    let mut servo = DsyrsSyncClient::new(ctx, servo_config);
    
    println!("Connected to Servo (slave 1)");
    servo.init()?;
    
    // Do some work with servo...
    let speed = servo.get_speed()?;
    println!("  Servo speed: {} rpm", speed);
    
    // Extract context to reuse for stepper
    let mut ctx = servo.into_context();
    println!("\nContext extracted from servo");
    
    // Switch to stepper on slave 2
    ctx.set_slave(Slave::from(2));
    println!("Switched context to slave 2 (Stepper)");
    
    // Now you can use this context with em2rs:
    // let stepper_config = em2rs::StepperConfig::new(2, 10000);
    // let mut stepper = em2rs::Em2rsSyncClient::new(ctx, stepper_config);
    // stepper.init()?;
    // stepper.move_relative(1000)?;
    
    // To switch back to servo, extract context from stepper:
    // let mut ctx = stepper.into_context();
    // ctx.set_slave(Slave::from(1));
    // let mut servo = DsyrsSyncClient::new(ctx, servo_config);
    
    println!("\nMulti-device pattern demonstrated!");
    println!("In real usage, both dsyrs and em2rs would share this context.");
    
    Ok(())
}

/// Example 3: Demonstrates the context pattern
/// (This example doesn't require hardware)
fn example_context_pattern() {
    println!("\n=== Context Sharing Pattern ===\n");
    
    println!("The pattern for dsyrs + em2rs interoperability:");
    println!();
    println!("```rust");
    println!("use dsyrs::{{DsyrsSyncClient, ServoConfig}};");
    println!("use em2rs::{{Em2rsSyncClient, StepperConfig}};");
    println!("use tokio_modbus::prelude::*;");
    println!();
    println!("// 1. Create RTU connection");
    println!("let builder = tokio_serial::new(\"/dev/ttyUSB0\", 115200);");
    println!("let ctx = client::sync::rtu::connect_slave(&builder, Slave::from(1))?;");
    println!();
    println!("// 2. Use servo");
    println!("let servo_config = ServoConfig::new(1);");
    println!("let mut servo = DsyrsSyncClient::new(ctx, servo_config);");
    println!("servo.init()?;");
    println!("servo.set_speed_command(500)?;");
    println!();
    println!("// 3. Extract context, switch to stepper");
    println!("let mut ctx = servo.into_context();");
    println!("ctx.set_slave(Slave::from(2));");
    println!();
    println!("// 4. Use stepper");
    println!("let stepper_config = StepperConfig::new(2, 10000);");
    println!("let mut stepper = Em2rsSyncClient::new(ctx, stepper_config);");
    println!("stepper.init()?;");
    println!("stepper.move_relative(1000)?;");
    println!();
    println!("// 5. Switch back to servo if needed");
    println!("let mut ctx = stepper.into_context();");
    println!("ctx.set_slave(Slave::from(1));");
    println!("let mut servo = DsyrsSyncClient::new(ctx, servo_config);");
    println!("```");
    println!();
    println!("Both libraries use the EXACT same tokio-modbus sync::Context!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("DSY-RS + EM2RS Interoperability Examples");
    println!("========================================\n");
    
    // This example doesn't require hardware:
    example_context_pattern();
    
    // Uncomment to run with actual hardware:
    // example_sync_servo()?;
    // example_multi_device_sync()?;
    
    println!("\n========================================");
    println!("Examples completed!");
    
    Ok(())
}
