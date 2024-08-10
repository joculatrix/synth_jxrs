use std::{
    error::Error,
    f64::consts::PI,
    io::{self, Stdout},
    sync::{Arc, Mutex},
};
use cpal::{Host, Stream};
use message::Message;
use osc::oscillator::Oscillator;
use tokio::sync::broadcast::{self, error::RecvError, Receiver, Sender};

// modules:
mod amp;
mod app;
mod filter;
mod message;
mod mixer;
mod osc;
mod synth;

// statics:
static mut SAMPLE_RATE: f64 = 48000.0;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, _rx) = broadcast::channel(10);

    let tx2 = tx.clone();
    let handle = tokio::spawn(async move {
        synth::build(tx2).await.unwrap();
    });

    app::run(tx);
    
    Ok(())
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
