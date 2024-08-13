use std::error::Error;

use midi_control::MidiMessage;
use midir::MidiInput;
use tokio::sync::broadcast::Sender;

use crate::message::Message;


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
        |timestamp, msg, tx| {
            if let Err(e) = parse_message(timestamp, msg, tx) {
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

fn parse_message(timestamp: u64, msg: &[u8], tx: &mut Sender<Message>) -> Result<(), Box<dyn Error>> {
    match MidiMessage::from(msg) {
        MidiMessage::Invalid => {
            return Err("invalid MIDI received".into());
        }
        MidiMessage::NoteOn(_channel, key_event) => unsafe {
            tx.send(Message::NoteOn(
                key_event.key,
                key_event.value
            ))?;
        }
        MidiMessage::NoteOff(_channel, key_event) => {
            tx.send(Message::NoteOff(key_event.key))?;
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