mod song;

use std::env;
use std::thread;

use anyhow::{anyhow, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};

use song::Song;

fn main() -> Result<()> {
    let path = env::args().nth(1).ok_or(anyhow!("No song specified"))?;
    let song = Song::open(path)?;

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(anyhow!("No default output device"))?;
    let config: StreamConfig = device.default_output_config()?.into();

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            todo!("Write some samples!")
        },
        move |err| {
            eprintln!("Error: {}", err);
        },
        None,
    )?;

    stream.play()?;

    thread::park();

    Ok(())
}
