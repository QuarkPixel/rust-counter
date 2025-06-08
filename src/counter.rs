use indicatif::ProgressBar;

use crate::{
    CHUNK_SIZE,
    utils::{self, Dict},
};
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    thread::{self, JoinHandle},
};

pub struct Counter {
    pub files: Vec<PathBuf>,
    pub handles: Vec<JoinHandle<Dict>>,
    pub word_count: usize,
}

impl Counter {
    pub fn new(dir: &Path) -> io::Result<Self> {
        Ok(Self {
            files: utils::find_text_files(dir)?,
            handles: Vec::new(),
            word_count: 0,
        })
    }

    pub fn process(&mut self, progress: &ProgressBar) {
        for chunk in self.files.chunks(CHUNK_SIZE) {
            let chunk = chunk.to_owned();
            let progress = progress.clone();

            let handle = thread::spawn(move || utils::calc_word(chunk, progress));
            self.handles.push(handle);
        }
    }

    pub fn collect<F>(&mut self, progress: &ProgressBar, init: F) -> Dict
    where
        F: FnOnce(),
    {
        let mut init = Some(init);
        let mut total_map: Dict = HashMap::new();

        for handle in self.handles.drain(..) {
            let thread_map = handle.join().unwrap();
            if let Some(f) = init.take() {
                f()
            }
            for (key, value) in thread_map {
                self.word_count += value as usize;
                *total_map.entry(key).or_insert(0) += value;
            }
            progress.inc(1);
        }

        total_map
    }
}
