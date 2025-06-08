mod config;
mod counter;
mod emoji;
mod progresser;
pub mod utils;

use crate::{
    config::Config,
    counter::Counter,
    progresser::{Bar, print_step},
};
use indicatif::HumanDuration;
use std::{env, error::Error, num::NonZeroUsize, process, thread, time::Instant};

pub const CHUNK_SIZE: usize = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let start = Instant::now();

    let AnalyzeInfo {
        map_count,
        total_count,
    } = count(config);

    let elapsed = start.elapsed();

    println!(
        "{} Done in {} | Map count: {} words | Speed: {:.2} words/ms",
        emoji::SPARKLE,
        HumanDuration(elapsed),
        map_count,
        total_count as f64 / elapsed.as_millis() as f64
    );
    Ok(())
}

struct AnalyzeInfo {
    map_count: usize,
    total_count: usize,
}

fn count(config: Config) -> AnalyzeInfo {
    print_step(1, emoji::FOLDER, "Resolving text files...");
    let mut counter = Counter::new(config.source_path).expect("Unable to read from the path.");
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
    utils::output_result(&sorted_word_count, NonZeroUsize::new(config.output_count));

    AnalyzeInfo {
        map_count: word_count.len(),
        total_count: counter.word_count,
    }
}
