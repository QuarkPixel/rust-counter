use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};

pub struct Bar(MultiProgress);

impl Bar {
    pub fn new() -> Self {
        Self(MultiProgress::new())
    }

    pub fn generate(&self, len: Option<u64>, hint: &'static str) -> ProgressBar {
        let bar = self.0.add(ProgressBar::with_draw_target(
            len,
            ProgressDrawTarget::stderr(),
        ));
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.bold} {bar:40.cyan/blue} {pos}/{len} files ({eta})")
                .unwrap()
                .progress_chars("##-"),
        );
        bar.set_prefix(format!("[{}]", hint));

        bar
    }
}

pub fn print_step(step: u8, emoji: console::Emoji, message: &str) {
    println!(
        "{} {}{}",
        style(format!("[{}/3]", step)).bold().dim(),
        emoji,
        message
    );
}
