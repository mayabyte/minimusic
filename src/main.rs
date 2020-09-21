mod discover;
mod transcode;


use crate::discover::find_music;
use crate::transcode::transcode_file;
use std::{
    path::PathBuf,
    fs::create_dir,
    process::{Command, exit},
};
use regex::Regex;
use structopt::StructOpt;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle, ParallelProgressIterator};


fn main() {
    let options = get_options();

    // Find all files in the given directory
    println!("Locating music files. This may take a moment...");
    let files = find_music(&options.input);

    // Set up the progress bar
    let progress_bar = ProgressBar::new(files.len() as u64)
        .with_style(ProgressStyle::default_bar().template(
            "[{pos}/{len} ({percent}%)] {wide_bar} [ETA: {eta}, {per_sec}]"
        ));

    files.par_iter()
        .progress_with(progress_bar)
        .for_each(|file| transcode_file(&options, file));
}

fn get_options() -> Options {
    let mut options = Options::from_args();

    // Check that the supplied codec is supported by the local installation of ffmpeg
    options.output_codec.make_ascii_lowercase();
    let output = Command::new("ffmpeg")
        .arg("-codecs")
        .output()
        .expect("Couldn't call ffmpeg. Is it installed?");
    let output = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
    let codec_re = Regex::new(r".EA...\s+[[:word:]]+").unwrap();
    let supported_codecs: Vec<&str> = output.lines()
        .filter(|line| codec_re.is_match(line))
        .filter_map(|line| line.trim().split_whitespace().nth(1))
        .collect();
    if !supported_codecs.contains(&options.output_codec.as_str()) {
        eprintln!("Error: codec \"{}\" not recognized by ffmpeg. Run \"ffmpeg -codecs\" to see the list of supported codecs.",
            options.output_codec);
        exit(1);
    }

    // Make sure the extension doesn't begin with a dot
    options.extension = options.extension.strip_prefix(".")
        .map(|x| x.to_owned())
        .unwrap_or(options.extension);

    // Ensure the output directory exists
    if !options.output.exists() {
        create_dir(&options.output).expect("Couldn't create output directory.");
    };

    options
}


#[derive(StructOpt)]
#[structopt(
    name = "minimusic",
    about = "A tool to bulk-transcode music files for storage on mobile devices or servers."
)]
pub struct Options {
    #[structopt(
        short, long, parse(from_os_str),
        help = "The root folder in which to recursively search for music."
    )]
    pub input: PathBuf,

    #[structopt(
        short, long, parse(from_os_str),
        help = "The new root folder to write transcoded music to. Preserves the same internal folder \
                structure as the input folder."
    )]
    pub output: PathBuf,

    #[structopt(
        short = "c",
        long = "codec",
        help = "The codec to convert all music into."
    )]
    pub output_codec: String,

    #[structopt(
        short, long,
        help = "The file extension to be used for transcoded music files."
    )]
    pub extension: String
}
