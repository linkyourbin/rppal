// spi_ws2812_advanced.rs - 高级灯光效果（包含多 LED 独立控制效果）
//
// 包含更多适合 64 颗 LED 的高级效果

use std::error::Error;
use std::f32::consts::PI;
use std::thread;
use std::time::{Duration, Instant};

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

const NUM_LEDS: usize = 64;
const BRIGHTNESS: u8 = 26; // 全局亮度 (0-255)，对应 10% 亮度

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

    fn white() -> Self {
        Color::new(255, 255, 255)
    }

    fn with_brightness(&self, brightness: u8) -> Self {
        Color::new(
            ((self.r as u16 * brightness as u16) / 255) as u8,
            ((self.g as u16 * brightness as u16) / 255) as u8,
            ((self.b as u16 * brightness as u16) / 255) as u8,
        )
    }
}

// 正确的 WS2812 编码方式（每个位用 8 个 SPI 位编码）
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

fn encode_colors(colors: &[Color; NUM_LEDS]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(NUM_LEDS * 24);

    for color in colors.iter() {
        let color = color.with_brightness(BRIGHTNESS);
        // WS2812 使用 GRB 顺序
        encode_byte_to_spi(color.g, &mut buffer);
        encode_byte_to_spi(color.r, &mut buffer);
        encode_byte_to_spi(color.b, &mut buffer);
    }

    buffer
}

