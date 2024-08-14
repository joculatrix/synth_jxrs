use tokio::sync::broadcast::{Sender, Receiver};

use crate::osc::{oscillator, wave::Waveform};

/// The message type sent between threads by the application's broadcast channel.
/// 
/// Most types are targeted at specific oscillators. For these, the first parameter is 
/// always the index of the oscillator.
#[derive(Clone,Debug)]
pub enum Message {
    Attack(usize, f64),             // for sending attack value from UI to oscillator
    Bypass(usize, bool),            // for enabling/disabling an oscillator
    Decay(usize, f64),              // for sending decay value from UI to oscillator
    Freq(usize, f64),               // for sending frequency edits from UI to oscillator
    Gain(usize, f64),               // for sending gain value from UI to oscillator
    Mode(usize, oscillator::Mode),  // for toggling an oscillator between constant frequency (Freq) and MIDI-based
    NoteOn(u8, u8),                 // MIDI NoteOn: (pitch, velocity)
    NoteOff(u8),                    // MIDI NoteOff: (pitch)
    Release(usize, f64),            // for sending release value from UI to oscillator
    Sample(usize, f64),             // for sending a sample (f64) from an oscillator (usize) to the UI
    Sustain(usize, f64),            // for sending sustain value from UI to oscillator
    Quit(),                         // for sending exit signal between threads
    Waveform(usize, Waveform)       // for using the UI to change the waveform of an oscillator
}

pub struct Channel {
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>
}