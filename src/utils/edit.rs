use audiotags::Tag;

use crate::PathBuf;

pub fn set(
    file: &PathBuf,
    title: Option<String>,
    artist: Option<String>,
    album_title: Option<String>,
    genre: Option<String>,
    composer: Option<String>,
    track_number: Option<u16>,
) {
    let mut tag = Tag::new()
        .read_from_path(file)
        .expect("Audio File Not Found");

    if let Some(title) = title {
        tag.set_title(title.as_str())
    }

    if let Some(album_title) = album_title {
        tag.set_album_title(album_title.as_str())
    }

    if let Some(artist) = artist {
        tag.set_artist(artist.as_str())
    }

    if let Some(genre) = genre {
        tag.set_genre(genre.as_str())
    }

    if let Some(composer) = composer {
        tag.set_composer(composer)
    }

    if let Some(track_number) = track_number {
        tag.set_track_number(track_number)
    }

    tag.write_to_path(file.as_os_str().to_str().unwrap())
        .expect("Fail to save");
}
