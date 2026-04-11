mod args;
mod download;
mod progress;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::get_args();

    let filename = match args.output {
        Some(name) => name,
        None => utils::extract_filename(&args.url),
    };

    let downloader = download::Downloader::new(args.url, filename, args.quiet);

    downloader.download().await?;

    Ok(())
}
