use crate::oscillator::Oscillator;
use crate::song::Song;

pub struct Sequence {
    song: Song,
    sample_rate: f32,
    note: usize,
    clock: f32,
    deadline: f32,
}

impl Sequence {
    pub fn new(song: Song, sample_rate: f32) -> Self {
        let note = song.sequence.len() - 1;

        Sequence {
            song,
            sample_rate,
            note,
            clock: 0.,
            deadline: 1.,
        }
    }

    pub fn next(&mut self) -> Option<Vec<Oscillator>> {
        self.clock += 1.;

        if self.clock < self.deadline {
            return None;
        }
        self.note = (self.note + 1) % self.song.sequence.len();

        let note = &self.song.sequence[self.note];
        println!("{}", note);

        self.deadline = (self.clock + (note.length * self.sample_rate)).floor();

        // FM
        let mut oscillators: Vec<Oscillator> = self
            .song
            .oscillators
            .iter()
            .map(|ratio| Oscillator::new(self.sample_rate, note.frequency * ratio))
            .collect();

        // Envelope
        oscillators.push(Oscillator::new(note.length * self.sample_rate, 0.5));

        Some(oscillators)
    }
}
