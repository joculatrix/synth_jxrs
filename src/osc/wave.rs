use std::f64::consts::PI;
use rand::{self, Rng};

pub enum Waveform {
    Noise,
    Saw,
    Sine,
    Square,
    Triangle,
}

impl Waveform {
    /// Generates a sample of the wave amplitude for a given point in time. Currently,
    /// this is used in [`osc::init_tables()`] to produce a single cycle of each wave shape
    /// for the samples to later be referenced by [`Oscillator`]s as needed.
    /// 
    /// ['osc::init_tables()`]: super::init_tables
    /// [`Oscillator`]:         super::oscillator::Oscillator
    pub fn calc(&self, delta: f64, freq: f64) -> f64 {
        match self {
            // White noise, generated via random amplitudes between -1.0 and 1.0.
            Waveform::Noise => rand::thread_rng().gen::<f64>() * 2.0 - 1.0,
            // Approximation of a sawtooth wave using the first 40 harmonics of a sine wave:
            //
            // f(n) = 2 * (sin(1 * 2πfx) + sin(2 * 2πfx) + sin(3 * 2πfx) + ... + sin(n * 2πfx)) / π
            Waveform::Saw => {
                let mut res = 0.0f64;
                for i in 1..=40 {
                    res += Waveform::Sine.calc(delta, i as f64 * freq);
                }
                res * (2.0 / PI)
            },
            // The amplitude of a sine wave at a given instant can be calculated by the function:
            // 
            // sin(2πfx)
            // 
            // Where f is the frequency of the wave, and x is the current time. In this implementation,
            // x is represented by "delta".
            Waveform::Sine => (freq * 2.0 * PI * delta).sin(),
            // A square wave is just a sine wave quantized to binary amplitude values of either 1 or -1.
            // Thus, just calculate it as if it were a sine wave, and then turn it into an if-else.
            Waveform::Square => if Waveform::Sine.calc(delta, freq) > 0.0 { 1.0 } else { -1.0 },
            Waveform::Triangle => (Waveform::Sine.calc(delta, freq)).asin() * (2.0 / PI),
        }
    }

    /// Retrieve the appropriate sample from the lookup table.
    pub fn get_sample(&self, phase: f64) -> f64 {
        let i = phase as usize;

        unsafe {
            match self {
                Waveform::Noise => super::NOISE_TABLE[i],
                Waveform::Saw => super::SAW_TABLE[i],
                Waveform::Sine => super::SINE_TABLE[i],
                Waveform::Square => super::SQUARE_TABLE[i],
                Waveform::Triangle => super::TRI_TABLE[i],
            }
        } 
    }
}