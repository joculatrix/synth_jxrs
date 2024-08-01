use std::{fs::{self, File}, io::{self, Write}, ops::Index};
use wave::Waveform;

pub mod oscillator;
pub mod wave;

/// Number of samples stored in the reference tables for each waveform.
const TABLE_LENGTH: usize = 1024;

static mut NOISE_TABLE:     [f64; TABLE_LENGTH] = [0.0; TABLE_LENGTH];
static mut SAW_TABLE:       [f64; TABLE_LENGTH] = [0.0; TABLE_LENGTH];
static mut SINE_TABLE:      [f64; TABLE_LENGTH] = [0.0; TABLE_LENGTH];
static mut SQUARE_TABLE:    [f64; TABLE_LENGTH] = [0.0; TABLE_LENGTH];
static mut TRI_TABLE:       [f64; TABLE_LENGTH] = [0.0; TABLE_LENGTH];

/// Calculate [`TABLE_LENGTH`] samples for each [`Waveform`], storing in
/// a static array that can be used to oscillate later without constantly
/// calculating trigonometric functions on the fly.
pub unsafe fn init_tables() -> Result<(), &'static str> {
    for i in 0..TABLE_LENGTH {
        let delta = i as f64 / TABLE_LENGTH as f64;
            
        NOISE_TABLE[i] = Waveform::Noise.calc(delta, 1.0);
        SAW_TABLE[i] = Waveform::Saw.calc(delta, 1.0);
        SINE_TABLE[i] = Waveform::Sine.calc(delta, 1.0);
        SQUARE_TABLE[i] = Waveform::Square.calc(delta, 1.0);
        TRI_TABLE[i] = Waveform::Triangle.calc(delta, 1.0);
    }

    /* let mut tables = [NOISE_TABLE, SAW_TABLE, SINE_TABLE, SQUARE_TABLE, TRI_TABLE];

     match fs::read_to_string("data/tables") {
        Ok(data) => {
            println!("Reading tables from stored data...");

            let mut lines = data.lines();

            tables.iter_mut()
                  .for_each(|table| {
                        let mut samples = lines.next().unwrap().split(' ');
                        table.iter_mut()
                            .for_each(|mut x|
                                *x = samples.next().unwrap().parse::<f64>().unwrap()
                            );
                });

            println!("Done.")
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                println!("Generating wave tables...");
                
                match fs::create_dir("data") {
                    Ok(_) => (),
                    Err(e) => {
                        if e.kind() != io::ErrorKind::AlreadyExists {
                            return Err("Couldn't create data directory.")
                        }
                    }
                }

                let mut f = File::create("data/tables")
                    .expect("Couldn't create data file.");

                for i in 0..TABLE_LENGTH {
                    let delta = i as f64 / TABLE_LENGTH as f64;
                        
                    NOISE_TABLE[i] = Waveform::Noise.calc(delta, 1.0);
                    SAW_TABLE[i] = Waveform::Saw.calc(delta, 1.0);
                    SINE_TABLE[i] = Waveform::Sine.calc(delta, 1.0);
                    SQUARE_TABLE[i] = Waveform::Square.calc(delta, 1.0);
                    TRI_TABLE[i] = Waveform::Triangle.calc(delta, 1.0);
                }

                for table in tables {
                    for sample in table {
                        f.write(sample.to_string().as_bytes());
                        f.write(&[b' ']);
                    }
                    f.write(&[b'\n']);
                }
                
                println!("Done.");
            } else { return Err("Unknown error reading tables.dat.") }
        }
    }
    */
    Ok(())
}