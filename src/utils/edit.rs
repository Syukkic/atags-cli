use audiotags::{MimeType, Picture, Tag};
use std::path::Path;
use std::{fs::File, io::Read};

use crate::Commands;

fn read_image(img: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut f = File::open(img)?;
    let mut buffer = Vec::new();
    let _ = f.read_to_end(&mut buffer);

    Ok(buffer)
}

pub fn set(args: &Commands) {
    let mut tag = Tag::new()
        .read_from_path(&args.name)
        .expect("Audio File Not Found");

    if let Some(title) = &args.title {
        tag.set_title(title.as_str())
    }

    if let Some(album_title) = &args.album_title {
        tag.set_album_title(album_title.as_str())
    }

    if let Some(artist) = &args.artist {
        tag.set_artist(artist.as_str())
    }

    if let Some(genre) = &args.genre {
        tag.set_genre(genre.as_str())
    }

    if let Some(composer) = &args.composer {
        tag.set_composer(composer.to_string())
    }

    if let Some(track_number) = args.track_number {
        tag.set_track_number(track_number)
    }

    if let Some(album_cover) = &args.album_cover {
        let extension = Path::new(album_cover)
            .extension()
            .unwrap()
            .to_str()
            .unwrap();

        let mut inp_ext = "image/".to_string();
        inp_ext.push_str(extension);

        let mime_type = MimeType::try_from(inp_ext.as_str()).unwrap();
        let data = read_image(album_cover.as_str()).unwrap();
        let album_cover = Picture {
            data: &data,
            mime_type,
        };
        tag.set_album_cover(album_cover);
    }

    tag.write_to_path(args.name.to_str().unwrap())
        .expect("Fail to save");
}
