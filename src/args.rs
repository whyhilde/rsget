use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    pub url: String,

    /// Set output filename
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Quiet mode
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub quiet: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
