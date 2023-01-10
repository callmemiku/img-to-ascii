use clap::Parser;

///img-to-ascii converter cli app
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to file relative to .exe
    #[arg(short, long)]
    pub path: String,

    /// Number of pixels per symbol
    #[arg(short, long)]
    pub resolution: i8,

    /// Path to output file
    #[arg(short, long, default_value_t = String::from("output.txt"))]
    pub output: String,
}

impl Args {
    pub fn parse_args() -> Args {
        Args::parse()
    }
}