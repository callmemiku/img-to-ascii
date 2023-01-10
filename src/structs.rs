#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    #[arg(short, long)]
    path: String,

    /// Size limit (mb)
    #[arg(short, long)]
    size: u8,
}