use anyhow::{Context, Ok, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct Downloader {
    client: Client,
    url: String,
    output: String,
    quiet: bool,
}

impl Downloader {
    pub fn new(url: String, output: String, quiet: bool) -> Self {
        Self {
            client: Client::new(),
            url,
            output,
            quiet,
        }
    }

    pub async fn download(&self) -> Result<()> {
        let response = self.client.get(&self.url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP error: {}", response.status());
        }

        let total_size = response.content_length().unwrap_or(0);

        let pb = if !self.quiet && total_size > 0 {
            let pb = ProgressBar::new(total_size);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_size} ({eta})")
                    .unwrap()
                    .progress_chars("#### "),
            );
            Some(pb)
        } else if !self.quiet {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {bytes} downloaded...")
                    .unwrap(),
            );
            Some(pb)
        } else {
            None
        };

        let mut file = File::create(&self.output).await?;

        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed")?;

            file.write_all(&chunk).await?;

            downloaded += chunk.len() as u64;

            if let Some(pb) = &pb {
                pb.set_position(downloaded);
            }
        }

        if let Some(pb) = pb {
            pb.finish_with_message("Download complete!");
        }

        Ok(())
    }
}
