use tokio::sync::broadcast::{Sender, Receiver};

use crate::osc::wave::Waveform;

#[derive(Clone)]
pub enum Message {
    Freq(usize, f64),           // for sending frequency edits from UI to oscillator
    Sample(usize, f64),         // for sending a sample (f64) from an oscillator (usize) to the UI
    Quit(),                     // for sending exit signal between threads
    Waveform(usize, Waveform)   // for using the UI to change the waveform of an oscillator
}

pub struct Channel {
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>
}