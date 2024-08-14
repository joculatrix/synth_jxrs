use crate::{amp::Amplifier, synth};

use super::{wave::Waveform, *};

pub struct Oscillator {
    pub amp: Amplifier,
    pub bypass: bool,
    fm: Option<Box<Oscillator>>,
    fm_range: u16,
    frequency: f64,
    id: usize,
    midi_notes: Vec<u8>,
    mode: Mode,
    phase: f64,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(id: usize) -> Oscillator {
        Oscillator {
            amp: Amplifier::default(),
            bypass: true,
            fm: None,
            fm_range: 100,
            frequency: 440.0,
            id,
            midi_notes: vec![],
            mode: Mode::Freq,
            phase: 0.0,
            waveform: Waveform::Sine,
        }
    }

    pub fn calc(&mut self) -> f64 {
        if self.bypass {
            return 0.0;
        }

        let mut frequency = self.frequency;

        let res = if self.waveform == Waveform::Noise {
            Waveform::Noise.calc(0.0, 0.0)
        } else {
            self.waveform.get_sample(self.phase)
        };

        // for frequency modulation
        if let Some(ref mut osc) = &mut self.fm {
            frequency += (self.fm_range / 2) as f64 * osc.calc();
        }

        // iterate to next sample
        unsafe {
            let table_length = TABLE_LENGTH as f64;

            // frequency should not affect white noise
            if self.waveform == Waveform::Noise {
                self.phase += 1.0;
            } else {
                self.phase +=
                    frequency * table_length / crate::SAMPLE_RATE;
            }
            if self.phase >= table_length as f64 {
                self.phase -= table_length as f64;
            }
        }

        if self.mode == Mode::MIDI {
            self.amp.calc(res)
        } else {
            res
        }
    }

    pub fn get_mode(&self) -> Mode {
        self.mode
    }

    pub fn note_on(&mut self, pitch: u8) {
        if self.midi_notes.is_empty() {
            self.phase = 0.0;
        }
        if !self.midi_notes.contains(&pitch) {
            self.midi_notes.insert(0, pitch);
            unsafe {
                self.set_freq(synth::MIDI_TO_HZ[pitch as usize]);
            }
        }
    }

    pub fn note_off(&mut self, pitch: u8) {
        for i in 0..self.midi_notes.len() {
            if self.midi_notes[i] == pitch {
                self.midi_notes.remove(i);
                break;
            }
        }
        if !self.midi_notes.is_empty() {
            unsafe {
                self.set_freq(synth::MIDI_TO_HZ[self.midi_notes[0] as usize]);
            }
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


#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Mode {
    Freq,
    MIDI,
}