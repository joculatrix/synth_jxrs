use std::{
    error::Error,
    f64::consts::PI,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast::{self};

// modules:
mod app;
mod message;
mod midi;
mod synth;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, _rx) = broadcast::channel(10);

    let mut handles = vec![];

    let tx2 = tx.clone();
    handles.push(tokio::spawn(async move {
        if let Err(e) = synth::build(tx2).await {
            eprintln!("Audio error: {e}");
        }
    }));

    let tx3 = tx.clone();
    handles.push(tokio::spawn(async move {
        if let Err(e) = midi::listen(tx3).await {
            eprintln!("MIDI error: {e}");
        }
    }));

    if let Err(e) = app::run(tx) {
        eprintln!("Application error: {e}");
    }
    
    for handle in handles {
        handle.await?;
    }

    Ok(())
}