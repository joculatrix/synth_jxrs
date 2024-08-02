use std::fmt::Display;
use crate::*;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SizedSample};
use message::Message;
use tokio::sync::broadcast::Sender;

static mut SAMPS_PER_SCOPE: u32 = 0;


pub fn build(tx: Sender<Message>) -> Result<(), &'static str> {
    let host = get_host();
    let Some(device) = host.default_output_device() else {
        return Err("Failed to identify an output device.");
    };
    // println!("Output device: {}", device.name().unwrap());

    let config = device.default_output_config().unwrap();
    // println!("Output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), tx),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), tx),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), tx),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), tx),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), tx),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), tx),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), tx),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), tx),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), tx),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), tx),
        _ => todo!(),
    }

    Ok(())
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, tx: Sender<Message>)
where
    T: SizedSample + FromSample<f64> + Display,
{
    unsafe {
        super::SAMPLE_RATE = config.sample_rate.0 as f64;
        SAMPS_PER_SCOPE = app::SCOPE_LEN / SAMPLE_RATE as u32;
        osc::init_tables();
    }

    let channels = config.channels as usize;

    let mut osc = Oscillator::new(0);

    let mut samps_iter = 0;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            output(data, channels, &mut osc, tx.to_owned(), &mut samps_iter)
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    ).unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(5000));
}

fn output<T>(
    output: &mut [T],
    channels: usize,
    osc: &mut Oscillator,
    tx: Sender<Message>,
    samps_iter: &mut u32
)
where
    T: SizedSample + FromSample<f64> + Display
{
    for frame in output.chunks_mut(channels) {
        let amp = 0.75 * osc.calc();
        let value: T = T::from_sample(amp);
        unsafe {
            if *samps_iter == SAMPS_PER_SCOPE {
                tx.send(Message::Sample(0, amp));
                *samps_iter = 0;
            } else {
                *samps_iter += 1;
            }
        }
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
