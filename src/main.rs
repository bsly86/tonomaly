/// goal: highly modular, simple enough to structure for application across various use cases

use tonomaly::synth::oscillators::{Oscillatable, Oscillator, Square, Sine};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut wave = Square::new(440.0, 44100.0, 0.5);
    let sample_interval = Duration::from_secs_f32(1.0 / wave.get_sample_rate());

    thread::spawn(move || {
        loop {
            let start = Instant::now();
            
            let sample = wave.next();
            println!("{sample}");

            let elapsed = start.elapsed();
            if elapsed < sample_interval {
                thread::sleep(sample_interval - elapsed);
            }
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
