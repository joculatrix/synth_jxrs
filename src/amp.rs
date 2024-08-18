use std::{collections::BTreeSet, time::Instant};

/// Manages the amplitude, both statically and based on MIDI signals and time, of an [`Oscillator`].
/// 
/// [`Oscillator`]: crate::osc::oscillator::Oscillator
pub struct Amplifier {
    /// The set of MIDI pitches `[0..127]` currently held by the MIDI input device.
    active_notes: BTreeSet<u8>,
    /// Contains duration information for changing amplitude throughout a note's lifetime.
    pub adsr: Envelope,
    /// The overall volume modifier of the signal. Stored in the struct, this field is measured as an
    /// amplitude multiplier, e.g. some value typically in the range `[0..1]`. However, when modified by
    /// user input, the public-facing value is measured in dB as that is more commonly used by audio
    /// professionals and musicians.
    gain: f64,
    /// This field is used to keep track of what amplitude the signal should start at when releasing.
    last_amplitude: f64,
    /// When legato is `false`, envelopes restart from the beginning when two notes overlap. When legato
    /// is `true`, overlapping notes will continue with the same, uninterrupted envelope.
    legato: bool,
    /// Stores whether there is currently any active note being held.
    note_on: bool,
    /// If a note is currently held, the `Instant` of when the most recent note began.
    start_time: Option<Instant>,
    /// If the envelope is currently in the release phase, the `Instant` of when the last note was released.
    release_time: Option<Instant>,
}

impl Amplifier {
    /// Returns a new `Amplifier` using the default [`Envelope`].
    pub fn default() -> Amplifier {
        Amplifier::new(Envelope::default())
    }

    /// Returns a new 'Amplifier', accepting any [`Envelope`] into the `adsr` parameter.
    pub fn new(adsr: Envelope) -> Amplifier {
        Amplifier {
            active_notes: BTreeSet::new(),
            adsr,
            gain: 1.0,
            last_amplitude: 0.0,
            legato: false,
            note_on: false,
            start_time: None,
            release_time: None,
        }
    }

    /// Sends the MIDI "NoteOn" signal to `self`.
    /// 
    /// If another note is already held, this function will add the new note to the `Amplifier`'s internal
    /// set of active notes. This function also restarts the [`Envelope`] from the beginning of the attack,
    /// if `self.legato` is `false`.
    pub fn note_on(&mut self, pitch: u8) {
        self.note_on = true;
        if self.start_time.is_none() { // if no other note is currently playing
            self.active_notes.insert(pitch);
            self.start_time = Some(Instant::now());
            self.release_time.take();
        } else if !self.active_notes.contains(&pitch) { // if another note is playing but not this one
            self.active_notes.insert(pitch);
            if !self.legato {
                self.start_time = Some(Instant::now());
            }
        }
    }

    /// Sends the MIDI "NoteOff" signal to `self`.
    /// 
    /// The note affected by the signal is removed from the `Amplifier`'s internal set of active notes.
    /// If that was the only actively held note, the [`Envelope`] releases.
    pub fn note_off(&mut self, pitch: u8) {
        if self.release_time.is_none() && self.active_notes.len() <= 1 {
            self.note_on = false;
            self.start_time.take();
            self.release_time = Some(Instant::now());
        }
        self.active_notes.remove(&pitch);
    }

    /// Multiplies `sample_in` by an amplitude modifier based on `self`'s [`Envelope`] and the history of
    /// MIDI signals sent to this `Amplifier`.
    /// 
    /// The growth and decay of amplitude during `attack`, `decay`, and `release` are linear. Note also that
    /// if `self.adsr.decay > 0.0`, the starting amplitude, or the amplitude reached by the end of the `attack`
    /// duration if there is one, is 1.0, allowing for the amplitude to decrease towards the `sustain` amplitude.
    /// 
    /// If no notes are held, and a duration greater than 'self.adsr.release' has passed, this function
    /// returns 0.0.
    pub fn calc(&mut self, sample_in: f64) -> f64 {
        let amplitude = if self.note_on {
            let since_attack = Instant::now()
                .duration_since(self.start_time
                .expect("Start time shouldn't be None when note_on is true"))
                .as_secs_f64();

            // The Amplifier's last_amplitude field is used to keep track of the last generated
            // amplitude of the "on" phase of the note's lifetime. This is used to prevent the
            // note from jumping up to sustain amplitude if it wasn't reached before the note ended.
            self.last_amplitude = if since_attack <= self.adsr.attack { // attack
                if self.adsr.decay > 0.0 {
                    // attack towards 1.0 so decay can decrease to sustain amplitude
                    since_attack / self.adsr.attack
                } else {
                    // attack towards sustain amplitude
                    self.adsr.sustain * since_attack / self.adsr.attack
                }
            } else if since_attack > self.adsr.attack + self.adsr.decay { // sustain
                self.adsr.sustain
            } else { // decay
                1.0 - ((1.0 - self.adsr.sustain) * (since_attack - self.adsr.attack)) / self.adsr.decay
            };

            self.last_amplitude

        } else if let Some(release_time) = self.release_time {
            let since_release = Instant::now()
                .duration_since(release_time)
                .as_secs_f64();

            if since_release >= self.adsr.release {
                self.release_time.take();
                0.0
            } else {
                self.last_amplitude - (since_release / self.adsr.release)
            }
        } else {
            0.0
        };

        sample_in * amplitude * self.gain
    }

    /// Returns the `gain` field of `self`.
    pub fn get_gain(&mut self) -> f64 {
        self.gain
    }

    /// Modifies the `gain` property of `self`.
    /// 
    /// The value of the `gain_gb` argument should be measured in dB. Often this value is between -60 and 0.
    /// The gain in dB will be converted to an amplitude modifier between 0.0 and 1.0 before assignment.
    pub fn set_gain(&mut self, gain_db: f64) {
        self.gain = crate::synth::db_to_amp(gain_db);
    }
}


/// An ADSR amplitude envelope for use by an [`Amplifier`].
pub struct Envelope {
    /// The time (in seconds) the sound takes to reach its peak amplitude after beginning.
    pub attack: f64,
    /// The time (in seconds) the sound takes to decay to its sustain amplitude, after the attack.
    pub decay: f64,
    /// The amplitude `[0..1]` the sound should hold at, after the attack and decay times lapse. 
    sustain: f64,
    /// The time (in seconds) the sound takes from when the note stops to reach an amplitude of 0.
    pub release: f64,
}

impl Envelope {
    /// Returns a new `Envelope` with the default values: instant `attack`, `decay`, and `release`
    /// times, and a `sustain` amplitude of 1.0.
    pub fn default() -> Envelope {
        Envelope {
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
        }
    }

    /// Replaces the `sustain` field of `self` with the given value.
    /// 
    /// The input value is constrained to the range `[0..1]`.
    pub fn set_sustain(&mut self, sustain: f64) {
        self.sustain = sustain.min(1.0).max(0.0);
    }
}