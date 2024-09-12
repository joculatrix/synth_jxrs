use std::error::Error;

use midi_control::MidiMessage;
use midir::{MidiInput, MidiInputConnection};
use tokio::sync::broadcast::Sender;

use crate::message::Message;

type Connection = MidiInputConnection<Sender<Message>>;

/// Connects to the first active MIDI device and listens for input, sending appropriate signals to [`synth`].
/// 
/// If no MIDI devices are found, the thread will continue running and will make another attempt to connect to
/// a MIDI device when sent a button press from the UI in [`app`].
/// 
/// [`app`]:    crate::app
/// [`synth`]:  crate::synth
pub async fn listen(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
    let mut rx = tx.subscribe();

    let mut connection = connect(tx.clone());

    loop { tokio::select! {
        Ok(msg) = rx.recv() => {
            match msg {
                Message::Quit() => {
                    if let Some(connection) = connection.take() {
                        connection.close();
                    }
                    break;
                }
                Message::ResetMIDI() => {
                    if let Some(connection) = connection.take() {
                        connection.close();
                    }
                    connection = connect(tx.clone());
                }
                _ => (),
            }
        }
    }}

    Ok(())
}

fn connect(tx: Sender<Message>) -> Option<Connection> {
    // client_name is currently unused by the midir code, and its intended purpose is unexplained,
    // so for now I'm passing in an empty string:
    let stream = MidiInput::new("").unwrap();

    let inputs = stream.ports();
    let port = match inputs.len() {
        0 => {
            eprintln!("MIDI error: no MIDI input ports found");
            return None
        },
        // TODO: allow port selection through UI instead of forcing first port
        _ => &inputs[0],
    };

    match stream.connect(
        port,
        "synth_jxrs_port",
        |_timestamp, msg, tx| {
            if let Err(e) = parse_message(msg, tx) {
                eprintln!("MIDI error: {e}");
            }
        },
        tx,
    ) {
        Ok(c) => {
            eprintln!("Successfully connected to MIDI input.");
            Some(c)
        }
        Err(e) => {
            eprintln!("MIDI error: {e}");
            None
        }
    }
}

/// Communicates to the rest of the program based on received MIDI input.
/// 
/// Currently, only `NoteOn` and `NoteOff` events are supported. Other signals do nothing.
fn parse_message(msg: &[u8], tx: &mut Sender<Message>) -> Result<(), Box<dyn Error>> {
    match MidiMessage::from(msg) {
        MidiMessage::Invalid => {
            return Err("invalid MIDI received".into());
        }
        MidiMessage::NoteOn(_channel, key_event) => {
            tx.send(Message::NoteOn{
                pitch: key_event.key,
                _velocity: key_event.value
            })?;
        }
        MidiMessage::NoteOff(_channel, key_event) => {
            tx.send(Message::NoteOff{pitch: key_event.key})?;
        }
        MidiMessage::PolyKeyPressure(_, _) => {}
        MidiMessage::ControlChange(_, _) => {}
        MidiMessage::ProgramChange(_, _) => {}
        MidiMessage::ChannelPressure(_, _) => {}
        MidiMessage::PitchBend(_channel, lsb, msb) => {
            tx.send(Message::PitchBend{ lsb, msb })?;
        }
        MidiMessage::SysEx(_) => {}
    }
    Ok(())
}