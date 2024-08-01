// use statements for in-library use:
use std::f64::consts::PI;
use cpal::Host;
use osc::init_tables;
use osc::oscillator::Oscillator;
use osc::wave::Waveform;

// modules:
mod amp;
mod filter;
mod mixer;
mod osc;
mod synth;

// use statements for re-importing to public API:
pub use synth::build;

// statics:
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