use std::{array, fmt::Display};
use crate::*;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SizedSample, Stream};
use message::Message;
use osc::oscillator::Mode;
use tokio::sync::broadcast::Sender;

/// The number of [`Oscillator`]s the synthesizer should have. Currently, this is a convenience identifier
/// for a value that shouldn't be edited. In order for this number to have the power to quickly alter the
/// program, [`app`] would have to be heavily refactored to allow Slint to dynamically and cleanly generate
/// the UI for an arbitrary amount of oscillators.
pub const NUM_OSCS: usize = 3;

/// A table of MIDI pitch values `[0..127]` and their corresponding frequencies in Hz.
/// 
/// The values in this table are inserted in [`build()`], as non-constant functions can't be used to
/// initialize statics.
pub static mut MIDI_TO_HZ: [f64; 128] = [0.0; 128];

/// A wrapper for `cpal::Stream` to force it to implement the `Send` trait.
/// 
/// Because [`run()`] must repeatedly `await` async operations to receive messages from other tasks,
/// but the audio stream must continue to function while this happens, the stream has to implement
/// the `Send` trait in order to be held until its intended shutdown.
struct StreamWrapper {
    stream: Stream,
}

unsafe impl Send for StreamWrapper { }

/// Connects to the audio device and begins running the audio stream task.
/// 
/// This function also calculates the values of [`MIDI_TO_HZ`] and determines the sample format of
/// the audio device to be used as the type of [`run()`], which this function `await`s.
pub async fn build(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let Some(device) = host.default_output_device() else {
        return Err("Failed to identify an output device.".into());
    };
    // println!("Output device: {}", device.name().unwrap());

    let config = device.default_output_config()?;
    // println!("Output config: {:?}", config);

    unsafe {
        MIDI_TO_HZ = array::from_fn(|i| {
            let val = f64::powf(2.0, (i as f64 - 69.0) / 12.0);
            440.0 * val
        });
    }

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), tx).await?,
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), tx).await?,
        _ => return Err("Unsupported sample format.".into()),
    }

    Ok(())
}

/// Initializes wavetables and oscillators, runs audio playback stream, and handles incoming messages from other tasks.
/// 
/// The `cpal::Stream` used to play audio uses [`output()`] as a callback.
async fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, tx: Sender<Message>) -> Result<(), Box<dyn Error>>
where
    T: SizedSample + FromSample<f64> + Display,
{
    unsafe {
        super::SAMPLE_RATE = config.sample_rate.0 as f64;
        osc::init_tables();
    }

    let channels = config.channels as usize;

    let mut oscs = Vec::with_capacity(NUM_OSCS);
    for _ in 0..NUM_OSCS {
        oscs.push(Mutex::new(Oscillator::new()));
    }
    let oscs = Arc::new(oscs);

    let stream_oscs = Arc::clone(&oscs);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let stream_oscs = Arc::clone(&stream_oscs);
            output(data, channels, stream_oscs)
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    let stream = StreamWrapper{ stream };

    stream.stream.play()?;

    let mut rx = tx.subscribe();

    loop { tokio::select! {
        Ok(msg) = rx.recv() => {
            match msg {
                Message::Quit() => {
                    return Ok(());
                }
                Message::Attack(i, a) => {
                    oscs[i].lock().unwrap().amp.adsr.attack = a;
                }
                Message::Bypass(i, b) => {
                    oscs[i].lock().unwrap().bypass = b;
                }
                Message::Decay(i, d) => {
                    oscs[i].lock().unwrap().amp.adsr.decay = d;
                }
                Message::Freq(i, f) => {
                    oscs[i].lock().unwrap().set_freq(f);
                }
                Message::Gain(i, g) => {
                    oscs[i].lock().unwrap().amp.set_gain(g);
                }
                Message::Mode(i, m) => {
                    oscs[i].lock().unwrap().set_mode(m);
                }
                Message::NoteOn{pitch, velocity: _} => {
                    oscs.iter().for_each(|osc| {
                        let mut lock = osc.lock().unwrap();
                        if lock.get_mode() == Mode::MIDI {
                            lock.amp.note_on(pitch);
                            lock.note_on(pitch);
                        }
                    })
                }
                Message::NoteOff{pitch} => {
                    oscs.iter().for_each(|osc| {
                        let mut lock = osc.lock().unwrap();
                        if lock.get_mode() == Mode::MIDI {
                            lock.amp.note_off(pitch);
                            lock.note_off(pitch);
                        }
                    })
                }
                Message::Release(i, r) => {
                    oscs[i].lock().unwrap().amp.adsr.release = r;
                }
                Message::Sustain(i, s) => {
                    oscs[i].lock().unwrap().amp.adsr.set_sustain(s);
                }
                Message::Waveform(i, w) => {
                    oscs[i].lock().unwrap().set_waveform(w);
                }
                _ => ()
            } 
        }
        else => { }
    }}
}

/// Callback for `cpal::Stream` used by [`run()`].
/// 
/// This function generates and outputs to the audio device each sample.
fn output<T>(
    output: &mut [T],
    channels: usize,
    oscs: Arc<Vec<Mutex<Oscillator>>>,
)
where
    T: SizedSample + FromSample<f64> + Display
{
    for frame in output.chunks_mut(channels) {
        let mut amps: [f64; NUM_OSCS] = [0.0; NUM_OSCS];

        for i in 0..NUM_OSCS {
            let amp = oscs[i].lock().unwrap().calc();
            amps[i] = amp;
        }
        
        let mut value: f64 = 0.0;

        for amp in amps {
            value += amp;
        }

        let value = T::from_sample(0.3 * value);

        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}