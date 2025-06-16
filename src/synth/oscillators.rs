/// Essential keywords:
/// frequency: the amount of times something happens within a set period; in this context, frequency is the number of cycles (0 -> 1 -> -1 -> 0) / second
/// sample rate: the number of samples taken / second, a sample is a "snapshot" of the audio
/// phase: phase represents the progress through the cycle, a 0.0 representing the start of the cycle, with 1.0 representing the end. for a sine wave, a phase of 0.25 means the sample's value is at 1
/// phase increment: a calculation of how much to progress the phase each sample
/// duty: the shape inside a single cycle, how much time is spent high (1) vs low (0). 0.5 would be high for half the cycle, low for half the cycle

pub trait Oscillatable {
    fn set_frequency(&mut self, frequency: f32);
    fn get_frequency(&self) -> f32;
    fn get_sample_rate(&self) -> f32;
    fn set_phase_increment(&mut self);
    fn next(&mut self) -> f32;
}

pub enum Oscillator {
    Sine(Sine),
    Square(Square),
    Sawtooth(Sawtooth),
    Triangle(Triangle),
}

pub enum WaveformType {
    Sine,
    Square,
    Sawtooth,
    Triangle
}

impl Oscillator {
    pub fn next(&mut self) -> f32 {
        match self {
            Oscillator::Sine(osc) => osc.next(),
            Oscillator::Square(osc) => osc.next(),
            Oscillator::Sawtooth(_) => todo!(),
            Oscillator::Triangle(_) => todo!(),
        }
    }
}

/// Sine ///
pub struct Sine {
    pub frequency: f32,
    pub sample_rate: f32,
    pub phase: f32,
    pub phase_increment: f32,
}

impl Sine {
    // Initialization of sine wave characteristics
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        let mut osc = Self {
            frequency,
            sample_rate,
            phase: 0.0,
            phase_increment: 0.0,
        };
        osc.set_phase_increment();
        osc
    }
}

impl Oscillatable for Sine {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.set_phase_increment();
    }

    fn get_frequency(&self) -> f32 {
        self.frequency
    }

    fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }

    fn set_phase_increment(&mut self) {
        self.phase_increment = self.frequency / self.sample_rate;
    }

    fn next(&mut self) -> f32 {
        let sample = (2.0 * std::f32::consts::PI * self.phase).sin(); // one cycle of a sine wave is 2pi radians

        self.phase += self.phase_increment; // how far into the cycle are we?
        if self.phase >= 1.0 {
            self.phase = 0.0; // if phase >= 1, bring it back down to 0
        }
        sample // return sample
    }
}

/// Square ///
pub struct Square {
    pub frequency: f32,
    pub sample_rate: f32,
    pub phase: f32,
    pub phase_increment: f32,
    pub duty: f32,
}

impl Square {
    pub fn new(frequency: f32, sample_rate: f32, duty: f32,) -> Self {
        let mut osc = Self {
            frequency,
            sample_rate,
            phase: 0.0,
            phase_increment: 0.0,
            duty
        };
        osc.set_phase_increment();
        osc
    }

    pub fn set_duty(&mut self, duty: f32) {
        self.duty = duty;
    }

}

impl Oscillatable for Square {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.set_phase_increment();
    }

    fn get_frequency(&self) -> f32 {
        self.frequency
    }

    fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }

    fn set_phase_increment(&mut self) {
        self.phase_increment = self.frequency / self.sample_rate;
    }

    fn next(&mut self) -> f32 {
        let sample = if self.phase < self.duty {1.0} else {-1.0}; // use 0.5 as an example for duty. if phase is less than 0.5, then we're in the high state. if it's above 0.5, we go into the low state. see example keywords for a definition of duty
        
        self.phase += self.phase_increment; // how far into the cycle are we?
        if self.phase >= 1.0 {
            self.phase = 0.0; // if phase >= 1, bring it back down to 0
        }
        sample // return sample
    }
}

/// Saw ///
pub struct Sawtooth {}

impl Sawtooth {
    // todo
}

/// Triangle ///
pub struct Triangle {}

impl Triangle {
    // todo
}

