use super::wave::Waveform;

pub struct Oscillator {
    frequency: f64,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(frequency: f64, waveform: Waveform) -> Oscillator {
        Oscillator {
            frequency,
            waveform
        }
    }

    pub fn calc(&self, delta: f64) -> f64 {
        self.waveform.calc(delta, self.frequency)
    }
}