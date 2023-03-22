use std::f32::consts::TAU;

pub struct Oscillator {
    sample_rate: f32,
    frequency: f32,
    clock: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32, frequency: f32) -> Self {
        Oscillator {
            sample_rate,
            frequency,
            clock: 0.,
        }
    }

    pub fn next(&mut self) -> f32 {
        self.clock += 1.;

        (self.clock * self.frequency * TAU / self.sample_rate).sin()
    }
}
