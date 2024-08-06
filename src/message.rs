use tokio::sync::broadcast::{Sender, Receiver};

#[derive(Clone)]
pub enum Message {
    Freq(usize, f64),           // for sending frequency edits from UI to oscillator
    Sample(usize, f64),         // for sending a sample (f64) from an oscillator (usize) to the UI
    Quit(),                     // for sending exit signal between threads
}

pub struct Channel {
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>
}