/// Struct for managing over-arching volume and mixing for the synthesizer.
pub struct Mixer {
    /// The overall volume modifier of the signal. Stored in the struct, this field is measured as an
    /// amplitude multiplier, e.g. some value typically in the range `[0..1]`. However, when modified by
    /// user input, the public-facing value is measured in dB as that is more commonly used by audio
    /// professionals and musicians.
    master_gain: f64,
}

impl Mixer {
    /// Returns a new `Mixer` with a `master_gain` value of 1.0.
    pub fn new() -> Mixer {
        Mixer {
            master_gain: 1.0,
        }
    }

    /// Multiples `sample_in` by `self.master_gain`.
    pub fn calc(&self, sample_in: f64) -> f64 {
        sample_in * self.master_gain
    }

    /// Modifies the `master_gain` property of `self`.
    /// 
    /// The value of the `gain_db` argument should be measured in dB. Often this value is between -60 and 0.
    /// The gain in dB will be converted to an amplitude modifier between 0.0 and 1.0 before assignment.
    pub fn set_gain(&mut self, gain_db: f64) {
        self.master_gain = crate::synth::db_to_amp(gain_db);
    }
}