
use tokio::sync::broadcast::{Sender, Receiver};

use crate::message::{Channel, Message};

const OSC_LEN: usize = 1024;

#[derive(Clone,Copy)]
pub struct App {
    osc_data: [[u64; OSC_LEN]; 1],
}

impl App {
    pub fn new() -> App {
        App {
            osc_data: [[0; OSC_LEN]; 1],
        }
    }

    pub fn osc_data(&self, osc: usize) -> [u64; OSC_LEN] {
        self.osc_data[osc]
    }

    pub fn update_osc_data(&mut self, osc: usize, sample: f64) {
        let sample = ((sample + 1.0) * 50.0) as u64; // move range from (-1,1) to (0,2) and then expand upwards to 100
        for i in 0..(OSC_LEN - 1) {
            self.osc_data[osc][i] = self.osc_data[osc][i+1];
        }
        self.osc_data[osc][OSC_LEN - 1] = sample;
    }
}