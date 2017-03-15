/// # Audio processing
///
///  * Loading wav file
///  * Playing PCM data using sound card

use audrey;
use dsp::vectors::{Vector};

pub fn load(file_name: &str) -> Vector {
    let mut reader = audrey::open(file_name).unwrap();
    let samples: Vec<f64> = reader.samples().map(Result::unwrap).collect();
    Vector::from_reals(samples)
}
