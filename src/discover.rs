use std::{
    path::{Path, PathBuf},
    fs::read_dir,
};
use rayon::{
    iter::{ParallelIterator, ParallelBridge, ParallelExtend},
};


pub fn find_all_music(root: &Path) -> Vec<PathBuf> {
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
                .fold(Vec::new, |mut acc, path| {
                    if path.is_dir() {
                        acc.par_extend(find_all_music(&path));
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
