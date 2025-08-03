/// goal: highly modular, simple enough to structure for application across various use cases

use tonomaly::synth::oscillators::{WaveformType};

use tonomaly::synth::voices::*;
use tonomaly::synth::effects::*;
use tonomaly::synth::adsr::*;

use tonomaly::synth::output::Audio;

use std::thread;
use std::time::{Duration};

fn main() {

    // config for all voices 

    let waveform = WaveformType::Sine;

    let fx = EffectsChain { low_pass_filter: Some(LowPassFilter::new(4000.0, 44100.0)) };

    let envelope = ADSR::new(
        0.1,
        0.2,
        0.8,
        1.0,
        44100.0,
    );

    let mut voice_manager = VoiceManager::new(44100.0);
    voice_manager.spawn(1, 880.0, waveform, fx, envelope);
    voice_manager.spawn(2, 440.0, waveform, fx, envelope);

    let aud = Audio::new(move || {
        voice_manager.next_sample()
    });
    
    aud.set_volume(0.2); 
    
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
