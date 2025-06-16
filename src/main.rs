/// goal: highly modular, simple enough to structure for application across various use cases

use tonomaly::synth::oscillators::{Oscillatable, Oscillator, Square, Sine, WaveformType};

use tonomaly::synth::voices::*;
use tonomaly::synth::effects::*;
use tonomaly::synth::adsr::*;

use std::thread;
use std::time::{Duration, Instant};

fn main() {

    let envelope = ADSR::new(
        0.2,
        0.2,
        0.2,
        0.2,
        44100.0,
    );
    
    let voice = Voice::new(
        WaveformType::Sine,
        EffectsChain {  },
        envelope,
        440.0,
        44100.0,
    );

    thread::spawn(move || {
        
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
