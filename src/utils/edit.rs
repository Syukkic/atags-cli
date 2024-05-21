use audiotags::Tag;

use crate::Commands;

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

    tag.write_to_path(args.name.to_str().unwrap())
        .expect("Fail to save");
}
