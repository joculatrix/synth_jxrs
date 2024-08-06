
use std::collections::VecDeque;

use tokio::sync::broadcast::{Sender, Receiver};

use crate::{message::{Channel, Message}, synth};

pub const SCOPE_LEN: usize = 128;

#[derive(Clone)]
pub struct App {
    osc_data: [VecDeque<u64>; synth::NUM_OSCS],
}

impl App {
    pub fn new() -> App {
        let mut vecs = core::array::from_fn(|_|
            VecDeque::with_capacity(SCOPE_LEN)
        );
        vecs.iter_mut().for_each(|v| v.resize(SCOPE_LEN, 0));

        App {
            osc_data: vecs,
        }
    }

    pub fn osc_data(&mut self, osc: usize) -> &[u64] {
        self.osc_data[osc].make_contiguous();
        self.osc_data[osc].as_slices().0
    }

    pub fn update_osc_data(&mut self, osc: usize, sample: f64) {
        let sample = ((sample + 1.0) * 50.0) as u64; // move range from (-1,1) to (0,2) and then expand upwards to 100
        self.osc_data[osc].pop_front();
        self.osc_data[osc].push_back(sample);
    }
}