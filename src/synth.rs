use std::fmt::Display;
use crate::*;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SizedSample};
use message::Message;
use tokio::sync::broadcast::Sender;

pub const NUM_OSCS: usize = 3;
static mut SAMPS_PER_SCOPE: u32 = 0;

struct StreamWrapper {
    stream: Stream,
}

unsafe impl Send for StreamWrapper { }


pub async fn build(tx: Sender<Message>) -> Result<(), &'static str> {
    let host = get_host();
    let Some(device) = host.default_output_device() else {
        return Err("Failed to identify an output device.");
    };
    // println!("Output device: {}", device.name().unwrap());

    let config = device.default_output_config().unwrap();
    // println!("Output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), tx).await,
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), tx).await,
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), tx).await,
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), tx).await,
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), tx).await,
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), tx).await,
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), tx).await,
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), tx).await,
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), tx).await,
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), tx).await,
        _ => todo!(),
    }

    Ok(())
}

async fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, tx: Sender<Message>)
where
    T: SizedSample + FromSample<f64> + Display,
{
    unsafe {
        super::SAMPLE_RATE = config.sample_rate.0 as f64;
        SAMPS_PER_SCOPE = app::SCOPE_LEN as u32 / SAMPLE_RATE as u32;
        osc::init_tables();
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
    ).unwrap();

    let stream = StreamWrapper{ stream };

    stream.stream.play().unwrap();

    loop { tokio::select! {
        Ok(msg) = rx.recv() => {
            match msg {
                Message::Quit() => {
                    return;
                }
                Message::Freq(i, f) => {
                    oscs[i].lock().unwrap().set_freq(f);
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
            unsafe {
                if *samps_iter == SAMPS_PER_SCOPE {
                    tx.send(Message::Sample(i, amp));
                    *samps_iter = 0;
                } else {
                    *samps_iter += 1;
                }
            }
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
