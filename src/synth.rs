use crate::song::Song;

pub struct Synth {
    channels: usize,
}

impl Synth {
    pub fn new(song: Song, sample_rate: f32, channels: usize) -> Self {
        Synth { channels }
    }

    pub fn play(&mut self, data: &mut [f32]) {
        for frame in data.chunks_mut(self.channels) {
            for sample in frame.iter_mut() {
                *sample = 0.;
            }
        }
    }
}
