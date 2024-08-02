
use std::collections::VecDeque;

use tokio::sync::broadcast::{Sender, Receiver};

use crate::message::{Channel, Message};

pub const OSC_LEN: u32 = 1024;

#[derive(Clone)]
pub struct App {
    osc_data: [VecDeque<u64>; 1],
}

impl App {
    pub fn new() -> App {
        let mut vec = VecDeque::new();
        for _ in 0..=OSC_LEN {
            vec.push_back(0);
        }

        App {
            osc_data: [vec],
        }
    }

    pub fn osc_data(&mut self, osc: usize) -> &[u64] {
        self.osc_data[osc].make_contiguous()
    }

    pub fn update_osc_data(&mut self, osc: usize, sample: f64) {
        let sample = ((sample + 1.0) * 50.0) as u64; // move range from (-1,1) to (0,2) and then expand upwards to 100
        self.osc_data[osc].pop_front();
        self.osc_data[osc].push_back(sample);
    }
}