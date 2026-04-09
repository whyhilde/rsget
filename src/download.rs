use anyhow::Result;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download(url: &str, path: &str) -> Result<()> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    let mut file = File::create(path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        file.write_all(&bytes).await?;
    }

    Ok(())
}
