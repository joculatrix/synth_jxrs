use std::{
    error::Error,
    f64::consts::PI,
    io::{self, Stdout},
    sync::{Arc, Mutex},
};
use cpal::{Host, Stream};
use osc::oscillator::Oscillator;
use tokio::sync::broadcast::{self};

// modules:
mod amp;
mod app;
mod filter;
mod message;
mod midi;
mod mixer;
mod osc;
mod synth;

// statics:
static mut SAMPLE_RATE: f64 = 48000.0;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, _rx) = broadcast::channel(10);

    let mut handles = vec![];

    let tx2 = tx.clone();
    handles.push(tokio::spawn(async move {
        synth::build(tx2).await.unwrap();
    }));

    let tx3 = tx.clone();
    handles.push(tokio::spawn(async move {
        midi::listen(tx3).await.unwrap();
    }));

    app::run(tx);
    
    for handle in handles {
        handle.await?;
    }

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
