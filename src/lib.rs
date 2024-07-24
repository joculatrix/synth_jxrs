use std::f64::consts::PI;

use cpal::{traits::HostTrait, Host};

fn get_host() -> Host {
    cpal::default_host()
}

pub enum Waveform {
    Noise,
    Saw,
    Sine,
    Square,
    Triangle,
}

impl Waveform {
    fn calc(&self, delta: f64, freq: u16) -> f64 {
        match self {
            Waveform::Noise => todo!(),
            Waveform::Saw => todo!(),
            // The amplitude of a sine wave at a given instant can be calculated by the function:
            // 
            // sin(2πfx)
            // 
            // Where f is the frequency of the wave, and x is the current time. In this implementation,
            // x is represented as the variable "delta".
            Waveform::Sine => ((freq as f64) * 2.0 * PI * delta).sin(),
            // A square wave is just a sine wave quantized to binary amplitude values of either 1 or -1.
            // Thus, just calculate it as if it were a sine wave, and then turn it into an if-else.
            Waveform::Square => if Waveform::Sine.calc(delta, freq) > 0.0 { 1.0 } else { -1.0 },
            Waveform::Triangle => todo!(),
        }
    }
}

pub struct Oscillator {
    frequency: u16,
    waveform: Waveform,
}

impl Oscillator {
    fn calc(&self, delta: f64) -> f64 {
        self.waveform.calc(delta, self.frequency)
    }
}

pub struct Synth {
    oscillators: Vec<Oscillator>,
}

impl Synth {
    pub fn run() -> Result<(), &'static str> {
        let host = get_host();
        let device = host.default_output_device() else {
            return Err("Failed to identify an output device.");
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_is_available() {
        let host = get_host();
        let device = host.default_output_device();

        assert!(device.is_some(), "Failed to acquire output device");
    }
}