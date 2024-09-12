use std::{
    array, 
    fmt::Display,
    sync::LazyLock,
};
use crate::*;
use message::Message;
use mixer::Mixer;
use osc::oscillator::{PitchMode, Oscillator};
use tokio::sync::broadcast::Sender;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SizedSample, Stream};

pub mod amp;
pub mod mixer;
pub mod osc;

/// The number of [`Oscillator`]s the synthesizer should have. Currently, this is a convenience identifier
/// for a value that shouldn't be edited. In order for this number to have the power to quickly alter the
/// program, [`app`] would have to be heavily refactored to allow Slint to dynamically and cleanly generate
/// the UI for an arbitrary amount of oscillators.
pub const NUM_OSCS: usize = 4;

/// A table of MIDI pitch values `[0..127]` and their corresponding frequencies in Hz.
/// 
/// This array is referenced in [`build()`] to ensure its initialization at startup.
pub static MIDI_TO_HZ: LazyLock<[f64; 128]> = LazyLock::new(|| {
    array::from_fn(|i| {
        let val = f64::powf(2.0, (i as f64 - 69.0) / 12.0);
        440.0 * val
    })
});

/// The amount of audio samples played per second by the audio device. This value is set in [`run()`]
/// after the audio device has been detected and connected to.
static mut SAMPLE_RATE: f64 = 48000.0;

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
/// This function also calls the initialization of [`MIDI_TO_HZ`] and determines the sample format of
/// the audio device to be used as the type of [`run()`], which this function `await`s.
pub async fn build(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let Some(device) = host.default_output_device() else {
        return Err("Failed to identify an output device.".into());
    };
    // println!("Output device: {}", device.name().unwrap());

    let config = device.default_output_config()?;
    // println!("Output config: {:?}", config);

    let _ = &*MIDI_TO_HZ;

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
        SAMPLE_RATE = config.sample_rate.0 as f64;
    }
    osc::init_tables();
    let channels = config.channels as usize;

    // initialize oscillators
    let mut oscs = Vec::with_capacity(NUM_OSCS);
    for _ in 0..NUM_OSCS {
        oscs.push(Mutex::new(Oscillator::new()));
    }
    let oscs = Arc::new(oscs);
    let stream_oscs = Arc::clone(&oscs);

    // initialize mixer
    let mixer = Arc::new(Mutex::new(Mixer::new()));
    let stream_mixer = Arc::clone(&mixer);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let stream_mixer = Arc::clone(&stream_mixer);
            let stream_oscs = Arc::clone(&stream_oscs);
            output(stream_mixer, data, channels, stream_oscs)
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
                Message::Attack(attack) => {
                    mixer.lock().unwrap().amp.adsr.attack = attack;
                }
                Message::Bypass(i, bypass) => {
                    oscs[i].lock().unwrap().bypass = bypass;
                }
                Message::Coarse(i, coarse) => {
                    oscs[i].lock().unwrap().detune_coarse(coarse);
                }
                Message::Decay(decay) => {
                    mixer.lock().unwrap().amp.adsr.decay = decay;
                }
                Message::Fine(i, fine) => {
                    oscs[i].lock().unwrap().detune_fine(fine);
                }
                Message::FmRange(i, range) => {
                    oscs[i].lock().unwrap().set_fm_range(range);
                }
                Message::Freq(i, freq) => {
                    oscs[i].lock().unwrap().set_freq(freq);
                }
                Message::Gain(i, gain) => {
                    oscs[i].lock().unwrap().set_gain(gain);
                }
                Message::Master(gain) => {
                    mixer.lock().unwrap().set_gain(gain);
                }
                Message::MixerMode(mode) => {
                    mixer.lock().unwrap().set_mode(mode);
                }
                Message::NoteOn{pitch, _velocity} => {
                    oscs.iter().for_each(|osc| {
                        let mut lock = osc.lock().unwrap();
                        if lock.get_mode() == PitchMode::MIDI {
                            lock.note_on(pitch);
                        }
                    });
                    mixer.lock().unwrap().amp.note_on(pitch);
                }
                Message::NoteOff{pitch} => {
                    oscs.iter().for_each(|osc| {
                        let mut lock = osc.lock().unwrap();
                        if lock.get_mode() == PitchMode::MIDI {
                            lock.note_off(pitch);
                        }
                    });
                    mixer.lock().unwrap().amp.note_off(pitch);
                }
                Message::PitchBend{lsb, msb} => {
                    oscs.iter().for_each(|osc| {
                        let mut lock = osc.lock().unwrap();
                        lock.pitch_bend(lsb, msb);
                    });
                }
                Message::PitchMode(i, mode) => {
                    oscs[i].lock().unwrap().set_mode(mode);
                }
                Message::Output(i, mode) => {
                    let mut lock = oscs[i].lock().unwrap();
                    match lock.get_output_mode() {
                        osc::oscillator::OutputMode::Master => (),
                        osc::oscillator::OutputMode::Osc(j) => {
                            oscs[j].lock().unwrap().remove_fm_in(i);
                        }
                    }
                    match mode {
                        osc::oscillator::OutputMode::Master => {
                            lock.set_output(mode);
                        }
                        osc::oscillator::OutputMode::Osc(j) => {
                            lock.set_output(mode);
                            oscs[j].lock().unwrap().add_fm_in(i);
                        }
                    }
                }
                Message::Release(release) => {
                    mixer.lock().unwrap().amp.adsr.release = release;
                }
                Message::Sustain(sustain) => {
                    mixer.lock().unwrap().amp.adsr.set_sustain(sustain);
                }
                Message::Waveform(i, waveform) => {
                    oscs[i].lock().unwrap().set_waveform(waveform);
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
    mixer: Arc<Mutex<Mixer>>,
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
            let mut lock = oscs[i].lock().unwrap();
            match lock.get_output_mode() {
                osc::oscillator::OutputMode::Master => {
                    amps[i] = lock.calc();
                }
                osc::oscillator::OutputMode::Osc(j) => {
                    oscs[j].lock().unwrap().fm_sample_in(i, lock.calc());
                }
            }
        }

        let mut value: f64 = 0.0;
        for amp in amps {
            value += amp;
        }
        let value = T::from_sample(
            mixer.lock().unwrap().calc(0.25 * value)
        );

        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

/// Converts a gain value measured in decibels (`db`) to an amplitude value.
pub fn db_to_amp(db: f64) -> f64 {
    f64::powf(10.0, db / 20.0)
}