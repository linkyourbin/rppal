# RPPAL - Raspberry Pi Peripheral Access Library (Community Enhanced Edition)

[![Build status](https://github.com/golemparts/rppal/actions/workflows/ci.yml/badge.svg)](https://github.com/golemparts/rppal/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/crates/v/rppal)](https://crates.io/crates/rppal)
[![Minimum rustc version](https://img.shields.io/badge/rustc-v1.60.0-lightgray.svg)](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html)
[![Documentation](https://docs.rs/rppal/badge.svg)](https://docs.rs/rppal)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🎉 Community Enhanced Edition

This is an enhanced fork of the original RPPAL project with additional features and hardware support. While the original project is no longer maintained (as of July 1, 2025), this community edition continues to provide updates and improvements.

### ✨ New Features & Enhancements

#### 🇨🇳 Extended Hardware Support
- **Raspberry Pi CM0** (China Special Edition) - Full support added
- Improved device detection for Chinese market variants
- Enhanced compatibility with regional hardware configurations

#### 🎨 WS2812 LED Strip Examples
A comprehensive collection of examples for controlling WS2812/NeoPixel LED strips via SPI:

1. **`spi_ws2812_snake.rs`** - Snake animation effect
   - Sequential LED lighting from 0 to 63
   - Reverse animation from 63 to 0
   - Configurable timing and colors
   - **Verified accurate timing** ✓

2. **`spi_ws2812_advanced.rs`** - 12 advanced lighting effects
   - 🌈 Rainbow wave - Full spectrum color flow
   - ⚪ Meteor effect - White meteor with trailing tail
   - 💙 Breathing wave - Cyan wave breathing pattern
   - 🔴 Scanner effect - K.I.T.T. style red scanner
   - 🔥 Fire effect - Yellow-orange flame simulation
   - 🎨 Rainbow chase - Multi-color chase pattern
   - ⚡ Lightning effect - White lightning flashes
   - 🌈 Gradient rainbow - Rotating rainbow gradient
   - 💜 Theater chase - Magenta theater lights
   - 💙 Pulse wave - Cyan pulse from center
   - 🎨 Color wave - Sine wave color transitions
   - ⭐ Twinkle stars - Random white star twinkling

#### 🖥️ I2C Display Examples
Enhanced I2C examples for OLED displays and device management:

3. **`i2c_ssd1306.rs`** - SSD1306 OLED display control
   - Full display driver implementation
   - Text and graphics support
   - Multiple display modes

4. **`i2c_ssd1306_native.rs`** - Native OLED control
   - Direct register access
   - Low-level display control
   - Performance optimized

5. **`i2c_scan.rs`** - I2C device scanner
   - Scan all I2C addresses (0x03-0x77)
   - Detect connected devices
   - Useful for debugging

#### 💡 PWM Effect Examples
Additional PWM examples for LED effects:

6. **`pwm_breath.rs`** - Breathing LED effect
   - Smooth brightness transitions
   - Sine wave modulation
   - Hardware PWM implementation

#### 📚 Documentation Enhancements
- **`WS2812_接线说明.md`** - Complete wiring guide (Chinese)
  - Detailed pin connections
  - Power requirements and calculations
  - Safety recommendations
  - Troubleshooting tips

- **`WS2812_故障排查.md`** - Troubleshooting guide (Chinese)
  - Common issues and solutions
  - Power supply diagnostics
  - Signal integrity problems
  - Voltage drop analysis

#### 🔧 Technical Improvements

**WS2812 SPI Encoding:**
- Precise timing implementation using 8-bit SPI encoding
- 0-bit: `0b10000000` (high 0.156μs, low 1.094μs)
- 1-bit: `0b11100000` (high 0.469μs, low 0.781μs)
- Compliant with WS2812 timing specifications (±150ns tolerance)
- Optimized for 6.4 MHz SPI frequency

**Power Management:**
- Configurable brightness levels (5%-100%)
- Current consumption calculations
- Safe operating recommendations
- Multi-point power injection guidance

**Color Support:**
- Full RGB color spectrum
- HSV to RGB conversion
- Gamma correction support
- Brightness adjustment per LED

## 📊 Contribution Summary

### Hardware Compatibility
| Device | Original Support | Enhanced Support |
|--------|-----------------|------------------|
| Raspberry Pi CM0 (China) | ❌ | ✅ |
| WS2812 LED Strips | ❌ | ✅ |
| Standard Models | ✅ | ✅ |

### Example Programs
| Category | Count | Description |
|----------|-------|-------------|
| WS2812 Examples | 2 | LED strip control via SPI |
| I2C Examples | 3 | OLED display and device scanning |
| PWM Examples | 1 | Breathing LED effect |
| Documentation | 5 | Comprehensive guides |
| Original Examples | 14 | GPIO, I2C, SPI, PWM, UART |

### Code Quality
- ✅ Verified timing accuracy for WS2812
- ✅ Comprehensive error handling
- ✅ Power consumption optimization
- ✅ Detailed inline documentation
- ✅ Safety warnings and best practices

## 🚀 Quick Start - WS2812 LED Control

### Hardware Setup

**Required Components:**
- Raspberry Pi (any model)
- WS2812/WS2812B LED strip (64 LEDs)
- 5V power supply (5A recommended for 64 LEDs)
- Jumper wires

**Wiring:**
```
Raspberry Pi GPIO 10 (Pin 19) ──[330Ω]──> WS2812 DIN
Raspberry Pi GND (Pin 6)      ──────────> Common GND
5V Power Supply GND           ──────────> Common GND
5V Power Supply +5V           ──────────> WS2812 VCC
```

**Enable SPI:**
```bash
sudo raspi-config
# Interface Options → SPI → Yes
sudo reboot
```

### Running Examples

**Snake Animation:**
```bash
cargo run --example spi_ws2812_snake
```

**Advanced Effects (12 modes):**
```bash
cargo run --example spi_ws2812_advanced
```

**Diagnostic Tool:**
```bash
cargo run --example spi_ws2812_diagnostic
```

### Configuration

Adjust brightness in the example files:
```rust
const BRIGHTNESS: u8 = 26;  // 10% brightness (safe for most power supplies)
const BRIGHTNESS: u8 = 51;  // 20% brightness (requires 5V/2A+)
const BRIGHTNESS: u8 = 128; // 50% brightness (requires 5V/4A+)
```

## 📖 Original RPPAL Documentation

### About the Original Project

RPPAL was created in 2016 and provided comprehensive access to Raspberry Pi peripherals through a user-friendly Rust interface. The original project is no longer maintained as of July 1, 2025, but this community edition continues its legacy.

### Supported Peripherals

RPPAL provides access to the Raspberry Pi's GPIO, I2C, PWM, SPI and UART peripherals. In addition to peripheral access, RPPAL also offers support for USB to serial adapters.

The library can be used in conjunction with a variety of platform-agnostic drivers through its `embedded-hal` trait implementations. Both `embedded-hal` v0.2.7 and v1 are supported.

### Compatibility

RPPAL requires a recent release of Raspberry Pi OS. Similar Linux distributions may work, but are unsupported. Both GNU and musl `libc` targets are supported.

**Supported Models:**
- Raspberry Pi A, A+, B, B+, 2B, 3A+, 3B, 3B+, 4B, 5
- Raspberry Pi CM, CM 3, CM 3+, CM 4, CM 5, CM 5 Lite, **CM0 (China)**
- Raspberry Pi 400, 500
- Raspberry Pi Zero, Zero W, Zero 2 W

## 📦 Usage

Add RPPAL as a dependency in `Cargo.toml`:

```toml
[dependencies]
rppal = "0.22"
```

Import the modules you need:

```rust
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rppal::pwm::{Channel, Pwm};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::uart::{Parity, Uart};
```

## 🎯 Examples

### Original Examples

The `examples` directory contains detailed examples for each peripheral:

**GPIO:**
- `gpio_blinkled.rs` - Blink an LED
- `gpio_status.rs` - Read GPIO pin status
- `gpio_servo_softpwm.rs` - Control servo with software PWM
- `gpio_multithreaded_*.rs` - Multi-threaded GPIO access

**I2C:**
- `i2c_ds3231.rs` - DS3231 RTC communication
- `i2c_ssd1306.rs` - OLED display control ⭐
- `i2c_ssd1306_native.rs` - Native OLED control ⭐
- `i2c_scan.rs` - Scan for I2C devices ⭐

**SPI:**
- `spi_25aa1024.rs` - EEPROM communication
- `spi_ws2812*.rs` - **NEW: WS2812 LED control** ⭐

**PWM:**
- `pwm_servo.rs` - Hardware PWM servo control
- `pwm_blinkled.rs` - PWM LED brightness
- `pwm_breath.rs` - Breathing LED effect ⭐

**UART:**
- `uart_blocking_read.rs` - Serial communication

### WS2812 Examples (New)

See the [Quick Start](#-quick-start---ws2812-led-control) section above for WS2812-specific examples.

## ⚙️ Optional Features

By default, all optional features are enabled. You can selectively enable features in `Cargo.toml`:

```toml
[dependencies]
rppal = { version = "0.22", default-features = false, features = ["gpio", "spi"] }
```

Available features:
- `gpio` - GPIO support
- `i2c` - I2C support
- `pwm` - PWM support
- `spi` - SPI support
- `uart` - UART support
- `hal` - embedded-hal trait implementations
- `hal-unproven` - embedded-hal unproven trait implementations

## 🔍 Troubleshooting

### WS2812 Issues

**LEDs not lighting:**
1. Check power supply (5V/5A recommended for 64 LEDs)
2. Verify SPI is enabled: `ls /dev/spi*`
3. Check wiring connections
4. Ensure common ground between Pi and power supply
5. Run diagnostic: `cargo run --example spi_ws2812_diagnostic`

**Incorrect colors:**
1. Lower brightness: `const BRIGHTNESS: u8 = 26;`
2. Try different SPI frequency
3. Add 330Ω resistor on data line
4. Check for voltage drop on long strips

**Detailed troubleshooting:** See `WS2812_故障排查.md`

### General Issues

For general RPPAL issues, refer to the original documentation or open an issue in this repository.

## 🤝 Contributing

Contributions are welcome! This community edition aims to:
- Maintain compatibility with new Raspberry Pi models
- Add support for additional peripherals
- Improve documentation and examples
- Fix bugs and enhance performance

Please open an issue or pull request if you'd like to contribute.

## 📄 License

RPPAL is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Original Author:** Rene van der Meer - Creator of RPPAL
- **Community Contributors:** All who have contributed to this enhanced edition
- **Special Thanks:** To the Rust embedded community for continued support

## 📞 Support

- **Documentation:** [docs.rs/rppal](https://docs.rs/rppal)
- **Issues:** GitHub Issues
- **Discussions:** GitHub Discussions

---

**Note:** This is a community-maintained fork. For the original project, see [golemparts/rppal](https://github.com/golemparts/rppal).
