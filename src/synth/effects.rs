/// goal: basic effects such as reverb, chorus, pwm, saturation, low pass

#[derive(PartialEq, Copy, Clone)]
pub struct LowPassFilter {
    
    pub cutoff_frequency: f32,
    pub sample_rate: f32,
    previous_output: f32,
    coefficient: f32,

}

impl LowPassFilter {

    pub fn new(cutoff_frequency: f32, sample_rate: f32) -> Self {
        
        let mut filter = Self {
            cutoff_frequency: cutoff_frequency.clamp(0.0, sample_rate / 2.0),
            sample_rate,
            previous_output: 0.0,
            coefficient: 0.0,

        };
        
        filter.update_coefficient();
        filter

    }

    pub fn set_cutoff_frequency(&mut self, cutoff_frequency: f32) {
        
        self.cutoff_frequency = cutoff_frequency.clamp(0.0, self.sample_rate / 2.0);
        self.update_coefficient();

    }

    pub fn get_cutoff_frequency(&self) -> f32 {
        
        self.cutoff_frequency

    }

    fn update_coefficient(&mut self) {

        let rc = 1.0 / (2.0 * std::f32::consts::PI * self.cutoff_frequency);
        let dt = 1.0 / self.sample_rate;
        self.coefficient = dt / (rc + dt);

    }

    /// Process a single sample through the low pass filter
    pub fn process(&mut self, input: f32) -> f32 {
        
        let output = self.coefficient * input + (1.0 - self.coefficient) * self.previous_output;
        self.previous_output = output;
        output

    }

    pub fn reset(&mut self) {
        
        self.previous_output = 0.0;

    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct EffectsChain {

    pub low_pass_filter: Option<LowPassFilter>,

}

impl EffectsChain {

    pub fn new() -> Self {
        
        Self {
            
            low_pass_filter: None,

        }
    }

    pub fn add_low_pass_filter(&mut self, cutoff_frequency: f32, sample_rate: f32) {
        self.low_pass_filter = Some(LowPassFilter::new(cutoff_frequency, sample_rate));
    }

    /// Remove the low pass filter from the effects chain
    pub fn remove_low_pass_filter(&mut self) {
        self.low_pass_filter = None;
    }

    /// Process a sample through all active effects in the chain
    pub fn process(&mut self, input: f32) -> f32 {
        let mut output = input;

        // Apply low pass filter if enabled
        if let Some(filter) = &mut self.low_pass_filter {
            output = filter.process(output);
        }

        output
    }

    /// Reset all effects in the chain
    pub fn reset(&mut self) {
        if let Some(filter) = &mut self.low_pass_filter {
            filter.reset();
        }
    }
}

impl Default for EffectsChain {
    fn default() -> Self {
        Self::new()
    }
}

