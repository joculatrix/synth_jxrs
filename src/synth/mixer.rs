use super::amp::Amplifier;

/// Struct for managing over-arching volume and mixing for the synthesizer.
pub struct Mixer {
    /// The overall volume modifier of the signal. Stored in the struct, this field is measured as an
    /// amplitude multiplier, e.g. some value typically in the range `[0..1]`. However, when modified by
    /// user input, the public-facing value is measured in dB as that is more commonly used by audio
    /// professionals and musicians.
    master_gain: f64,
    /// The [`Amplifier`] that handles MIDI signals and envelope calculation for this `Mixer`.
    pub amp: Amplifier,
    /// The current [`SynthMode`] of the synthesizer.
    mode: SynthMode,
}

impl Mixer {
    /// Returns a new `Mixer` with a `master_gain` value of 1.0.
    pub fn new() -> Mixer {
        Mixer {
            master_gain: 1.0,
            amp: Amplifier::default(),
            mode: SynthMode::MIDI,
        }
    }

    /// If `self.mode` is [`SynthMode::MIDI`], calls [`Amplifier::calc()`] on `sample_in` before multiplying
    /// by `self.master_gain`.
    /// 
    /// Otherwise, just multiplies `sample_in` by `self.master_gain`.
    pub fn calc(&mut self, sample_in: f64) -> f64 {
        if self.mode == SynthMode::MIDI {
            self.amp.calc(sample_in) * self.master_gain
        } else {
            sample_in * self.master_gain
        }
    }

    /// Modifies the `master_gain` property of `self`.
    /// 
    /// The value of the `gain_db` argument should be measured in dB. Often this value is between -60 and 0.
    /// The gain in dB will be converted to an amplitude modifier between 0.0 and 1.0 before assignment.
    pub fn set_gain(&mut self, gain_db: f64) {
        self.master_gain = crate::synth::db_to_amp(gain_db);
    }

    /// Replaces `self.mode` with `mode`.
    pub fn set_mode(&mut self, mode: SynthMode) {
        self.mode = mode;
    }
}

/// Determines whether the envelope of the synth should follow MIDI signals.
#[derive(Clone,Debug,PartialEq)]
pub enum SynthMode {
    Constant,
    MIDI,
}