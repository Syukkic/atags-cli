mod utils;
use clap::Parser;
use utils::{display::show, edit::set};

pub type PathBuf = std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Commands {
    /// File Path
    name: PathBuf,

    /// Set audio title
    #[arg(short, long)]
    title: Option<String>,
    /// Set audio artist
    #[arg(short, long)]
    artist: Option<String>,
    /// Set audio genre
    #[arg(short, long)]
    genre: Option<String>,
    /// Set audio composer
    #[arg(short, long)]
    composer: Option<String>,
    /// Set audio track number
    #[arg(long)]
    track_number: Option<u16>,
    /// Set audio album title
    #[arg(long)]
    album_title: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Commands::parse();
    set(&args);
    show(&args.name);

    // println!("{:?}", Args);

    Ok(())
}
