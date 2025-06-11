use crate::synth::oscillators;
/// goal: each note pressed is its own "voice", each spawned voice should be able to be spawned with any effect, frequency, oscillator, etc. 
use crate::synth::oscillators::*;
use crate::synth::effects::*;
use crate::synth::adsr::*;

pub struct Voice {
    oscillator: Oscillator,
    effects: EffectsChain,
    envelope: ADSR,
    frequency: f32,
}

impl Voice {
    pub fn new(oscillator: Oscillator, effects: EffectsChain, envelope: ADSR, frequency: f32) -> Self {
        Self {
            oscillator,
            effects,
            envelope,
            frequency
        }
    }
}