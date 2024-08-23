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

/// These tests ensure the first samples of [`SAW_TABLE`], [`SINE_TABLE`], and [`TRI_TABLE`]
/// occur at zero-crossings (amplitude 0.0), making it more feasible to minimize audible popping
/// by starting notes at the first sample. See [`Oscillator::note_on()`].
/// 
/// These tests exclude the square wave because it only has values of `-1.0` and `1.0`.
/// 
/// [`Oscillator::note_on()`]:  oscillator::Oscillator::note_on()
#[cfg(test)]
mod zero_crossing_tests {
    use super::*;

    #[test]
    fn first_saw_sample_is_zero() {
        init_tables();
        assert_eq!(SAW_TABLE[0], 0.0)
    }

    #[test]
    fn first_sine_sample_is_zero() {
        init_tables();
        assert_eq!(SINE_TABLE[0], 0.0)
    }

    #[test]
    fn first_triangle_sample_is_zero() {
        init_tables();
        assert_eq!(TRI_TABLE[0], 0.0)
    }
}