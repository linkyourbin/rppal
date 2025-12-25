// pwm_breath.rs - Alternating LED breathing effect using dual hardware PWM.
//
// Creates an alternating breathing effect on two LEDs.
// When PWM0 brightens, PWM1 dims, and vice versa.
//
// Remember to add a resistor of an appropriate value in series, to prevent
// exceeding the maximum current rating of the GPIO pin and the LED.
//
// Interrupting the process by pressing Ctrl-C causes the application to exit
// immediately without disabling the PWM channel.

use std::error::Error;
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};

// Breathing effect configuration
const PWM_FREQUENCY: f64 = 1000.0;        // 1 kHz - high enough to avoid visible flicker
const BREATH_CYCLE_MS: u64 = 3000;        // 3 seconds for one complete breath cycle
const STEP_DELAY_MS: u64 = 20;            // 20ms between updates (50 updates/sec)
const MIN_DUTY_CYCLE: f64 = 0.01;         // Minimum brightness (1%)
const MAX_DUTY_CYCLE: f64 = 1.0;          // Maximum brightness (100%)

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting alternating LED breathing effect...");
    println!("GPIO12 (Pin 32) - Hardware PWM0");
    println!("GPIO13 (Pin 33) - Hardware PWM1");
    println!("Press Ctrl+C to stop\n");

    // Initialize PWM channel 0 (GPIO12, physical pin 32) - starts dim
    let pwm0 = Pwm::with_frequency(
        Channel::Pwm0,
        PWM_FREQUENCY,
        MIN_DUTY_CYCLE,
        Polarity::Normal,
        true,
    )?;

    // Initialize PWM channel 1 (GPIO13, physical pin 33) - starts bright
    let pwm1 = Pwm::with_frequency(
        Channel::Pwm1,
        PWM_FREQUENCY,
        MAX_DUTY_CYCLE,
        Polarity::Normal,
        true,
    )?;

    println!("Alternating breathing effect active...");
    println!("When PWM0 brightens, PWM1 dims (and vice versa)");
    println!("Frequency: {} Hz", PWM_FREQUENCY);
    println!("Cycle time: {} ms", BREATH_CYCLE_MS);
    println!("Update rate: {} ms\n", STEP_DELAY_MS);

    // Calculate total steps in one breath cycle
    let steps_per_cycle = BREATH_CYCLE_MS / STEP_DELAY_MS;

    // Main breathing loop
    let mut step = 0u64;
    loop {
        // Calculate current position in breathing cycle (0.0 to 1.0)
        let position = (step % steps_per_cycle) as f64 / steps_per_cycle as f64;

        // Use sine wave for smooth breathing effect
        // sin gives values from -1 to 1, we convert to 0 to 1
        let sine_value = ((position * 2.0 * PI).sin() + 1.0) / 2.0;

        // Map sine value to duty cycle range for PWM0
        let duty_cycle0 = MIN_DUTY_CYCLE + sine_value * (MAX_DUTY_CYCLE - MIN_DUTY_CYCLE);

        // PWM1 uses inverted duty cycle (when PWM0 is bright, PWM1 is dim)
        let duty_cycle1 = MAX_DUTY_CYCLE - sine_value * (MAX_DUTY_CYCLE - MIN_DUTY_CYCLE);

        // Update both PWM channels with alternating duty cycles
        pwm0.set_duty_cycle(duty_cycle0)?;
        pwm1.set_duty_cycle(duty_cycle1)?;

        // Print current brightness for both channels
        if step % 25 == 0 {  // Print every 500ms
            print!("\rPWM0: {:3.0}%  PWM1: {:3.0}% ", duty_cycle0 * 100.0, duty_cycle1 * 100.0);
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }

        // Wait before next update
        thread::sleep(Duration::from_millis(STEP_DELAY_MS));

        step += 1;
    }

    // This code is unreachable in the infinite loop, but kept for reference
    // When interrupted, the PWM channel will be automatically disabled
    #[allow(unreachable_code)]
    Ok(())
}
