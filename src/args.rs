use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rsget", about = "Download files")]
pub struct Args {
    pub url: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub quiet: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
