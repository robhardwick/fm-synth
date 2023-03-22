mod song;

use std::env;

use anyhow::{anyhow, Result};

use song::Song;

fn main() -> Result<()> {
    let path = env::args().nth(1).ok_or(anyhow!("No song specified"))?;
    let song = Song::open(path)?;

    Ok(())
}
