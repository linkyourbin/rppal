// i2c_scan.rs - Scans all I2C buses for connected devices
//
// This tool scans the I2C bus for connected devices and displays their addresses.
// It's useful for detecting I2C devices without knowing their addresses in advance.

use std::error::Error;
use rppal::i2c::I2c;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== I2C Bus Scanner ===");
    println!();

    // Try to open I2C interface
    let mut i2c = match I2c::new() {
        Ok(i2c) => i2c,
        Err(e) => {
            eprintln!("❌ Failed to open I2C interface: {}", e);
            eprintln!();
            eprintln!("Possible causes:");
            eprintln!("  1. I2C is not enabled in /boot/firmware/config.txt");
            eprintln!("  2. No permission to access /dev/i2c-* (try with sudo)");
            eprintln!("  3. I2C kernel module not loaded");
            eprintln!();
            eprintln!("To enable I2C:");
            eprintln!("  1. Edit: sudo nano /boot/firmware/config.txt");
            eprintln!("  2. Uncomment: dtparam=i2c_arm=on");
            eprintln!("  3. Reboot: sudo reboot");
            return Err(e.into());
        }
    };

    println!("✅ I2C interface opened successfully");
    println!();

    // Get I2C capabilities
    let capabilities = i2c.capabilities();
    println!("I2C Capabilities:");
    println!("  Functions: {:?}", capabilities);
    println!();

    println!("Scanning I2C bus for devices...");
    println!("This may take a few seconds...");
    println!();

    // Print header
    println!("     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f");

    let mut device_count = 0;
    let mut detected_addresses = Vec::new();

    // Scan all possible I2C addresses (0x03 to 0x77)
    for row in 0..8 {
        print!("{:0>2x}: ", row * 16);

        for col in 0..16 {
            let addr = (row * 16 + col) as u16;

            // Skip reserved addresses
            if addr < 0x03 || addr > 0x77 {
                print!("   ");
                continue;
            }

            // Try to set slave address and perform a quick read
            match i2c.set_slave_address(addr) {
                Ok(_) => {
                    // Try a quick read to see if device responds
                    let mut buf = [0u8; 1];
                    match i2c.read(&mut buf) {
                        Ok(_) => {
                            print!("{:0>2x} ", addr);
                            device_count += 1;
                            detected_addresses.push(addr);
                        }
                        Err(_) => {
                            print!("-- ");
                        }
                    }
                }
                Err(_) => {
                    print!("-- ");
                }
            }
        }
        println!();
    }

    println!();
    println!("=== Scan Complete ===");
    println!();

    if device_count == 0 {
        println!("❌ No I2C devices detected");
        println!();
        println!("Possible reasons:");
        println!("  1. No I2C devices are connected");
        println!("  2. Devices are not powered");
        println!("  3. Wrong SDA/SCL connections");
        println!("  4. Pull-up resistors missing (usually 4.7kΩ)");
    } else {
        println!("✅ Found {} device(s) at address(es):", device_count);
        for addr in detected_addresses {
            println!("   0x{:02X} ({})", addr, addr);

            // Identify common devices
            match addr {
                0x68 | 0x69 => println!("      → Likely: DS3231 RTC, MPU6050, or similar"),
                0x76 | 0x77 => println!("      → Likely: BMP280, BME280, or similar"),
                0x3C | 0x3D => println!("      → Likely: SSD1306 OLED display"),
                0x27 => println!("      → Likely: PCF8574 I2C LCD adapter"),
                0x48 | 0x49 | 0x4A | 0x4B => println!("      → Likely: ADS1115, PCF8591, or similar"),
                0x50 | 0x51 | 0x52 | 0x53 => println!("      → Likely: EEPROM (24Cxx series)"),
                _ => {}
            }
        }
    }

    println!();
    println!("I2C Pins on Raspberry Pi CM0:");
    println!("  SDA: GPIO2  (Pin 3)");
    println!("  SCL: GPIO3  (Pin 5)");
    println!();

    Ok(())
}
