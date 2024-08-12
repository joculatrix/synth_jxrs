use super::{wave::Waveform, *};

pub struct Oscillator {
    fm: Option<Box<Oscillator>>,
    fm_range: u16,
    frequency: f64,
    id: usize,
    mode: Mode,
    phase: f64,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(id: usize) -> Oscillator {
        Oscillator {
            fm: None,
            fm_range: 100,
            frequency: 440.0,
            id,
            mode: Mode::Freq,
            phase: 0.0,
            waveform: Waveform::Sine,
        }
    }

    pub fn calc(&mut self) -> f64 {
        let mut frequency = self.frequency;
        let res = self.waveform.get_sample(self.phase);

        // for frequency modulation
        if let Some(ref mut osc) = &mut self.fm {
            frequency += (self.fm_range / 2) as f64 * osc.calc();
        }

        // iterate to next sample
        unsafe {
            let table_length = TABLE_LENGTH as f64;

            self.phase +=
                frequency * table_length / crate::SAMPLE_RATE;
            
            if self.phase >= table_length as f64 {
                self.phase -= table_length as f64;
            }
        }

        res
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

    pub fn set_freq(&mut self, freq: f64) {
        self.frequency = freq;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }
}


#[derive(Clone)]
pub enum Mode {
    Freq,
    MIDI,
}