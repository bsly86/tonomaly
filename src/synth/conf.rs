// global config
pub struct SynthConfig {
    pub base_freq: f32,
    pub sample_rate: f32,
}

impl Default for SynthConfig {
    fn default() -> Self {
        Self {
            base_freq: 440.0,
            sample_rate: 44100.0,
        }
    }
}

impl SynthConfig {
    pub fn new(base_freq: f32, sample_rate: f32) -> Self {
        Self {
            base_freq,
            sample_rate
        }
    }
}