extern crate audrey;
extern crate cpal;
extern crate futures;
extern crate rtlsdr;
extern crate dsp;


mod audio;
mod drivers;

fn main() {
    let samples = audio::load("data/sine_440hz.wav");
    audio::play(&samples);
//    drivers::test_rtlsdr();
}