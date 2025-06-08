use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct Bar(MultiProgress);

impl Bar {
    pub fn new() -> Self {
        Self(MultiProgress::new())
    }

    pub fn generate(&self, len: u64, hint: &'static str) -> ProgressBar {
        let bar = self.0.add(ProgressBar::new(len));
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
