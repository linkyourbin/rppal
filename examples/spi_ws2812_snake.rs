// spi_ws2812_snake.rs - 贪吃蛇效果（修复时序）
//
// 使用正确的 SPI 编码确保 WS2812 时序准确

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

const NUM_LEDS: usize = 64;
const BRIGHTNESS: u8 = 38; // 全局亮度 (0-255)

#[derive(Clone, Copy, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    fn black() -> Self {
        Color::new(0, 0, 0)
    }

    fn with_brightness(&self, brightness: u8) -> Self {
        Color::new(
            ((self.r as u16 * brightness as u16) / 255) as u8,
            ((self.g as u16 * brightness as u16) / 255) as u8,
            ((self.b as u16 * brightness as u16) / 255) as u8,
        )
    }
}

// WS2812 时序编码表（使用 3 个 SPI 位编码 1 个 WS2812 位）
// 在 6.4 MHz SPI 频率下：
// - 每个 SPI 位 = 156.25 ns
// - 3 个 SPI 位 = 468.75 ns ≈ 0.47 μs
//
// WS2812 要求：
// - 0 码：高 0.4μs，低 0.85μs (总 1.25μs)
// - 1 码：高 0.8μs，低 0.45μs (总 1.25μs)
//
// 使用 4 个 SPI 位编码 1 个 WS2812 位：
// - 4 个 SPI 位 = 625 ns ≈ 0.625 μs
// - 0 码：0b1000 (高 156ns, 低 469ns) - 接近但偏短
// - 1 码：0b1110 (高 469ns, 低 156ns) - 接近但偏短
//
// 更好的方案：使用 3.2 MHz SPI 频率
// - 每个 SPI 位 = 312.5 ns
// - 0 码：0b100 (高 312ns, 低 625ns) ≈ (0.31μs, 0.63μs)
// - 1 码：0b110 (高 625ns, 低 312ns) ≈ (0.63μs, 0.31μs)

// 将一个字节编码为 SPI 数据（每个位用 4 个 SPI 位编码）
fn encode_byte_to_spi(byte: u8, buffer: &mut Vec<u8>) {
    for i in 0..8 {
        let bit = (byte >> (7 - i)) & 1;
        if bit == 1 {
            // 1 码：0b11100000 (高电平更长)
            buffer.push(0b11100000);
        } else {
            // 0 码：0b10000000 (高电平更短)
            buffer.push(0b10000000);
        }
    }
}

// 将颜色数组编码为 SPI 缓冲区
fn encode_colors_to_spi(colors: &[Color; NUM_LEDS]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(NUM_LEDS * 24); // 每个 LED 24 字节

    for color in colors.iter() {
        let color = color.with_brightness(BRIGHTNESS);
        // WS2812 使用 GRB 顺序
        encode_byte_to_spi(color.g, &mut buffer);
        encode_byte_to_spi(color.r, &mut buffer);
        encode_byte_to_spi(color.b, &mut buffer);
    }

    buffer
}

// 发送颜色数据到 WS2812 灯带
fn update_leds(spi: &mut Spi, colors: &[Color; NUM_LEDS]) -> Result<(), Box<dyn Error>> {
    let buffer = encode_colors_to_spi(colors);
    spi.write(&buffer)?;

    // 发送复位信号（至少 50μs 的低电平）
    // 在 6.4 MHz 下，64 个零字节 = 80μs
    let reset = vec![0u8; 64];
    spi.write(&reset)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("初始化 SPI 接口...");

    // 尝试不同的 SPI 频率以获得最佳时序
    // 推荐频率：3.2 MHz 或 6.4 MHz
    let spi_freq = 6_400_000; // 6.4 MHz

    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, spi_freq, Mode::Mode0)?;
    println!("SPI 初始化成功！频率: {} Hz", spi_freq);
    println!("开始贪吃蛇效果...\n");

    let mut colors = [Color::black(); NUM_LEDS];
    let snake_color = Color::new(255, 200, 0); // 黄橙色

    loop {
        // 从 0 到 63 依次点亮
        println!("正向：0 -> 63");
        for i in 0..NUM_LEDS {
            colors[i] = snake_color;
            update_leds(&mut spi, &colors)?;
            print!("\r点亮 LED {:2}", i);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            thread::sleep(Duration::from_millis(200));
        }
        println!();

        // 短暂停留
        thread::sleep(Duration::from_millis(500));

        // 从 63 到 0 依次熄灭
        println!("反向：63 -> 0");
        for i in (0..NUM_LEDS).rev() {
            colors[i] = Color::black();
            update_leds(&mut spi, &colors)?;
            print!("\r熄灭 LED {:2}", i);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            thread::sleep(Duration::from_millis(200));
        }
        println!();

        // 短暂停留后重新开始
        thread::sleep(Duration::from_millis(500));
        println!("重新开始...\n");
    }
}
