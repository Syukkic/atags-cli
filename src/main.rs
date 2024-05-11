mod utils;
use clap::{Parser, Subcommand};
use utils::{display::show, edit::set};

pub type PathBuf = std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show audio exiting metadata
    Show { file: PathBuf },
    /// Set audio metadata
    Set {
        file: PathBuf,

        /// Set audio name
        #[arg(short, long)]
        title: Option<String>,
        /// Set audio artist(s)
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
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    if let Some(cli) = cli.command {
        match cli {
            Commands::Show { file } => show(&file),
            Commands::Set {
                file,
                title,
                artist,
                album_title,
                genre,
                composer,
                track_number,
            } => set(
                &file,
                title,
                artist,
                album_title,
                genre,
                composer,
                track_number,
            ),
        }
    }

    Ok(())
}
