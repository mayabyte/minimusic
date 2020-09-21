use std::{
    path::{Path, PathBuf},
    fs::read_dir,
    process::Command,
};
use rayon::iter::{ParallelIterator, ParallelBridge, ParallelExtend};


pub fn find_music(root: &Path) -> Vec<PathBuf> {
    match read_dir(root) {
        Ok(files) => {
            files.par_bridge()
                .filter_map(|dir| {
                    match dir {
                        Ok(dir_entry) => Some(dir_entry.path()),
                        Err(e) => {
                            eprintln!("An error occurred: {}", e);
                            None
                        }
                    }
                })
                .filter(|path| is_audio(path))
                .fold(Vec::new, |mut acc, path| {
                    if path.is_dir() {
                        acc.par_extend(find_music(&path));
                    } else {
                        acc.push(path);
                    };
                    acc
                })
                .reduce(Vec::new, |mut acc, sublist| { acc.extend(sublist); acc})
        },
        Err(e) => {
            eprintln!("Couldn't read {}. Reason: {}", root.to_string_lossy(), e);
            Vec::new()
        }
    }
}

fn is_audio(file: &Path) -> bool {
    let output = Command::new("mediainfo")
        .arg(file)
        .output()
        .expect("Couldn't call mediainfo. Is it installed?");
    let output = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
    output.lines().any(|line| line.trim().eq_ignore_ascii_case("audio"))
}