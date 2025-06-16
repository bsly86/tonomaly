use crate::synth::oscillators;
/// goal: each note pressed is its own "voice", each spawned voice should be able to be spawned with any effect, frequency, oscillator, etc. 
use crate::synth::oscillators::*;
use crate::synth::effects::*;
use crate::synth::adsr::*;

pub struct Voice {
    waveform: WaveformType,
    osci: Oscillator,
    effects: EffectsChain,
    envelope: ADSR,
    frequency: f32,
    sample_rate: f32,
    is_active: bool,
}

impl Voice {
    
    pub fn new(waveform: WaveformType, effects: EffectsChain, mut envelope: ADSR, frequency: f32, sample_rate: f32) -> Self {
        
        envelope.note_on();

        let mut osci = match waveform {
            WaveformType::Sine => Oscillator::Sine(Sine::new(frequency, sample_rate)),
            WaveformType::Square => Oscillator::Square(Square::new(frequency, sample_rate, 0.5)),
            WaveformType::Sawtooth => todo!(),
            WaveformType::Triangle => todo!(),
        };

        
        Self {
            waveform,
            osci,
            effects,
            envelope,
            frequency,
            sample_rate,
            is_active: true,
        }
        
    }

    pub fn next_sample(&mut self) -> f32 {
        if !self.is_active && self.envelope.is_finished() {
            return 0.0;
        }

        let mut sample = self.osci.next();
        sample

    }

}