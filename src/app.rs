
use tokio::sync::broadcast::{Sender, Receiver};

use crate::message::Message;

const OSC_LEN: usize = 1024;

pub struct App {
    osc_data: [[u64; OSC_LEN]; 1],
    channel: Channel,
}

impl App {
    pub fn new(tx: Sender<Message>) -> App {
        let rx = tx.subscribe();
        App {
            osc_data: [[0; OSC_LEN]; 1],
            channel: Channel{ tx, rx },
        }
    }

    pub fn osc_data(&self, osc: usize) -> [u64; OSC_LEN] {
        self.osc_data[osc]
    }

    fn update_osc_data(&mut self, osc: usize, sample: f64) {
        let sample = ((sample + 1.0) * 50.0) as u64; // move range from (-1,1) to (0,2) and then expand upwards to 100
        self.osc_data[osc][OSC_LEN - 1] = sample;
    }
}


struct Channel {
    tx: Sender<Message>,
    rx: Receiver<Message>
}