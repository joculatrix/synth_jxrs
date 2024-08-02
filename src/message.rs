use tokio::sync::broadcast::{Sender, Receiver};

#[derive(Clone)]
pub enum Message {
    Sample(usize, f64),         // for sending a sample (f64) from an oscillator (usize) to the UI
}

pub struct Channel {
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>
}