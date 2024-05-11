use audiotags::Tag;
use std::collections::HashMap;

use crate::PathBuf;

pub fn show(file: &PathBuf) {
    let tag = Tag::new().read_from_path(file).expect("Audio Not Found");

    let mut all_tags: HashMap<&str, String> = HashMap::new();

    if tag.title().is_some() {
        all_tags.insert("Title", tag.title().unwrap().to_string());
    }

    if tag.artists().is_some() {
        all_tags.insert("Artist", tag.artist().unwrap().to_string());
    }

    if tag.artists().is_some() {
        all_tags.insert("Artist", tag.artist().unwrap().to_string());
    }

    if tag.year().is_some() {
        all_tags.insert("Date", tag.year().unwrap().to_string());
    }

    if tag.duration().is_some() {
        all_tags.insert("Duration", tag.duration().unwrap().to_string());
    }

    if tag.album_title().is_some() {
        all_tags.insert("Album Title", tag.album_title().unwrap().to_string());
    }

    if tag.album_artist().is_some() {
        all_tags.insert("Album Artists", tag.album_artist().unwrap().to_string());
    }

    if tag.genre().is_some() {
        all_tags.insert("Genre", tag.genre().unwrap().to_string());
    }

    if tag.composer().is_some() {
        all_tags.insert("Composer", tag.composer().unwrap().to_string());
    }

    if tag.comment().is_some() {
        all_tags.insert("Composer", tag.comment().unwrap().to_string());
    }

    if tag.track_number().is_some() {
        all_tags.insert("Composer", tag.track_number().unwrap().to_string());
    }

    println!("{:#?}", all_tags);
}
