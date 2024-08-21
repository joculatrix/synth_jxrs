use std::error::Error;

use tokio::sync::broadcast::Sender;
use crate::{message::Message, mixer::SynthMode, osc::{self, oscillator}};

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

        match prop {
            OscProps::Bypass => {
                let value = match value {
                    0.0 => false,
                    1.0 => true,
                    _ => panic!(),
                };
                tx2.send(Message::Bypass(index, value));
            }
            OscProps::Coarse => unsafe {
                tx2.send(Message::Coarse(index, value.to_int_unchecked()));
            }
            OscProps::Fine => {
                tx2.send(Message::Fine(index, value.into()));
            }
            OscProps::Freq => {
                tx2.send(Message::Freq(index, value.into()));
            }
            OscProps::Gain => {
                tx2.send(Message::Gain(index, value.into()));
            }
            OscProps::Mode => unsafe {
                let value = match value.to_int_unchecked() {
                    0 => oscillator::OscMode::MIDI,
                    1 => oscillator::OscMode::Constant,
                    _ => panic!(),
                };
                tx2.send(Message::OscMode(index, value));
            }
            OscProps::Waveform => unsafe {
                let waveform = match value.to_int_unchecked() {
                    0 => osc::wave::Waveform::Noise,
                    1 => osc::wave::Waveform::Saw,
                    3 => osc::wave::Waveform::Square,
                    4 => osc::wave::Waveform::Triangle,
                    _ => osc::wave::Waveform::Sine, // just set to Sine if something goes wrong?
                };
                tx2.send(Message::Waveform(index, waveform));
            }
        }
    });

    let tx3 = tx.clone();

    main_window.on_amp_prop_changed(move |prop, value| {
        match prop {
            AmpProps::Attack => {
                tx3.send(Message::Attack(value.into()));
            }
            AmpProps::Decay => {
                tx3.send(Message::Decay(value.into()));
            }
            AmpProps::Sustain => {
                tx3.send(Message::Sustain(value.into()));
            }
            AmpProps::Release => {
                tx3.send(Message::Release(value.into()));
            }
            AmpProps::Gain => {
                tx3.send(Message::Master(value.into()));
            }
            AmpProps::Mode => unsafe {
                let value = match value.to_int_unchecked() {
                    0 => SynthMode::MIDI,
                    1 => SynthMode::Constant,
                    _ => panic!()
                };
                tx3.send(Message::MixerMode(value));
            }
        }
    });

    main_window.on_set_precision(|value, precision| {
        format!("{value:.0$}", usize::try_from(precision).unwrap_or(0)).into()
    });

    main_window.run()?;

    tx.send(Message::Quit())?;

    Ok(())
}