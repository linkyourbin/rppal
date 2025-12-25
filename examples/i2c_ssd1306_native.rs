// i2c_ssd1306_native.rs - SSD1306 OLED Display (Pure RPPAL)
use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal::i2c::I2c;

const SSD1306_ADDR: u16 = 0x3C;
const CMD: u8 = 0x00;
const DATA: u8 = 0x40;

struct Ssd1306 {
    i2c: I2c,
    buf: Vec<u8>,
}

impl Ssd1306 {
    fn new(mut i2c: I2c) -> Result<Self, Box<dyn Error>> {
        i2c.set_slave_address(SSD1306_ADDR)?;
        Ok(Ssd1306 { i2c, buf: vec![0u8; 1024] })
    }

    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        for cmd in &[0xAE, 0xD5, 0x80, 0xA8, 0x3F, 0xD3, 0x00, 0x40, 0x8D, 0x14,
                     0x20, 0x00, 0xA1, 0xC8, 0xDA, 0x12, 0x81, 0xCF, 0xD9, 0xF1,
                     0xDB, 0x40, 0xA6, 0xAF] {
            self.i2c.block_write(CMD, &[*cmd])?;
        }
        Ok(())
    }

    fn clear(&mut self) {
        self.buf.fill(0);
    }

    fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        self.i2c.block_write(CMD, &[0x21, 0, 127])?;
        self.i2c.block_write(CMD, &[0x22, 0, 7])?;
        for chunk in self.buf.chunks(32) {
            self.i2c.block_write(DATA, chunk)?;
        }
        Ok(())
    }

    fn pixel(&mut self, x: usize, y: usize, on: bool) {
        if x >= 128 || y >= 64 { return; }
        let idx = (y / 8) * 128 + x;
        if on { self.buf[idx] |= 1 << (y % 8); }
        else { self.buf[idx] &= !(1 << (y % 8)); }
    }

    fn text(&mut self, x: usize, y: usize, s: &str) {
        const F: [[u8;5];10] = [
            [0x3E,0x51,0x49,0x45,0x3E], [0x00,0x42,0x7F,0x40,0x00],
            [0x42,0x61,0x51,0x49,0x46], [0x21,0x41,0x45,0x4B,0x31],
            [0x18,0x14,0x12,0x7F,0x10], [0x27,0x45,0x45,0x45,0x39],
            [0x3C,0x4A,0x49,0x49,0x30], [0x01,0x71,0x09,0x05,0x03],
            [0x36,0x49,0x49,0x49,0x36], [0x06,0x49,0x49,0x29,0x1E],
        ];
        let mut cx = x;
        for c in s.chars() {
            if c.is_numeric() {
                let d = c.to_digit(10).unwrap() as usize;
                for (col, &byte) in F[d].iter().enumerate() {
                    for row in 0..8 {
                        if byte & (1 << row) != 0 {
                            self.pixel(cx + col, y + row, true);
                        }
                    }
                }
                cx += 6;
            }
        }
    }

    fn rect(&mut self, x: usize, y: usize, w: usize, h: usize, fill: bool) {
        if fill {
            for dy in 0..h {
                for dx in 0..w {
                    self.pixel(x + dx, y + dy, true);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== SSD1306 Test (Pure RPPAL) ===\n");
    
    let mut display = Ssd1306::new(I2c::new()?)?;
    display.init()?;
    println!("✅ Display initialized\n");

    // Test 1: Rectangles
    display.clear();
    display.rect(10, 10, 30, 20, true);
    display.rect(60, 10, 30, 20, true);
    display.flush()?;
    println!("Test 1: Shapes");
    thread::sleep(Duration::from_secs(2));

    // Test 2: Counter
    println!("Test 2: Counter");
    for i in 0..10 {
        display.clear();
        display.text(50, 25, &format!("{}", i));
        display.flush()?;
        thread::sleep(Duration::from_secs(1));
    }

    display.clear();
    display.flush()?;
    println!("\n✅ Complete");
    Ok(())
}
