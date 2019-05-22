extern crate serde;
extern crate reqwest;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Manga {
    title: String,
}

#[derive(Debug, Deserialize)]
struct Chapter {
    volume: String,
    chapter: String,
    lang_code: String,
    group_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaData {
    manga: Manga,
    chapter: HashMap<String, Chapter>,
}

pub fn get_manga_data(client: &mut reqwest::Client, manga: &str) -> MangaData {
    let base_url = reqwest::Url::parse("https://mangadex.org/api/manga/").unwrap();
    let url = base_url.join(manga).unwrap();

    let json: MangaData = client.get(url).send().unwrap().json().unwrap();
    json
}
