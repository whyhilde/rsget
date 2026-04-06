use anyhow::Result;
use clap::Parser;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;

#[derive(Parser, Debug)]
#[command(name = "rsget")]
#[command(about = "Download files", long_about = None)]
struct Args {
    url: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let filename = match args.output {
        Some(name) => name,
        None => extract_filename(&args.url),
    };

    let client = Client::new();
    let mut response = client.get(&args.url).send()?;

    let mut file = File::create(&filename)?;
    copy(&mut response, &mut file)?;

    println!("Downloaded: {}", filename);
    Ok(())
}

fn extract_filename(url: &str) -> String {
    url.split("/").last().unwrap_or("download").to_string()
}
