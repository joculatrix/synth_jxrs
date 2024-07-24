use std::fmt::Display;
use super::*;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SizedSample};
use rand::{self, Rng};

pub enum Waveform {
    Noise,
    Saw,
    Sine,
    Square,
    Triangle,
}

impl Waveform {
    fn calc(&self, delta: f64, freq: f64) -> f64 {
        match self {
            // White noise, generated via random amplitudes between -1.0 and 1.0.
            Waveform::Noise => rand::thread_rng().gen::<f64>() * 2.0 - 1.0,
            // Approximation of a sawtooth wave using the first 40 harmonics of a sine wave:
            //
            // for n=[2..=40], f(n) = 2 * (g(n-1) + sin(n * 2πfx)) / π
            // g(n) = g(n-1) + sin(n * 2πfx)
            // g(1) = sin(2πfx)
            //
            // This is far from the fastest way to approximate this wave. Consider replacing or
            // having two versions of the sawtooth.
            Waveform::Saw => {
                let mut res = 0.0f64;
                for i in 1..=40 {
                    res += Waveform::Sine.calc(delta, i as f64 * freq);
                }
                res * (2.0 / PI)
            },
            // The amplitude of a sine wave at a given instant can be calculated by the function:
            // 
            // sin(2πfx)
            // 
            // Where f is the frequency of the wave, and x is the current time. In this implementation,
            // x is represented by "delta".
            Waveform::Sine => (freq * 2.0 * PI * delta).sin(),
            // A square wave is just a sine wave quantized to binary amplitude values of either 1 or -1.
            // Thus, just calculate it as if it were a sine wave, and then turn it into an if-else.
            Waveform::Square => if Waveform::Sine.calc(delta, freq) > 0.0 { 1.0 } else { -1.0 },
            Waveform::Triangle => (Waveform::Sine.calc(delta, freq)).asin() * (2.0 / PI),
        }
    }
}

pub struct Oscillator {
    frequency: f64,
    waveform: Waveform,
}

impl Oscillator {
    fn calc(&self, clock: &SampleClock) -> f64 {
        self.waveform.calc(clock.clock / clock.sample_rate, self.frequency)
    }
}

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

    fn tick(&mut self) {
        self.clock = (self.clock + 1.0) % self.sample_rate;
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
    osc.push(Oscillator{ frequency: 440.0, waveform: Waveform::Saw });

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
        clock.tick();
        let value: T = T::from_sample(0.0025 * osc[0].calc(clock));
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