fn update_leds(spi: &mut Spi, colors: &[Color; NUM_LEDS]) -> Result<(), Box<dyn Error>> {
    let buffer = encode_colors(colors);
    spi.write(&buffer)?;
    let reset = vec![0u8; 64];
    spi.write(&reset)?;
    Ok(())
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

// 效果 1: 彩虹流水（每个 LED 不同颜色，整体移动）
fn effect_rainbow_wave(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let t = elapsed_ms as f32 / 50.0;
    for i in 0..NUM_LEDS {
        let hue = ((i as f32 / NUM_LEDS as f32) * 360.0 + t) % 360.0;
        colors[i] = hsv_to_rgb(hue, 1.0, 1.0);
    }
}

// 效果 2: 流星效果（白色流星）
fn effect_meteor(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let period = 2000;
    let t = elapsed_ms % period;
    let position = ((t as f32 / period as f32) * NUM_LEDS as f32) as usize;

    colors.fill(Color::black());

    if position < NUM_LEDS {
        colors[position] = Color::white(); // 白色流星头
    }

    for i in 1..=10 {
        if position >= i && position - i < NUM_LEDS {
            let brightness = (255 * (11 - i) / 11) as u8;
            colors[position - i] = Color::new(brightness, brightness, brightness); // 白色尾巴
        }
    }
}

// 效果 3: 呼吸波浪（波浪形的亮度变化，青色）
fn effect_breath_wave(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let t = elapsed_ms as f32 / 100.0;
    for i in 0..NUM_LEDS {
        let phase = (i as f32 / NUM_LEDS as f32) * PI * 2.0 + t / 10.0;
        let brightness = ((phase.sin() + 1.0) / 2.0 * 255.0) as u8;
        colors[i] = Color::new(0, brightness, brightness); // 青色
    }
}

// 效果 4: 扫描效果（类似 K.I.T.T.，红色）
fn effect_scanner(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let period = 2000;
    let t = elapsed_ms % (period * 2);

    let position = if t < period {
        ((t as f32 / period as f32) * NUM_LEDS as f32) as usize
    } else {
        NUM_LEDS - 1 - (((t - period) as f32 / period as f32) * NUM_LEDS as f32) as usize
    };

    colors.fill(Color::black());

    for i in 0..NUM_LEDS {
        let distance = if i > position {
            i - position
        } else {
            position - i
        };

        if distance < 5 {
            let brightness = (255 * (5 - distance) / 5) as u8;
            colors[i] = Color::new(brightness, 0, 0); // 红色
        }
    }
}

// 效果 5: 火焰效果
fn effect_fire(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let t = elapsed_ms as f32 / 50.0;
    for i in 0..NUM_LEDS {
        let flicker = ((t + i as f32 * 0.5).sin() * 0.3 + 0.7).max(0.0).min(1.0);
        let r = (255.0 * flicker) as u8;
        let g = (100.0 * flicker) as u8;
        colors[i] = Color::new(r, g, 0);
    }
}

// 效果 6: 彩虹追逐（多个彩色点追逐）
fn effect_rainbow_chase(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let t = elapsed_ms as f32 / 100.0;
    colors.fill(Color::black());

    for chase in 0..4 {
        let offset = (chase as f32 * NUM_LEDS as f32 / 4.0) as usize;
        let position = ((t + offset as f32) as usize) % NUM_LEDS;
        let hue = (chase as f32 * 90.0) % 360.0;
        colors[position] = hsv_to_rgb(hue, 1.0, 1.0);

        for i in 1..=3 {
            let tail_pos = (position + NUM_LEDS - i) % NUM_LEDS;
            let brightness = (4 - i) as f32 / 4.0;
            colors[tail_pos] = hsv_to_rgb(hue, 1.0, brightness);
        }
    }
}

// 效果 7: 闪电效果（白色闪电）
fn effect_lightning(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let period = 3000;
    let t = elapsed_ms % period;

    if t < 50 || (t > 100 && t < 120) {
        colors.fill(Color::white()); // 白色闪电
    } else {
        colors.fill(Color::new(0, 0, 20)); // 暗蓝色背景
    }
}

// 效果 8: 渐变彩虹（静态彩虹，慢慢旋转）
fn effect_gradient_rainbow(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let t = elapsed_ms as f32 / 100.0;
    for i in 0..NUM_LEDS {
        let hue = ((i as f32 / NUM_LEDS as f32) * 360.0 + t) % 360.0;
        colors[i] = hsv_to_rgb(hue, 1.0, 1.0);
    }
}

// 效果 9: 剧场追逐（品红色）
fn effect_theater_chase(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let period = 200;
    let t = elapsed_ms % (period * 3);
    let offset = (t / period) as usize;

    colors.fill(Color::black());
    for i in (0..NUM_LEDS).step_by(3) {
        let pos = (i + offset) % NUM_LEDS;
        colors[pos] = Color::new(255, 0, 255); // 品红色
    }
}

// 效果 10: 脉冲波（从中心向两边扩散，青色）
fn effect_pulse_wave(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let period = 2000;
    let t = elapsed_ms % period;
    let progress = t as f32 / period as f32;
    let center = NUM_LEDS / 2;

    colors.fill(Color::black());

    for i in 0..NUM_LEDS {
        let distance = if i > center {
            i - center
        } else {
            center - i
        } as f32;

        let wave_pos = progress * (NUM_LEDS / 2) as f32;
        let diff = (distance - wave_pos).abs();

        if diff < 5.0 {
            let brightness = ((1.0 - diff / 5.0) * 255.0) as u8;
            colors[i] = Color::new(0, brightness, brightness); // 青色
        }
    }
}

// 效果 11: 彩色波浪（正弦波形的颜色变化）
fn effect_color_wave(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let t = elapsed_ms as f32 / 100.0;
    for i in 0..NUM_LEDS {
        let phase = (i as f32 / NUM_LEDS as f32) * PI * 4.0 + t / 5.0;
        let hue = ((phase.sin() + 1.0) / 2.0 * 360.0) as f32;
        colors[i] = hsv_to_rgb(hue, 1.0, 1.0);
    }
}

// 效果 12: 闪烁星空（白色星星）
fn effect_twinkle(colors: &mut [Color; NUM_LEDS], elapsed_ms: u64) {
    let t = elapsed_ms / 100;
    for i in 0..NUM_LEDS {
        let seed = (i as u64 * 1103515245 + t) % 256;
        if seed < 10 {
            colors[i] = Color::white(); // 白色星星
        } else if seed < 30 {
            let brightness = ((30 - seed) * 255 / 20) as u8;
            colors[i] = Color::new(brightness, brightness, brightness); // 白色渐暗
        } else {
            colors[i] = Color::black();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("初始化 SPI 接口...");
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_400_000, Mode::Mode0)?;
    println!("SPI 初始化成功！开始高级灯光效果演示...\n");

    let effects: Vec<(&str, fn(&mut [Color; NUM_LEDS], u64))> = vec![
        ("彩虹流水", effect_rainbow_wave),
        ("流星效果", effect_meteor),
        ("呼吸波浪", effect_breath_wave),
        ("扫描效果 (K.I.T.T.)", effect_scanner),
        ("火焰效果", effect_fire),
        ("彩虹追逐", effect_rainbow_chase),
        ("闪电效果", effect_lightning),
        ("渐变彩虹", effect_gradient_rainbow),
        ("剧场追逐", effect_theater_chase),
        ("脉冲波", effect_pulse_wave),
        ("彩色波浪", effect_color_wave),
        ("闪烁星空", effect_twinkle),
    ];

    let mut colors = [Color::black(); NUM_LEDS];
    let mut current_effect = 0;
    let mut effect_start_time = Instant::now();
    const EFFECT_DURATION: Duration = Duration::from_secs(10);

    loop {
        let elapsed = effect_start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as u64;

        if elapsed >= EFFECT_DURATION {
            current_effect = (current_effect + 1) % effects.len();
            effect_start_time = Instant::now();
            println!("\n切换到效果: {}", effects[current_effect].0);
        }

        effects[current_effect].1(&mut colors, elapsed_ms);
        update_leds(&mut spi, &colors)?;
        thread::sleep(Duration::from_millis(10));
    }
}
