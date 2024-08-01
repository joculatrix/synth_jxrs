use super::*;

pub enum FilterType {
    HP,
    LP,
}

pub struct Filter {
    r#type: FilterType,
    bypass: bool,
    cutoff: f64,
    resonance: f64,
    buf: [f64; 4],
}

impl Filter {
    pub fn new(r#type: FilterType, cutoff: f64, resonance: f64) -> Filter {
        unsafe {
            Filter {
                r#type,
                bypass: false,
                cutoff: 2.0 * PI * cutoff / SAMPLE_RATE,
                resonance,
                buf: [0.0; 4],
            }
        }
    }

    pub fn calc(&mut self, input: f64) -> f64 {
        match self.r#type {
            FilterType::HP => todo!(),
            FilterType::LP => todo!(),
        }
    }
}