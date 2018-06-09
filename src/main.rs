extern crate sample;
extern crate hound;

use std::error::Error;
use sample::{Sample, Signal, signal};
use hound::{WavSpec, WavWriter};

const SAMPLE_RATE: f64 = 44100.0;

fn main() -> Result<(), Box<Error>> {
    // This one works
    let mut sine1 = signal::rate(SAMPLE_RATE)
        .const_hz(440.0)
        .sine();

    // This one doesn't 
    let mut sine2 = signal::rate(SAMPLE_RATE)
        .hz(440.0, signal::gen(|| [440.0]))
        .sine();

    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create("sine_problems.wav", spec)?;

    for f in sine1.by_ref().take(44100).chain(sine2.by_ref().take(44100)) {
        for s in f.iter() {
            writer.write_sample(s.to_sample::<i16>())?;
        }
    }

    Ok(())
}
