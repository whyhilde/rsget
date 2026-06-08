use anyhow::{Context, Ok, Result};
use futures_util::StreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::progress::Progress;

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

        let pb = Progress::new(total_size, self.quiet);

        let mut file = File::create(&self.output).await?;

        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed")?;

            file.write_all(&chunk).await?;

            downloaded += chunk.len() as u64;

            pb.update(downloaded);
        }

        file.flush().await?;

        pb.finish();

        Ok(())
    }
}
