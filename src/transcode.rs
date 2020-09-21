use crate::Options;
use std::{
    path::{Path, PathBuf},
    process::{Command, Output, Stdio}
};


pub fn transcode_file(options: &Options, input_filename: &Path) {
    let output_filename = produce_output_filename(options, input_filename);
    transcode(input_filename, &output_filename, &options.output_codec);
}

fn produce_output_filename(options: &Options, input_filename: &Path) -> PathBuf {
    options.output
        .join(input_filename.strip_prefix(&options.input)
            .expect(&format!("Somehow found input file {} that isn't a subdirectory of the input.",
                input_filename.display())))
        .with_extension(&options.extension)
}

fn transcode(file: &Path, dest: &Path, codec: &str) {
    match Command::new("ffmpeg")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-i")
        .arg(file)
        .arg("-c:a")
        .arg(codec)
        .arg(dest)
        .output()
    {
        Ok(Output{ status, .. }) if !status.success() => {
            // eprintln!("Transcoding failed for file {} with non-zero status code",
            //     file.display());
        },
        Err(e) => {
            // eprintln!("Transcoding failed for file {}. Reason: {}",
            //     file.display(),
            //     e);
        },
        _ => { /* success */ }
    };
}
