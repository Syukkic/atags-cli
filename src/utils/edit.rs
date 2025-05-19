use anyhow::{Context, Result};
use audiotags::{MimeType, Picture, Tag};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::Commands;

fn read_image(path: &str) -> Result<Vec<u8>> {
    let mut f = File::open(path).with_context(|| format!("Failed to open image: {}", path))?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .with_context(|| format!("Failed to read image: {}", path))?;
    Ok(buffer)
}

fn mime_type_from_path(path: &str) -> Result<MimeType> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| anyhow::anyhow!("Missing or invalid file extension in '{}'", path))?;

    let mime = format!("image/{}", ext.to_lowercase());
    MimeType::try_from(mime.as_str())
        .with_context(|| format!("Unsupported image mime type: {}", mime))
}

pub fn set(args: &Commands) -> Result<()> {
    let mut tag = Tag::new()
        .read_from_path(&args.name)
        .with_context(|| format!("Failed to read audio file: {:?}", args.name))?;

    if let Some(title) = &args.title {
        tag.set_title(title);
    }
    if let Some(album_title) = &args.album_title {
        tag.set_album_title(album_title);
    }
    if let Some(artist) = &args.artist {
        tag.set_artist(artist);
    }
    if let Some(genre) = &args.genre {
        tag.set_genre(genre);
    }
    if let Some(composer) = &args.composer {
        tag.set_composer(composer.to_string());
    }
    if let Some(track_number) = args.track_number {
        tag.set_track_number(track_number);
    }

    if let Some(cover_path) = &args.album_cover {
        let mime_type = mime_type_from_path(cover_path)?;
        let data = read_image(cover_path)?;
        let picture = Picture {
            data: &data,
            mime_type,
        };
        tag.set_album_cover(picture);
        tag.write_to_path(args.name.to_str().unwrap())?;
        return Ok(());
    }

    tag.write_to_path(args.name.to_str().unwrap())
        .with_context(|| "Failed to write tags to file")?;

    Ok(())
}
