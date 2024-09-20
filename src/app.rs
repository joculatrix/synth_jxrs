use std::error::Error;

use tokio::sync::broadcast::Sender;
use crate::{
    message::Message,
    synth::{mixer::SynthMode, osc::{self, oscillator}}
};

// rust analyzer might flag the following macro as an error, but the project should still compile successfully:
slint::include_modules!();

/// Starts the UI and handles the Rust-side connecting operations called by Slint. Actual UI generation is handled
/// in `app.slint`.
pub fn run(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
    let main_window = MainWindow::new()?;

    let tx2 = tx.clone();

    // "prop_changed()" callback in MainWindow of app.slint
    main_window.on_osc_prop_changed(move |index, prop, value| {
        // Index values are hardcoded in app.slint -- if this cast fails, something is very wrong.
        let index = index as usize;

        let msg = match prop {
            OscProps::Bypass => {
                let value = match value {
                    0.0 => false,
                    1.0 => true,
                    _ => panic!(),
                };
                Message::Bypass {
                    oscillator: index,
                    bypass: value
                }
            }
            OscProps::Coarse => unsafe {
                Message::Coarse {
                    oscillator: index,
                    coarse: value.to_int_unchecked()
                }
            }
            OscProps::Fine => {
                Message::Fine {
                    oscillator: index,
                    fine: value.into()
                }
            }
            OscProps::FmRange => unsafe {
                Message::FmRange {
                    oscillator: index,
                    range: value.to_int_unchecked()
                }
            }
            OscProps::Freq => {
                Message::Freq {
                    oscillator: index,
                    freq: value.into()
                }
            }
            OscProps::Gain => {
                Message::Gain {
                    oscillator: index,
                    gain: value.into()
                }
            }
            OscProps::Mode => unsafe {
                let value = match value.to_int_unchecked() {
                    0 => oscillator::PitchMode::MIDI,
                    1 => oscillator::PitchMode::Constant,
                    _ => panic!(),
                };
                Message::PitchMode {
                    oscillator: index,
                    mode: value
                }
            }
            OscProps::Output => unsafe {
                let value = match value.to_int_unchecked() {
                    j if
                        (1..=4).contains(&j)
                        && j as usize - 1 != index => oscillator::OutputMode::Osc(j as usize - 1),
                    _ => oscillator::OutputMode::Master,
                };

                Message::Output {
                    oscillator: index,
                    mode: value
                }
            }
            OscProps::Waveform => unsafe {
                let waveform = match value.to_int_unchecked() {
                    0 => osc::wave::Waveform::Noise,
                    1 => osc::wave::Waveform::Saw,
                    3 => osc::wave::Waveform::Square,
                    4 => osc::wave::Waveform::Triangle,
                    _ => osc::wave::Waveform::Sine, // just set to Sine if something goes wrong?
                };
                Message::Waveform {
                    oscillator: index,
                    waveform
                }
            }
        };

        tx2.send(msg);
    });

    let tx3 = tx.clone();

    main_window.on_amp_prop_changed(move |prop, value| {
        let msg = match prop {
            AmpProps::Attack => {
                Message::Attack(value.into())
            }
            AmpProps::Decay => {
                Message::Decay(value.into())
            }
            AmpProps::Sustain => {
                Message::Sustain(value.into())
            }
            AmpProps::Release => {
                Message::Release(value.into())
            }
            AmpProps::Gain => {
                Message::Master(value.into())
            }
            AmpProps::Mode => unsafe {
                let value = match value.to_int_unchecked() {
                    0 => SynthMode::MIDI,
                    1 => SynthMode::Constant,
                    _ => panic!()
                };
                Message::MixerMode(value)
            }
        };

        tx3.send(msg);
    });

    let tx4 = tx.clone();

    main_window.on_midi_reset(move || {
        tx4.send(Message::ResetMIDI());
    });

    main_window.on_set_precision(|value, precision| {
        format!("{value:.0$}", usize::try_from(precision).unwrap_or(0)).into()
    });

    main_window.run()?;

    tx.send(Message::Quit())?;

    Ok(())
}