use std::{
    error::Error,
    f64::consts::PI,
    io::{self, Stdout},
    sync::{Arc, Mutex},
};
use app::App;
use cpal::{Host, Stream};
use message::Message;
use osc::oscillator::Oscillator;
use ratatui::{crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
}, prelude::CrosstermBackend, Terminal};
use tokio::sync::broadcast::{self, error::RecvError, Receiver, Sender};

// modules:
mod amp;
mod app;
mod filter;
mod message;
mod mixer;
mod osc;
mod synth;
mod ui;

// statics:
static mut SAMPLE_RATE: f64 = 48000.0;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, _rx) = broadcast::channel(10);

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let tx2 = tx.clone();
    let handle = tokio::spawn(async move {
        synth::build(tx2).await.unwrap();
    });

    run_app(&mut terminal, app, tx).await?;
    handle.await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: App,
    tx: Sender<Message>
) -> Result<(), Box<dyn Error>> {
    let mut rx = tx.subscribe();
    let app = Arc::new(Mutex::new(app));
    let ui_app_ref = Arc::clone(&app);

    let handle = tokio::spawn(async move { loop {
        match rx.recv().await {
            Ok(msg) => match msg {
                Message::Freq(_, _) => {}
                Message::Sample(i, samp) => {
                    app.lock().unwrap().update_osc_data(i, samp);
                }
                Message::Quit() => break,
            }
            Err(RecvError::Lagged(_)) => (),
            Err(e) => break,
        }
    }});

    loop {
        {
            let mut app = ui_app_ref.lock().unwrap();
            terminal.draw(|f| ui::ui(f, &mut app))?;
        }
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    match tx.send(Message::Quit()) {
                        Ok(_) => break,
                        Err(_) => panic!(),
                    }
                }
            }
        }
    }

    handle.await?;

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
