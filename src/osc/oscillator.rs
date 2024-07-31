use super::wave::Waveform;

pub struct Oscillator {
    fm: Option<Box<Oscillator>>,
    fm_range: u16,
    frequency: f64,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(frequency: f64, waveform: Waveform) -> Oscillator {
        Oscillator {
            fm: None,
            fm_range: 100,
            frequency,
            waveform
        }
    }

    pub fn calc(&self, delta: f64) -> f64 {
        if let Some(osc) = &self.fm {
            let frequency = self.frequency + ((self.fm_range / 2) as f64 * osc.calc(delta));
            self.waveform.calc(delta, frequency)
        } else {
            self.waveform.calc(delta, self.frequency)
        }
    }

    pub fn set_fm(&mut self, osc: Option<Oscillator>) {
        if let Some(osc) = osc {
            self.fm = Some(Box::new(osc));
        } else {
            self.fm = None;
        }
    }

    pub fn set_fm_range(&mut self, range: u16) {
        self.fm_range = range;
    }
}