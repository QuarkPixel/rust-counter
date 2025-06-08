use std::{
    collections::HashMap,
    fs, io,
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use compact_str::CompactString;
use indicatif::ProgressBar;

pub type Dict = HashMap<CompactString, u32>;

pub fn find_text_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_files = find_text_files(&path)?;
            files.extend(sub_files);
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
            files.push(path);
        }
    }

    Ok(files)
}

pub fn calc_word(chunk: &[PathBuf], progress: &ProgressBar) -> Dict {
    let mut local_map = HashMap::new();
    for path in chunk.iter() {
        if let Ok(text) = fs::read_to_string(path) {
            text.split_whitespace().for_each(|word| {
                let word = CompactString::from_str_to_lowercase(
                    word.trim_matches(|c: char| c.is_ascii_punctuation()),
                );
                if !word.is_empty() {
                    *local_map.entry(word).or_insert(0) += 1;
                }
            });
        }
        progress.inc(1);
    }
    local_map
}

pub fn output_result(result: &Vec<(&CompactString, &u32)>, output_length: Option<NonZeroUsize>) {
    let length = output_length
        .map_or_else(|| 0, |num| num.get())
        .min(result.len());

    for (i, &(key, value)) in result.iter().take(length).enumerate() {
        println!("{:>4}{}:{}", format!("{}.", i + 1), key, value);
    }
}
