use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rsget", about = "Download files")]
pub struct Args {
    pub url: String,

    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn get_args() -> Args {
    Args::parse()
}
