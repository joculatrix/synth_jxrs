use std::error::Error;

use tokio::sync::broadcast::Sender;
use crate::{message::Message, osc::{self, oscillator}};

// rust analyzer might flag the following macro as an error, but the project should still compile successfully:
slint::include_modules!();

pub fn run(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
    let main_window = MainWindow::new()?;

    let tx_clone = tx.clone();
    main_window.on_prop_changed(move |index, prop, value| {
        match prop {
            OscProps::Attack => {
                tx_clone.send(Message::Attack(index as usize, value as f64));
            }
            OscProps::Bypass => {
                let value = match value {
                    0.0 => false,
                    1.0 => true,
                    _ => panic!(),
                };
                tx_clone.send(Message::Bypass(index as usize, value));
            }
            OscProps::Decay => {
                tx_clone.send(Message::Decay(index as usize, value as f64));
            }
            OscProps::Freq => {
                tx_clone.send(Message::Freq(index as usize, value as f64));
            }
            OscProps::Gain => {
                tx_clone.send(Message::Gain(index as usize, value as f64));
            }
            OscProps::Mode => {
                let value = match value {
                    0.0 => oscillator::Mode::Freq,
                    1.0 => oscillator::Mode::MIDI,
                    _ => panic!(),
                };
                tx_clone.send(Message::Mode(index as usize, value));
            }
            OscProps::Release => {
                tx_clone.send(Message::Release(index as usize, value as f64));
            }
            OscProps::Sustain => {
                tx_clone.send(Message::Sustain(index as usize, value as f64));
            }
            OscProps::Waveform => {
                let waveform = match value {
                    0.0 => osc::wave::Waveform::Noise,
                    1.0 => osc::wave::Waveform::Saw,
                    3.0 => osc::wave::Waveform::Square,
                    4.0 => osc::wave::Waveform::Triangle,
                    _ => osc::wave::Waveform::Sine, // just set to Sine if something goes wrong?
                };
                tx_clone.send(Message::Waveform(index as usize, waveform));
            }
        }
    });

    main_window.run()?;

    tx.send(Message::Quit())?;

    Ok(())
}

slint::slint! {
    
}