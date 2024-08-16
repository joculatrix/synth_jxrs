use std::error::Error;

use midi_control::MidiMessage;
use midir::MidiInput;
use tokio::sync::broadcast::Sender;

use crate::message::Message;

/// Connects to the first active MIDI device and listens for input, sending appropriate signals to [`synth`].
/// 
/// # Errors
/// 
/// This function currently returns an `Err()` if no MIDI port is found, ending the task. This is fine for development
/// purposes, as [`synth`] and [`app`] continue running without it, but ideally, the program would either repeatedly
/// check for input ports or allow the user to trigger another attempt. The user would also ideally be able to select
/// different ports or devices, in case multiple are connected, as the function currently allows no choice.
/// 
/// [`app`]:    crate::app
/// [`synth`]:  crate::synth
pub async fn listen(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
    // client_name is currently unused by the midir code, and its intended purpose is unexplained,
    // so for now I'm passing in an empty string:
    let stream = MidiInput::new("").unwrap();

    let inputs = stream.ports();
    let port = match inputs.len() {
        0 => return Err("no MIDI input ports found".into()),
        // TODO: allow port selection through UI instead of forcing first port
        _ => &inputs[0],
    };

    let mut rx = tx.subscribe();

    let connection = stream.connect(
        port,
        "synth_jxrs_port",
        |_timestamp, msg, tx| {
            if let Err(e) = parse_message(msg, tx) {
                // do some sort of logging
            }
        },
        tx,
    )?;

    loop { tokio::select! {
        Ok(msg) = rx.recv() => {
            match msg {
                Message::Quit() => {
                    break;
                }
                _ => (),
            }
        }
    }}

    connection.close();

    Ok(())
}

/// Communicates to the rest of the program based on received MIDI input.
/// 
/// Currently, only `NoteOn` and `NoteOff` events are supported.
/// 
/// # Panics
/// 
/// `todo!()` is invoked for every MIDI input event aside from NoteOn and NoteOff until it comes
/// time to consider which should be implemented.
fn parse_message(msg: &[u8], tx: &mut Sender<Message>) -> Result<(), Box<dyn Error>> {
    match MidiMessage::from(msg) {
        MidiMessage::Invalid => {
            return Err("invalid MIDI received".into());
        }
        MidiMessage::NoteOn(_channel, key_event) => {
            tx.send(Message::NoteOn{
                pitch: key_event.key,
                velocity: key_event.value
            })?;
        }
        MidiMessage::NoteOff(_channel, key_event) => {
            tx.send(Message::NoteOff{pitch: key_event.key})?;
        }
        MidiMessage::PolyKeyPressure(_, _) => todo!(),
        MidiMessage::ControlChange(_, _) => todo!(),
        MidiMessage::ProgramChange(_, _) => todo!(),
        MidiMessage::ChannelPressure(_, _) => todo!(),
        MidiMessage::PitchBend(_, _, _) => todo!(),
        MidiMessage::SysEx(_) => todo!(),
    }
    Ok(())
}