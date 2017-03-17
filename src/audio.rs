#![allow(dead_code)]
/// # Audio processing
///
///  * Loading wav file
///  * Playing PCM data using sound card

use audrey;
use dsp::signals::{Signal};

use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use std;
use std::f64::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use cpal;


/// Load wav file
pub fn load(file_name: &str) -> Signal {
    let mut reader = audrey::open(file_name).unwrap();
    let samples: Vec<f64> = reader.samples().map(Result::unwrap).collect();
    Signal::from_reals(samples, 44100)
}

/// Play PCM data using system audio
struct MyExecutor;

impl Executor for MyExecutor {
    fn execute(&self, r: Run) {
        r.run();
    }
}

pub fn play(pcm: &Signal) {
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list().unwrap()
        .find(|x| {x.samples_rate.0 == 44100 && x.data_type == cpal::SampleFormat::F32})
        .expect("Failed to get 44100Hz frequency");

    println!("Sample rate: {:?}", format);

    let event_loop = cpal::EventLoop::new();
    let executor = Arc::new(MyExecutor);

    // The voice can be used to control the play/pause of the output,
    // while the samples_stream can be used to register a callback
    // that will be called whenever the backend is ready to get data.
    // See the documentation of futures-rs for more info about how to use streams.
    let (mut voice, stream) = cpal::Voice::new(&endpoint, &format, &event_loop).expect("Failed to create a voice");


    // Produce a sinusoid of maximum amplitude.
    let samples_rate = format.samples_rate.0 as f32;
    let mut data_source = (0u64..).map(move |t| t as f32 * 440.0 * 2.0 * (PI as f32) / samples_rate)     // 440 Hz
        .map(move |t| t.sin());

    voice.play();
    task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {
        match buffer {
            cpal::UnknownTypeBuffer::U16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    let value = ((value * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() { *out = value; }
                }
            },

            cpal::UnknownTypeBuffer::I16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    let value = (value * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() { *out = value; }
                }
            },

            cpal::UnknownTypeBuffer::F32(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    for out in sample.iter_mut() { *out = value; }
                }
            },
        };

        Ok(())
    })).execute(executor);

    thread::spawn(move || {
        voice.play();
        thread::sleep(Duration::from_millis(3000));
        voice.pause();
    });

    event_loop.run();
}