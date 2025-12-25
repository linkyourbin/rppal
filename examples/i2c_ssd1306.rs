// i2c_ssd1306.rs - SSD1306 OLED Display Example
//
// Demonstrates how to use the SSD1306 OLED display with RPPAL I2C.
// This example shows text, shapes, and basic graphics on a 128x64 OLED display.
//
// Hardware:
//   - SSD1306 OLED Display (128x64 or 128x32)
//   - I2C Address: 0x3C (or 0x3D)
//
// Connections:
//   Display VCC → 3.3V (Pin 1)
//   Display GND → GND  (Pin 6)
//   Display SDA → GPIO2 (Pin 3)
//   Display SCL → GPIO3 (Pin 5)
//
// Compile with:
//   cargo build --example i2c_ssd1306 --release --features embedded-hal-0

use std::error::Error;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rppal::i2c::I2c;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use embedded_graphics::{
    mono_font::{ascii::FONT_7X14_BOLD, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};

// Format Unix timestamp to HH:MM:SS (UTC+8 Beijing Time)
fn format_time(timestamp: u64) -> String {
    // Convert to Beijing Time (UTC+8)
    let total_seconds = timestamp + 8 * 3600;
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

// Format date as YYYY-MM-DD
fn format_date(timestamp: u64) -> String {
    let days_since_epoch = (timestamp / 86400) as i32;

    // Calculate year, month, day from days since epoch (simplified)
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let days_in_months = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &days in &days_in_months {
        if remaining_days < days {
            break;
        }
        remaining_days -= days;
        month += 1;
    }

    let day = remaining_days + 1;
    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("=== SSD1306 OLED Display Test ===");
    println!();

    // Initialize I2C
    // Note: I2c implements embedded-hal 0.2.x blocking::i2c::Write trait
    // when compiled with 'embedded-hal-0' feature
    println!("Initializing I2C...");
    let i2c = I2c::new().unwrap();
    println!("✅ I2C initialized");
    println!();

    // Create the I2C interface for SSD1306
    // The interface will handle setting the slave address automatically
    let interface = I2CDisplayInterface::new(i2c);

    // Create the display driver
    // Use DisplaySize128x64 for 128x64 displays, or DisplaySize128x32 for 128x32
    println!("Initializing SSD1306 display (128x64)...");
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    println!("✅ Display initialized");
    println!();

    // Clear the display
    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();

    // Test 1: Simple text
    println!("Test 1: Displaying text...");
    display.clear(BinaryColor::Off).unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_7X14_BOLD)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello, Rust!", Point::new(0, 0), text_style, Baseline::Top)
        .draw(&mut display).unwrap();

    Text::with_baseline("RPPAL + SSD1306", Point::new(0, 12), text_style, Baseline::Top)
        .draw(&mut display).unwrap();

    Text::with_baseline("I2C: 0x3C", Point::new(0, 24), text_style, Baseline::Top)
        .draw(&mut display).unwrap();

    Text::with_baseline("CM0 Board", Point::new(0, 36), text_style, Baseline::Top)
        .draw(&mut display).unwrap();

    display.flush().unwrap();
    println!("  ✅ Text displayed");
    thread::sleep(Duration::from_secs(3));

    // Test 2: Draw shapes
    println!("Test 2: Drawing shapes...");
    display.clear(BinaryColor::Off).unwrap();

    // Draw a rectangle
    Rectangle::new(Point::new(5, 5), Size::new(40, 30))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display).unwrap();

    // Draw a filled circle
    Circle::new(Point::new(70, 10), 20)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display).unwrap();

    // Draw some lines
    Line::new(Point::new(0, 45), Point::new(127, 45))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display).unwrap();

    Line::new(Point::new(64, 46), Point::new(64, 63))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display).unwrap();

    display.flush().unwrap();
    println!("  ✅ Shapes drawn");
    thread::sleep(Duration::from_secs(3));

    // Test 3: Animation - Moving rectangle
    println!("Test 3: Simple animation...");
    for x in 0..108 {
        display.clear(BinaryColor::Off).unwrap();

        Rectangle::new(Point::new(x as i32, 20), Size::new(20, 20))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display).unwrap();

        display.flush().unwrap();
        thread::sleep(Duration::from_millis(20));
    }
    println!("  ✅ Animation complete");
    thread::sleep(Duration::from_secs(1));

    // Test 4: Counter
    println!("Test 4: Counter display (10 seconds)...");
    for i in 0..10 {
        display.clear(BinaryColor::Off).unwrap();

        let text = format!("Counter: {}", i);
        Text::with_baseline(&text, Point::new(20, 25), text_style, Baseline::Top)
            .draw(&mut display).unwrap();

        display.flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("  ✅ Counter test complete");

    // Final message
    display.clear(BinaryColor::Off).unwrap();
    Text::with_baseline("Test Complete!", Point::new(15, 25), text_style, Baseline::Top)
        .draw(&mut display).unwrap();
    display.flush().unwrap();

    thread::sleep(Duration::from_secs(2));

    // Test 5: Real-time Clock Display
    println!("Test 5: Real-time clock (Press Ctrl+C to stop)...");
    println!();

    loop {
        // Get current system time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Format time and date
        let time_str = format_time(now);
        let date_str = format_date(now);

        // Clear and draw clock display
        display.clear(BinaryColor::Off).unwrap();

        // Draw a border
        Rectangle::new(Point::new(0, 0), Size::new(128, 64))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display).unwrap();

        // Display date
        Text::with_baseline(&date_str, Point::new(20, 10), text_style, Baseline::Top)
            .draw(&mut display).unwrap();

        // Display time (larger position for emphasis)
        Text::with_baseline(&time_str, Point::new(30, 30), text_style, Baseline::Top)
            .draw(&mut display).unwrap();

        // Display "Beijing Time" label
        Text::with_baseline("Beijing Time", Point::new(18, 45), text_style, Baseline::Top)
            .draw(&mut display).unwrap();

        display.flush().unwrap();

        // Print to console as well
        print!("\r{} {}  ", date_str, time_str);
        std::io::Write::flush(&mut std::io::stdout()).ok();

        // Update every second
        thread::sleep(Duration::from_secs(1));
    }

    // This code is unreachable due to the infinite loop
    #[allow(unreachable_code)]
    Ok(())
}
