use crate::oscillator::Oscillator;
use crate::sequence::Sequence;
use crate::song::Song;

pub struct Synth {
    channels: usize,
    sequence: Sequence,
    oscillators: Vec<Oscillator>,
}

impl Synth {
    pub fn new(song: Song, sample_rate: f32, channels: usize) -> Self {
        let sequence = Sequence::new(song, sample_rate);
        let oscillators = Vec::new();

        Synth {
            channels,
            sequence,
            oscillators,
        }
    }

    pub fn play(&mut self, data: &mut [f32]) {
        for frame in data.chunks_mut(self.channels) {
            let value = self.next();
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }

    fn next(&mut self) -> f32 {
        if let Some(oscillators) = self.sequence.next() {
            self.oscillators = oscillators;
        }

        self.oscillators
            .iter_mut()
            .map(Oscillator::next)
            .reduce(|oscillator, value| value * oscillator)
            .unwrap_or(0.)
    }
}
