use std::error::Error;

use tokio::sync::broadcast::Sender;
use crate::{message::Message, osc::{self, oscillator}};

// rust analyzer might flag the following macro as an error, but the project should still compile successfully:
slint::include_modules!();

/// Starts the UI and handles the Rust-side connecting operations called by Slint. Actual UI generation is handled
/// in `app.slint`.
pub fn run(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
    let main_window = MainWindow::new()?;

    let tx_clone = tx.clone();

    // "prop_changed()" callback in MainWindow of app.slint
    main_window.on_prop_changed(move |index, prop, value| {
        // Index values are hardcoded in app.slint -- if this cast fails, something is very wrong.
        let index = index as usize;

        match prop {
            OscProps::Attack => {
                tx_clone.send(Message::Attack(index, value.into()));
            }
            OscProps::Bypass => {
                let value = match value {
                    0.0 => false,
                    1.0 => true,
                    _ => panic!(),
                };
                tx_clone.send(Message::Bypass(index, value));
            }
            OscProps::Decay => {
                tx_clone.send(Message::Decay(index, value.into()));
            }
            OscProps::Freq => {
                tx_clone.send(Message::Freq(index, value.into()));
            }
            OscProps::Gain => {
                tx_clone.send(Message::Gain(index, value.into()));
            }
            OscProps::Mode => unsafe {
                let value = match value.to_int_unchecked() {
                    0 => oscillator::Mode::Freq,
                    1 => oscillator::Mode::MIDI,
                    _ => panic!(),
                };
                tx_clone.send(Message::Mode(index, value));
            }
            OscProps::Release => {
                tx_clone.send(Message::Release(index, value.into()));
            }
            OscProps::Sustain => {
                tx_clone.send(Message::Sustain(index, value.into()));
            }
            OscProps::Waveform => unsafe {
                let waveform = match value.to_int_unchecked() {
                    0 => osc::wave::Waveform::Noise,
                    1 => osc::wave::Waveform::Saw,
                    3 => osc::wave::Waveform::Square,
                    4 => osc::wave::Waveform::Triangle,
                    _ => osc::wave::Waveform::Sine, // just set to Sine if something goes wrong?
                };
                tx_clone.send(Message::Waveform(index, waveform));
            }
        }
    });

    main_window.global::<Util>().on_set_precision(|value, precision| {
        format!("{value:.0$}", usize::try_from(precision).unwrap_or(0)).into()
    });

    main_window.run()?;

    tx.send(Message::Quit())?;

    Ok(())
}