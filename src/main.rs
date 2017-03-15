extern crate audrey;
extern crate rtlsdr;
extern crate dsp;


mod audio;
mod drivers;

fn main() {
    let samples = audio::load("data/sine_440hz.wav");
    println!("Sample count: {}", samples.len());
//    drivers::test_rtlsdr();
}