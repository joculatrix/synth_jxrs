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
pub fn init_tables() {
    for i in 0..TABLE_LENGTH {
        let delta = i as f64 / TABLE_LENGTH as f64;
        unsafe {
            NOISE_TABLE[i] = Waveform::Noise.calc(delta, 1.0);
            SAW_TABLE[i] = Waveform::Saw.calc(delta, 1.0);
            SINE_TABLE[i] = Waveform::Sine.calc(delta, 1.0);
            SQUARE_TABLE[i] = Waveform::Square.calc(delta, 1.0);
            TRI_TABLE[i] = Waveform::Triangle.calc(delta, 1.0);
        }
    }
}