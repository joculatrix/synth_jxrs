use tokio::sync::broadcast::{Sender, Receiver};

use crate::{
    amp::{Amplifier, Envelope},
    app,
    midi,
    osc::{
        oscillator::{self, Oscillator},
        wave::Waveform
    },
};

/// The message type sent between tasks by the application's broadcast channel.
/// 
/// Most types are targeted at specific [`Oscillator`]s. For these, the first parameter is 
/// always the index of the Oscillator in the `Vec<Oscillator>` of [`synth`].
/// 
/// [`synth`]:      crate::synth
#[derive(Clone,Debug)]
pub enum Message {
    /// Sent by the UI in [`app`] to modify the `attack` value of an [`Oscillator`]'s [`Envelope`].
    Attack(usize, f64),
    /// Sent by the UI in [`app`] to modify the `bypass` value of an [`Oscillator`].
    Bypass(usize, bool),
    /// Sent by the UI in [`app`] to modify the `decay` value of an [`Oscillator`]'s [`Envelope`].
    Decay(usize, f64),
    /// Sent by the UI in [`app`] to modify the `frequency` value of an [`Oscillator`].
    Freq(usize, f64),
    /// Sent by the UI in [`app`] to modify the `gain` value of an [`Oscillator`]'s [`Amplifier`].
    Gain(usize, f64),
    /// Sent by the UI in [`app`] to modify the `mode` value of an [`Oscillator`].
    Mode(usize, oscillator::Mode), 
    /// Sent by [`midi`] to signal a MIDI note-on. Velocity is currently unused.
    NoteOn{pitch: u8, velocity: u8},
    /// Sent by [`midi`] to signal a MIDI note-off.
    NoteOff{pitch: u8},
    /// Sent by the UI in [`app`] to modify the `release` value of an [`Oscillator`]'s [`Envelope`].
    Release(usize, f64),
    /// Currently unused. Previously used to send samples to the UI to display an oscilloscope.
    Sample(usize, f64),
    /// Sent by the UI in [`app`] to modify the `sustain` value of an [`Oscillator`]'s [`Envelope`].
    Sustain(usize, f64),
    /// Sent to inform various tasks to shutdown.
    Quit(),
    /// Sent by the UI in [`app`] to modify the [`Waveform`] of an [`Oscillator`].
    Waveform(usize, Waveform),
}