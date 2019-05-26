extern crate reqwest;
extern crate serde;use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Manga {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub volume: String,
    pub chapter: String,
    pub lang_code: String,
    pub group_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaData {
    pub manga: Manga,
    pub chapter: HashMap<String, Chapter>,
}

pub fn get_manga_data(client: &reqwest::Client, manga: &str) -> MangaData {
    let base_url = reqwest::Url::parse("https://mangadex.org/api/manga/").unwrap();
    let url = base_url.join(manga).unwrap();

    let json: MangaData = client.get(url).send().unwrap().json().unwrap();
    json
}
