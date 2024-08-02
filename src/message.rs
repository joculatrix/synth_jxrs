pub enum Message {
    Sample(usize, f64),         // for sending a sample (f64) from an oscillator (usize) to the UI
}