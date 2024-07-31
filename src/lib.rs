use cpal::Host;

pub mod osc;
pub mod synth;

pub use synth::build;

use osc::init_tables;
use osc::oscillator::Oscillator;
use osc::wave::Waveform;

static mut SAMPLE_RATE: f64 = 48000.0;

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