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

    if title.is_some() {
        tag.set_title(title.as_deref().unwrap())
    }

    if album_title.is_some() {
        tag.set_album_title(album_title.as_deref().unwrap())
    }

    if artist.is_some() {
        tag.set_artist(artist.as_deref().unwrap())
    }

    if genre.is_some() {
        tag.set_genre(genre.as_ref().unwrap())
    }

    if composer.is_some() {
        tag.set_composer(composer.as_ref().unwrap().to_string())
    }

    if track_number.is_some() {
        tag.set_track_number(*track_number.as_ref().unwrap())
    }

    tag.write_to_path(file.as_os_str().to_str().unwrap())
        .expect("Fail to save");
}
