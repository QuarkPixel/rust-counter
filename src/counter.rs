use indicatif::ProgressBar;

use crate::{
    CHUNK_SIZE,
    utils::{self, Dict},
};
use std::{
    collections::HashMap,
    io,
    num::NonZeroU64,
    path::{Path, PathBuf},
    thread::Scope,
};

pub struct Counter {
    pub files: Vec<PathBuf>,
    pub word_count: usize,
}

impl Counter {
    pub fn new(dir: &Path) -> io::Result<Self> {
        Ok(Self {
            files: utils::find_text_files(dir)?,
            word_count: 0,
        })
    }

    pub fn process_and_collect<'a>(
        &'a mut self,
        s: &'a Scope<'a, '_>,
        process_bar: &'a ProgressBar,
        combine_bar: &'a ProgressBar,
    ) -> Dict {
        let mut handles = Vec::new();

        // Process files
        for chunk in self.files.chunks(CHUNK_SIZE) {
            let handle = s.spawn(move || utils::calc_word(chunk, process_bar));
            handles.push(handle);
        }

        // Collect result
        let mut length = NonZeroU64::new(handles.len() as _);
        let mut total_map: Dict = HashMap::new();

        for handle in handles {
            let thread_map = handle.join().unwrap();
            if let Some(length) = length.take() {
                combine_bar.reset();
                combine_bar.set_length(length.get());
            }
            for (key, value) in thread_map {
                self.word_count += value as usize;
                *total_map.entry(key).or_insert(0) += value;
            }
            combine_bar.inc(1);
        }

        total_map
    }
}
