use audiotags::Tag;
use std::collections::HashMap;

use crate::PathBuf;

pub fn show(file: &PathBuf) {
    let tag = Tag::new().read_from_path(file).expect("Audio Not Found");

    let mut all_tags: HashMap<&str, String> = HashMap::new();

    if let Some(title) = tag.title() {
        all_tags.insert("Title", title.to_string());
    }

    if let Some(artist) = tag.artist() {
        all_tags.insert("Artist", artist.to_string());
    }

    if let Some(date) = tag.year() {
        all_tags.insert("Date", date.to_string());
    }

    if let Some(duration) = tag.duration() {
        all_tags.insert("Duration", duration.to_string());
    }

    if let Some(album_title) = tag.album_title() {
        all_tags.insert("Album Title", album_title.to_string());
    }

    if let Some(album_artist) = tag.album_artist() {
        all_tags.insert("Album Artists", album_artist.to_string());
    }

    if let Some(genre) = tag.genre() {
        all_tags.insert("Genre", genre.to_string());
    }

    if let Some(composer) = tag.composer() {
        all_tags.insert("Composer", composer.to_string());
    }

    if let Some(comment) = tag.comment() {
        all_tags.insert("Comment", comment.to_string());
    }

    if let Some(track_number) = tag.track_number() {
        all_tags.insert("Track_number", track_number.to_string());
    }

    println!("{:#?}", all_tags);
}
