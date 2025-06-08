mod counter;
mod emoji;
mod progresser;
pub mod utils;

use console::style;
use indicatif::HumanDuration;
use std::{env, error::Error, num::NonZeroUsize, path::Path, time::Instant};
use crate::{counter::Counter, progresser::Bar};

pub const CHUNK_SIZE: usize = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let source_path = Path::new(&args[1]);
    let start = Instant::now();

    println!(
        "{} {}Resolving text files...",
        style("[1/3]").bold().dim(),
        emoji::FOLDER
    );
    // let files = find_text_files(Path::new(source_path))?;
    let mut counter = Counter::new(source_path).expect("Failed to read the path.");
    println!(
        "{} {}Processing files...",
        style("[2/3]").bold().dim(),
        emoji::FILE
    );

    let bars = Bar::new();

    let process_bar = bars.generate(counter.files.len() as _, "processing");

    counter.process(&process_bar);

    let combine_bar = bars.generate(counter.handles.len() as _, "combining");
    combine_bar.suspend(|| {});

    let map = counter.collect(&combine_bar, || combine_bar.reset_eta());

    process_bar.finish_and_clear();
    combine_bar.finish_and_clear();

    println!(
        "{} {}Writing results...",
        style("[3/3]").bold().dim(),
        emoji::COUNT
    );

    let mut sorted_map = map.iter().collect::<Vec<_>>();
    sorted_map.sort_by(|a, b| (b.1).cmp(a.1));
    utils::output_result(&sorted_map, NonZeroUsize::new(10))?;

    let elapsed = start.elapsed();

    println!(
        "{} Done in {} | Map count: {} words | Speed: {:.2} words/ms",
        emoji::SPARKLE,
        HumanDuration(elapsed),
        sorted_map.len(),
        counter.word_count as f64 / elapsed.as_millis() as f64
    );
    Ok(())
}
