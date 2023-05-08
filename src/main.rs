mod ffmpeg;

#[macro_use]
extern crate tracing;

use anyhow::Context;
use clap::Parser;
use ffmpeg_next::{codec, format, media};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    // #[arg(short, long)]
    // output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    ffmpeg::init_ffmpeg().context("Initializing ffmpeg and ffmpeg logger")?;

    let args = Args::parse();

    info!("Inputting from: {:?}", &args.input);
    // info!("Outputting to: {:?}", &args.output);

    // http://dranger.com/ffmpeg/tutorial01.html

    Ok(())
}
