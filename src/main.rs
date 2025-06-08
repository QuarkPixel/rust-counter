mod counter;
mod emoji;
mod progresser;
pub mod utils;

use crate::{
    counter::Counter,
    progresser::{Bar, print_step},
};
use indicatif::HumanDuration;
use std::{env, error::Error, num::NonZeroUsize, path::Path, thread, time::Instant};

pub const CHUNK_SIZE: usize = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let source_path = Path::new(&args[1]);
    let start = Instant::now();

    print_step(1, emoji::FOLDER, "Resolving text files...");
    // let files = find_text_files(Path::new(source_path))?;
    let mut counter = Counter::new(source_path).expect("Failed to read the path.");
    print_step(2, emoji::FILE, "Processing files...");

    let bars = Bar::new();
    let process_bar = bars.generate(Some(counter.files.len() as _), "processing");
    let combine_bar = bars.generate(None, "combining");

    let word_count = thread::scope(|s| -> utils::Dict {
        let map = counter.process_and_collect(s, &process_bar, &combine_bar);

        process_bar.finish_and_clear();
        combine_bar.finish_and_clear();
        map
    });

    print_step(3, emoji::COUNT, "Writing results...");

    let mut sorted_word_count = word_count.iter().collect::<Vec<_>>();
    sorted_word_count.sort_by(|a, b| (b.1).cmp(a.1));
    utils::output_result(&sorted_word_count, NonZeroUsize::new(10));

    let elapsed = start.elapsed();

    println!(
        "{} Done in {} | Map count: {} words | Speed: {:.2} words/ms",
        emoji::SPARKLE,
        HumanDuration(elapsed),
        sorted_word_count.len(),
        counter.word_count as f64 / elapsed.as_millis() as f64
    );
    Ok(())
}
