use std::time::Instant;

pub struct Amplifier {
    pub adsr: Envelope,
    last_amplitude: f64,
    note_on: bool,
    start_time: Option<Instant>,
    release_time: Option<Instant>,
}

impl Amplifier {
    pub fn default() -> Amplifier {
        Amplifier {
            adsr: Envelope::default(),
            last_amplitude: 0.0,
            note_on: false,
            start_time: None,
            release_time: None,
        }
    }

    pub fn new(adsr: Envelope) -> Amplifier {
        Amplifier {
            adsr,
            last_amplitude: 0.0,
            note_on: false,
            start_time: None,
            release_time: None,
        }
    }

    pub fn note_on(&mut self) {
        self.note_on = true;
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
            self.release_time.take();
        }
    }

    pub fn note_off(&mut self) {
        self.note_on = false;
        if self.release_time.is_none() {
            self.start_time = Some(Instant::now());
            self.start_time.take();
        }
    }

    pub fn calc(&mut self, sample_in: f64) -> f64 {
        let amplitude = if self.note_on {
            let since_attack = Instant::now()
                .duration_since(self.start_time
                .expect("Start time shouldn't be None when note_on is true"))
                .as_secs_f64();

            if since_attack <= self.adsr.attack {
                since_attack / self.adsr.attack
            } else if since_attack > self.adsr.attack + self.adsr.decay {
                self.adsr.sustain
            } else {
                (since_attack - self.adsr.attack) / self.adsr.decay
            }
        } else if let Some(release_time) = self.release_time {
            if self.last_amplitude <= 0.0001 {
                self.release_time.take();
                0.0
            } else {
                let since_release = Instant::now()
                    .duration_since(release_time)
                    .as_secs_f64();

                (since_release / self.adsr.release) * (0.0 - self.adsr.sustain) + self.adsr.sustain
            }
        } else {
            0.0
        };

        self.last_amplitude = amplitude;

        sample_in * amplitude
    }
}



pub struct Envelope {
    pub attack: f64,    // time to reach max amplitude (seconds)
    pub decay: f64,     // time after attack to reach sustain amplitude (seconds)
    sustain: f64,       // amplitude multiplier once sustain time is reached (0.0-1.0)
    pub release: f64,   // time for sound to diminish after key is released (seconds)
}

impl Envelope {
    pub fn default() -> Envelope {
        Envelope {
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
        }
    }

    pub fn set_sustain(&mut self, sustain: f64) {
        self.sustain = sustain.min(1.0).max(0.0);   // constrain to 0.0-1.0
    }
}