use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Song {
    pub oscillators: Vec<f32>,
    pub sequence: Vec<Note>,
}

#[derive(Deserialize)]
pub struct Note {
    pub frequency: f32,
    pub length: f32,
}

impl Song {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.frequency == 0. {
            Ok(())
        } else {
            write!(
                f,
                "{}░",
                " ".repeat(((self.frequency / 800.) * 80.) as usize)
            )
        }
    }
}
