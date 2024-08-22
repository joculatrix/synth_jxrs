#[allow(unused)]
use crate::{
    amp::{Amplifier, Envelope},
    app,
    midi,
    mixer::{self, Mixer},
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
    /// Sent by the UI in [`app`] to modify the `attack` value of the [`Mixer`]'s [`Envelope`].
    Attack(f64),

    /// Sent by the UI in [`app`] to modify the `bypass` value of an [`Oscillator`].
    Bypass(usize, bool),

    /// Sent by the UI in [`app`] to modify the `coarse` detune value of an [`Oscillator`].
    Coarse(usize, i32),

    /// Sent by the UI in [`app`] to modify the `decay` value of the [`Mixer`]'s [`Envelope`].
    Decay(f64),

    /// Sent by the UI in [`app`] to modify the `fine` detune value of an [`Oscillator`].
    Fine(usize, f64),

    /// Sent by the UI in [`app`] to modify the `fm_range` of an [`Oscillator`].
    FmRange(usize, u16),

    /// Sent by the UI in [`app`] to modify the `frequency` value of an [`Oscillator`].
    Freq(usize, f64),
    
    /// Sent by the UI in [`app`] to modify the `gain` value of an [`Oscillator`]'s [`Amplifier`].
    Gain(usize, f64),

    /// Sent by the UI in [`app`] to modify the master gain of the [`Mixer`].
    /// 
    /// [`Mixer`]:  crate::mixer::Mixer
    Master(f64),

    /// Sent by the UI in [`app`] to modify the `mode` value of the [`Mixer`].
    MixerMode(mixer::SynthMode),

    /// Sent by [`midi`] to signal a MIDI note-on. Velocity is currently unused.
    NoteOn{pitch: u8, velocity: u8},

    /// Sent by [`midi`] to signal a MIDI note-off.
    NoteOff{pitch: u8},

    /// Sent by the UI in [`app`] to modify the `mode` value of an [`Oscillator`].
    OscMode(usize, oscillator::OscMode), 

    /// Sent by the UI in [`app`] to modify where an [`Oscillator`]'s signal outputs to.
    Output(usize, oscillator::OutputMode),

    /// Sent by the UI in [`app`] to modify the `release` value of the [`Mixer`]'s [`Envelope`].
    Release(f64),

    /// Sent by the UI in [`app`] to notify [`midi`] to retry the connection to the MIDI device.
    ResetMIDI(),

    /// Currently unused. Previously used to send samples to the UI to display an oscilloscope.
    _Sample(usize, f64),

    /// Sent by the UI in [`app`] to modify the `sustain` value of the [`Mixer`]'s [`Envelope`].
    Sustain(f64),

    /// Sent to inform various tasks to shutdown.
    Quit(),

    /// Sent by the UI in [`app`] to modify the [`Waveform`] of an [`Oscillator`].
    Waveform(usize, Waveform),
}