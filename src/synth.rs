use std::fmt::Display;
use super::*;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SizedSample};

struct SampleClock {
    sample_rate: f64,
    clock: f64,
}

impl SampleClock {
    fn new(sample_rate: f64) -> SampleClock {
        SampleClock {
            sample_rate,
            clock: 0.0,
        }
    }

    fn tick(&mut self) -> f64 {
        self.clock = (self.clock + 1.0) % self.sample_rate;

        self.clock / self.sample_rate
    }
}

pub fn build() -> Result<(), &'static str> {
    let host = get_host();
    let Some(device) = host.default_output_device() else {
        return Err("Failed to identify an output device.");
    };
    println!("Output device: {}", device.name().unwrap());

    let config = device.default_output_config().unwrap();
    println!("Output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into()),
        _ => todo!(),
    }

    Ok(())
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig)
where
    T: SizedSample + FromSample<f64> + Display,
{
    let sample_rate = config.sample_rate.0 as f64;
    let mut clock = SampleClock::new(sample_rate);
    let channels = config.channels as usize;

    let mut osc = vec![];
    osc.push(Oscillator::new(440.0, Waveform::Sine));

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            output(data, channels, &mut clock, &osc)
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    ).unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
}

fn output<T>(output: &mut [T], channels: usize, clock: &mut SampleClock, osc: &Vec<Oscillator>)
where
    T: SizedSample + FromSample<f64> + Display
{
    for frame in output.chunks_mut(channels) {
        let delta = clock.tick();
        let value: T = T::from_sample(0.75 * osc[0].calc(delta));
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
