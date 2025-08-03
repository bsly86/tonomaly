/// attack decay sustain release

#[derive(PartialEq, Copy, Clone)]
pub enum ADSRstate {
    Inactive,
    Attack,
    Decay,
    Sustain,
    Release
}

#[derive(PartialEq, Copy, Clone)]
pub struct ADSR {
    // user configurable
    pub attack: f32, // time it takes to fade in
    pub decay: f32, // time it takes to fall to sustain level
    pub sustain: f32, // volume of sustain
    pub release: f32, // time it takes to fade to nothing
    pub sample_rate: f32,

    pub state: ADSRstate,
    pub current_amplitude: f32,
    pub release_start_amplitude: f32,
    pub time_in_state: f32,
}

impl ADSR {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32, sample_rate: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            sample_rate,

            state: ADSRstate::Inactive,
            current_amplitude: 0.0,
            time_in_state: 0.0,
            release_start_amplitude: 0.0,
        }
    }

    pub fn note_on(&mut self) {
        // println!("ADSR: note on")
        self.state = ADSRstate::Attack;
        self.time_in_state = 0.0;
    }

    pub fn note_off(&mut self) {
        // println!("ADSR: note off")
        self.release_start_amplitude = self.current_amplitude;
        self.state = ADSRstate::Release;
        self.time_in_state = 0.0;
    }

    pub fn next_sample(&mut self) -> f32 {
        let delta = 1.0 / self.sample_rate;
        self.time_in_state += delta;

        match self.state {
            ADSRstate::Inactive => {
                self.current_amplitude = 0.0;
            }

            ADSRstate::Attack => {
                let attack_progress = self.time_in_state / self.attack;
                // Use smooth curve instead of linear ramp
                self.current_amplitude = 1.0 - (-3.0 * attack_progress).exp();

                if self.time_in_state >= self.attack {
                    self.state = ADSRstate::Decay;
                    self.time_in_state = 0.0;
                }
            }

            ADSRstate::Decay => {
                if self.time_in_state >=  self.decay {
                    self.current_amplitude = self.sustain;
                    self.state = ADSRstate::Sustain;
                } else {
                    let decay_progress = self.time_in_state / self.decay;
                    self.current_amplitude = self.sustain + (1.0 - self.sustain) * (-3.0 * decay_progress).exp();
                }
            }

            ADSRstate::Sustain => {
                self.current_amplitude = self.sustain;
            }

            ADSRstate::Release => {
                if self.time_in_state >= self.release {
                    self.current_amplitude = 0.0;
                    self.state = ADSRstate::Inactive;
                } else {
                    let release_progress = self.time_in_state / self.release;
                    self.current_amplitude = self.release_start_amplitude * (-3.0 * release_progress).exp();
                }
            }
        }
        let mut amp = self.current_amplitude;
        amp = amp.clamp(0.0, 1.0);
        amp
    }

    pub fn current_state(self) -> ADSRstate {
        self.state
    }

    pub fn is_finished(&self) -> bool {
        self.state == ADSRstate::Inactive
    }

}