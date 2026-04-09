mod args;
mod download;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::get_args();

    let filename = match args.output {
        Some(name) => name,
        None => utils::extract_filename(&args.url),
    };

    download::download(&args.url, &filename).await?;

    println!("Downloaded: {}", filename); // ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    Ok(())
}
