use audiotags::Tag;
use std::collections::HashMap;

use crate::PathBuf;

pub fn show(file: &PathBuf) {
    let tag = Tag::new().read_from_path(file).expect("Audio Not Found");

    let mut all_tags: HashMap<&str, String> = HashMap::new();

    let tag_pairs = &[
        ("Title", tag.title().map(ToString::to_string)),
        ("Artist", tag.artist().map(ToString::to_string)),
        ("Date", tag.year().map(|y| y.to_string())),
        ("Duration", tag.duration().map(|d| d.to_string())),
        ("Album Title", tag.album_title().map(ToString::to_string)),
        ("Album Artists", tag.album_artist().map(ToString::to_string)),
        ("Genre", tag.genre().map(ToString::to_string)),
        ("Composer", tag.composer().map(ToString::to_string)),
        ("Comment", tag.comment().map(ToString::to_string)),
        ("Track_number", tag.track_number().map(|t| t.to_string())),
    ];

    for (key, value) in tag_pairs {
        if let Some(v) = value {
            all_tags.insert(key, v.clone());
        }
    }
    println!("{:#?}", all_tags);
}
