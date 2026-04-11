use indicatif::{ProgressBar, ProgressStyle};

pub struct Progress {
    bar: Option<ProgressBar>,
}

impl Progress {
    pub fn new(total_size: u64, quiet: bool) -> Self {
        let bar = if !quiet && total_size > 0 {
            let pb = ProgressBar::new(total_size);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} |{bar:40.cyan/blue}| {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#### "),
            );
            Some(pb)
        } else {
            None
        };
        Self { bar }
    }

    pub fn update(&self, current: u64) {
        if let Some(pb) = &self.bar {
            pb.set_position(current);
        }
    }

    pub fn finish(&self) {
        if let Some(pb) = &self.bar {
            pb.finish_with_message("Download complete!");
        }
    }
}
