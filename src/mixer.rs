use crate::amp;

pub struct Mixer {
    inputs: Vec<Input>,
    master: f64,
}

impl Mixer {
    pub fn new() -> Mixer {
        Mixer {
            inputs: vec![],
            master: 0.0,
        }
    }

    pub fn add_input(&mut self, amp: amp::Amplifier) {
        self.inputs.push(Input { input: amp, gain: 0.0 });
    }
}



struct Input {
    input: amp::Amplifier,
    gain: f64,
}