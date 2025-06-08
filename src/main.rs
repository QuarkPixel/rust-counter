use compact_str::CompactString;
use console::{Emoji, style};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    thread::{self, JoinHandle},
    time::Instant,
};

static FOLDER: Emoji<'_, '_> = Emoji("📁  ", "");
static FILE: Emoji<'_, '_> = Emoji("📄  ", "");
static COUNT: Emoji<'_, '_> = Emoji("🔢  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let source_path = Path::new(&args[1]);
    let start = Instant::now();

    let mut map = HashMap::new();

    println!(
        "{} {}Resolving text files...",
        style("[1/3]").bold().dim(),
        FOLDER
    );
    let files = find_text_files(Path::new(source_path))?;
    println!(
        "{} {}Processing files...",
        style("[2/3]").bold().dim(),
        FILE
    );

    let mut handles = Vec::new();
    const CHUNK_SIZE: usize = 10;
    let mut total_word_count: usize = 0;
    let muti_bar = MultiProgress::new();

    let process_bar = muti_bar.add(ProgressBar::new(files.len() as u64));
    process_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.bold} {bar:40.cyan/blue} {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("##-"),
    );
    process_bar.set_prefix("[processing]");

    for chunk in files.chunks(CHUNK_SIZE) {
        let handle = calc_word(chunk.to_owned(), process_bar.clone());
        handles.push(handle);
    }

    let combine_bar = muti_bar.add(ProgressBar::new(handles.len() as u64));
    combine_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.bold} {bar:40.cyan/blue} {pos}/{len} threads ({eta})")
            .unwrap()
            .progress_chars("##-"),
    );
    combine_bar.set_prefix("[combining] ");
    combine_bar.suspend(|| {});

    let mut first_call = true;

    // Wait for all threads to complete and collect results
    for handle in handles {
        let local_map = handle.join().unwrap();
        if first_call {
            first_call = false;
            combine_bar.reset_eta();
        }
        for (key, value) in local_map {
            total_word_count += value as usize;
            *map.entry(key).or_insert(0_u32) += value;
        }
        combine_bar.inc(1);
    }

    process_bar.finish_and_clear();
    combine_bar.finish_and_clear();

    println!(
        "{} {}Writing results...",
        style("[3/3]").bold().dim(),
        COUNT
    );

    let mut sorted_map = map.iter().collect::<Vec<_>>();
    sorted_map.sort_by(|a, b| (b.1).cmp(a.1));
    output_result(&sorted_map)?;

    let elapsed = start.elapsed();

    println!(
        "{} Done in {} | Map count: {} words | Speed: {:.2} words/ms",
        SPARKLE,
        HumanDuration(elapsed),
        sorted_map.len(),
        total_word_count as f64 / elapsed.as_millis() as f64
    );
    Ok(())
}

fn find_text_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
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

fn calc_word(
    chunk: Vec<PathBuf>,
    progress: ProgressBar,
) -> JoinHandle<HashMap<CompactString, u32>> {
    use compact_str::CompactString;
    thread::spawn(move || {
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
    })
}

fn output_result(result: &Vec<(&CompactString, &u32)>) -> io::Result<()> {
    let mut output_file = File::create("output.txt")?;
    const OUTPUT_LENGTH: usize = 100;

    for &(key, value) in result.iter().take(OUTPUT_LENGTH.min(result.len())) {
        writeln!(output_file, "{}: {}", key, value)?;
    }
    Ok(())
}
