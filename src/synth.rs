use std::{array, fmt::Display};
use crate::*;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Host, FromSample, SizedSample, Stream};
use message::Message;
use osc::oscillator::Mode;
use tokio::sync::broadcast::Sender;

pub const NUM_OSCS: usize = 3;

// initialized in build() since runtime calculations can't be used in static definitions,
// and hardcoding 128 frequency values would be messy
pub static mut MIDI_TO_HZ: [f64; 128] = [0.0; 128];

struct StreamWrapper {
    stream: Stream,
}

unsafe impl Send for StreamWrapper { }


pub async fn build(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
    let host = get_host();
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

async fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, tx: Sender<Message>) -> Result<(), Box<dyn Error>>
where
    T: SizedSample + FromSample<f64> + Display,
{
    unsafe {
        super::SAMPLE_RATE = config.sample_rate.0 as f64;
        if let Err(e) = osc::init_tables() {
            return Err(e.into());
        }
    }

    let channels = config.channels as usize;

    let mut oscs = Vec::with_capacity(NUM_OSCS);
    for i in 0..NUM_OSCS {
        oscs.push(Mutex::new(Oscillator::new(i)));
    }
    let oscs = Arc::new(oscs);

    let mut samps_iter = 0;
    let mut rx = tx.subscribe();

    let stream_oscs = Arc::clone(&oscs);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let stream_oscs = Arc::clone(&stream_oscs);
            output(data, channels, stream_oscs, tx.to_owned(), &mut samps_iter)
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    let stream = StreamWrapper{ stream };

    stream.stream.play()?;

    loop { tokio::select! {
        Ok(msg) = rx.recv() => {
            match msg {
                Message::Quit() => {
                    return Ok(());
                }
                Message::Bypass(i, b) => {
                    oscs[i].lock().unwrap().bypass = b;
                }
                Message::Freq(i, f) => {
                    oscs[i].lock().unwrap().set_freq(f);
                }
                Message::Mode(i, m) => {
                    oscs[i].lock().unwrap().set_mode(m);
                }
                Message::NoteOn(pitch, velocity) => unsafe {
                    oscs.iter().for_each(|osc| {
                        let mut lock = osc.lock().unwrap();
                        if lock.get_mode() == Mode::MIDI {
                            lock.amp.note_on(pitch);
                            lock.note_on(pitch);
                        }
                    })
                }
                Message::NoteOff(pitch) => {
                    oscs.iter().for_each(|osc| {
                        let mut lock = osc.lock().unwrap();
                        if lock.get_mode() == Mode::MIDI {
                            lock.amp.note_off(pitch);
                            lock.note_off(pitch);
                        }
                    })
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

fn output<T>(
    output: &mut [T],
    channels: usize,
    oscs: Arc<Vec<Mutex<Oscillator>>>,
    tx: Sender<Message>,
    samps_iter: &mut u32
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

fn get_host() -> Host {
    cpal::default_host()
}



#[cfg(test)]
mod tests {
    use cpal::traits::HostTrait;

    use super::*;

    #[test]
    fn device_is_available() {
        let host = get_host();
        let device = host.default_output_device();

        assert!(device.is_some(), "Failed to acquire output device");
    }
}
