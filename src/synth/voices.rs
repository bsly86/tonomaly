/// goal: each note pressed is its own "voice", each spawned voice should be able to be spawned with any effect, frequency, oscillator, etc. 
use crate::synth::oscillators;
use crate::synth::oscillators::*;
use crate::synth::effects::*;
use crate::synth::adsr::*;
use crate::synth::conf::*;

use std::collections::HashMap;

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
        // Get raw oscillator sample
        let sample = self.osci.next();
        
        // Apply ADSR envelope
        let envelope_amplitude = self.envelope.next_sample();
        let sample = sample * envelope_amplitude;
        
        // Apply effects chain
        let sample = self.effects.process(sample);
        
        // Update voice state
        if self.envelope.is_finished() {
            self.is_active = false;
        }
        
        sample
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active; 
        self.envelope.note_off();
    }

}

pub struct VoiceManager {
    voices: HashMap<u8, Voice>,
    config: SynthConfig,
}

impl VoiceManager {
    
    pub fn new(sample_rate: f32) -> Self {
        let config = SynthConfig::new(440.0, sample_rate);
        Self {
            voices: HashMap::new(),
            config,
        }
    }

    pub fn spawn(&mut self, note: u8, freq: f32, waveform: WaveformType, effects: EffectsChain, envelope: ADSR) {
        let voice = Voice::new(waveform, effects, envelope, freq, self.config.sample_rate);
        self.voices.insert(note, voice);
    }

    pub fn destroy(&mut self, note: u8) {
        if let Some(mut voice) = self.voices.remove(&note) {
            voice.set_active(false);
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut mixed_sample = 0.0;
        let mut finished_notes = Vec::new();
        let voice_count = self.voices.len() as f32;

        // Process all voices and collect finished ones
        for (note, voice) in &mut self.voices {
            let sample = voice.next_sample();
            mixed_sample += sample;
            
            if !voice.is_active {
                finished_notes.push(*note);
            }
        }

        // Remove finished voices
        for note in finished_notes {
            self.voices.remove(&note);
        }

        // Average the samples to prevent clipping
        if voice_count > 0.0 {
            mixed_sample / voice_count
        } else {
            0.0
        }
    }
}