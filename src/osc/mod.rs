use std::{array, sync::LazyLock};

use wave::Waveform;

pub mod oscillator;
pub mod wave;

/// Number of samples stored in the reference tables for each waveform.
const TABLE_LENGTH: usize = 1024;

static SAW_TABLE: LazyLock<[f64; TABLE_LENGTH]> = LazyLock::new(|| {
    array::from_fn(|i|
        Waveform::Saw.calc(i as f64 / TABLE_LENGTH as f64, 1.0)
    )
});
static SINE_TABLE: LazyLock<[f64; TABLE_LENGTH]> = LazyLock::new(|| {
    array::from_fn(|i|
        Waveform::Sine.calc(i as f64 / TABLE_LENGTH as f64, 1.0)
    )
});
static SQUARE_TABLE: LazyLock<[f64; TABLE_LENGTH]> = LazyLock::new(|| {
    array::from_fn(|i|
        Waveform::Square.calc(i as f64 / TABLE_LENGTH as f64, 1.0)
    )
});
static TRI_TABLE: LazyLock<[f64; TABLE_LENGTH]> = LazyLock::new(|| {
    array::from_fn(|i|
        Waveform::Triangle.calc(i as f64 / TABLE_LENGTH as f64, 1.0)
    )
});

/// Call the initialization for [`SAW_TABLE`], [`SINE_TABLE`], [`SQUARE_TABLE`], and
/// [`TRI_TABLE`] statics, so that their pre-generated values can be referenced at runtime
/// rather than doing constant calculations. See: [`Oscillator::calc()`].
/// 
/// [`Oscillator::calc()`]: oscillator::Oscillator::calc()
pub fn init_tables() {
    let _ = &*SAW_TABLE;
    let _ = &*SINE_TABLE;
    let _ = &*SQUARE_TABLE;
    let _ = &*TRI_TABLE;
}