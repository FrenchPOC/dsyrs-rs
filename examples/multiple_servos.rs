//! Multi-servo example for DSY-RS servo drive controller
//!
//! This example demonstrates:
//! - Controlling multiple servos on the same RS-485 bus
//! - Coordinated motion between servos
//! - Using async for concurrent operations
//!
//! Run with: cargo run --example multiple_servos

use dsyrs::{ControlMode, Direction, DsyrsClient, ServoConfig, ServoState};
use std::time::Duration;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

/// Servo IDs on the bus
const SERVO_IDS: [u8; 3] = [1, 2, 3];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("DSY-RS Multiple Servos Example");
    println!("===============================\n");

    // Serial port configuration
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115200;

    println!("Connecting to {} at {} baud...", port_name, baud_rate);
    println!(
        "Managing {} servos with IDs: {:?}\n",
        SERVO_IDS.len(),
        SERVO_IDS
    );

    // Open serial port with timeout
    let builder = tokio_serial::new(port_name, baud_rate).timeout(Duration::from_millis(100));
    let port = SerialStream::open(&builder)?;

    // Create initial Modbus RTU context (with first slave ID)
    let mut ctx = rtu::attach_slave(port, Slave::from(SERVO_IDS[0]));

    // Initialize all servos
    println!("--- Initializing Servos ---");
    for &slave_id in &SERVO_IDS {
        // Switch to this slave
        ctx.set_slave(Slave::from(slave_id));

        // Create configuration for this servo
        // Note: rated_current, encoder_type, encoder_resolution, and motor_model_code are optional.
        // If not specified, they will be read from the servo during init().
        let config = ServoConfig::new(slave_id)
            .with_control_mode(ControlMode::Speed)
            .with_direction(Direction::CwForward)
            .with_max_speed(3000)
            .with_rated_current(3.0);

        // Create temporary client for initialization
        // Note: In a real application, you might want to store these clients
        let mut servo = DsyrsClient::new(ctx, config);
        servo.init().await?;

        // Read status to verify connection
        let state = servo.get_servo_state().await?;
        let speed = servo.get_speed().await?;
        println!("Servo {}: State={:?}, Speed={} rpm", slave_id, state, speed);

        // Get context back for reuse
        ctx = servo.into_context();
    }

    // Read status from all servos
    println!("\n--- Reading All Servo Status ---");
    for &slave_id in &SERVO_IDS {
        ctx.set_slave(Slave::from(slave_id));

        let config = ServoConfig::new(slave_id);
        let mut servo = DsyrsClient::new(ctx, config);

        let status = servo.get_status().await?;
        println!("\nServo {} Status:", slave_id);
        println!("  State: {:?}", status.state);
        println!("  Speed: {} rpm", status.speed);
        println!("  Position: {} pulses", status.position);
        println!("  Torque: {:.1}%", status.torque as f32 * 0.1);
        println!("  Bus Voltage: {:.1} V", status.bus_voltage as f32 * 0.1);

        ctx = servo.into_context();
    }

    // Coordinated motion example
    println!("\n--- Coordinated Motion Demo ---");
    println!("Running all servos at different speeds...\n");

    let speeds: [i16; 3] = [100, 200, 300]; // Different speeds for each servo

    // Set speeds for all servos
    for (i, &slave_id) in SERVO_IDS.iter().enumerate() {
        ctx.set_slave(Slave::from(slave_id));

        let config = ServoConfig::new(slave_id);
        let mut servo = DsyrsClient::new(ctx, config);

        servo.set_speed_command(speeds[i]).await?;
        println!("Servo {}: Set speed to {} rpm", slave_id, speeds[i]);

        ctx = servo.into_context();
    }

    // Monitor for 3 seconds
    println!("\nMonitoring servo speeds...");
    for iteration in 1..=6 {
        tokio::time::sleep(Duration::from_millis(500)).await;

        print!("{}ms: ", iteration * 500);
        for &slave_id in &SERVO_IDS {
            ctx.set_slave(Slave::from(slave_id));

            let config = ServoConfig::new(slave_id);
            let mut servo = DsyrsClient::new(ctx, config);

            let speed = servo.get_speed().await?;
            print!("S{}={} rpm  ", slave_id, speed);

            ctx = servo.into_context();
        }
        println!();
    }

    // Synchronized stop
    println!("\n--- Stopping All Servos ---");
    for &slave_id in &SERVO_IDS {
        ctx.set_slave(Slave::from(slave_id));

        let config = ServoConfig::new(slave_id);
        let mut servo = DsyrsClient::new(ctx, config);

        servo.set_speed_command(0).await?;
        println!("Servo {}: Stopped", slave_id);

        ctx = servo.into_context();
    }

    // Wait for deceleration
    tokio::time::sleep(Duration::from_millis(1000)).await;

    // Verify all stopped
    println!("\n--- Verifying Stop ---");
    let mut all_stopped = true;
    for &slave_id in &SERVO_IDS {
        ctx.set_slave(Slave::from(slave_id));

        let config = ServoConfig::new(slave_id);
        let mut servo = DsyrsClient::new(ctx, config);

        let speed = servo.get_speed().await?;
        let stopped = speed.abs() < 5; // Within 5 rpm tolerance
        println!(
            "Servo {}: Speed={} rpm {}",
            slave_id,
            speed,
            if stopped { "✓" } else { "⚠ still moving" }
        );
        all_stopped = all_stopped && stopped;

        ctx = servo.into_context();
    }

    if all_stopped {
        println!("\n✓ All servos stopped successfully!");
    } else {
        println!("\n⚠ Some servos still moving, may need more deceleration time");
    }

    // Check for any faults
    println!("\n--- Final Fault Check ---");
    let mut any_fault = false;
    for &slave_id in &SERVO_IDS {
        ctx.set_slave(Slave::from(slave_id));

        let config = ServoConfig::new(slave_id);
        let mut servo = DsyrsClient::new(ctx, config);

        let state = servo.get_servo_state().await?;
        if state == ServoState::Error || state == ServoState::Alarm {
            println!("Servo {}: ✗ ERROR/ALARM detected", slave_id);
            any_fault = true;
        } else {
            println!("Servo {}: ✓ OK ({:?})", slave_id, state);
        }

        ctx = servo.into_context();
    }

    if any_fault {
        println!("\n⚠ Some servos have faults. Consider using reset_fault()");
    }

    println!("\n✓ Multiple servos example completed!");
    Ok(())
}
