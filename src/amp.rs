pub struct Amplifier {
    adsr: Envelope,
}

impl Amplifier {
    pub fn default() -> Amplifier {
        Amplifier {
            adsr: Envelope::default(),
        }
    }

    pub fn new(adsr: Envelope) -> Amplifier {
        Amplifier {
            adsr,
        }
    }
}



pub struct Envelope {
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,
}

impl Envelope {
    pub fn default() -> Envelope {
        Envelope {
            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            release: 0.0,
        }
    }

    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> Envelope {
        Envelope {
            attack,
            decay,
            sustain,
            release,
        }
    }
}