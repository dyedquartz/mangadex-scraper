extern crate clap;
extern crate regex;
extern crate reqwest;
mod mangadex_api;

use clap::{App, Arg, SubCommand};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
fn main() -> Result<(), reqwest::UrlError> {
    // command line arguments
    let args = App::new("mangadex-scraper")
        .version("0.3.1")
        .author("dyedquartz <dyedquartz@gmail.com>")
        .about("Scapes manga off of mangadex.org")
        .arg(
            Arg::with_name("id")
                .help("ID of the item to download")
                .required(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("manga")
                .arg(
                    Arg::with_name("lang")
                        .short("l")
                        .long("language")
                        .value_name("LANGUAGE")
                        .help("Downloads chapters for specific langages")
                        .takes_value(true),
                )
                .about("Downloads an entire manga"),
        )
        .subcommand(SubCommand::with_name("chapter").about("Downloads a single chapter"))
        .subcommand(
            SubCommand::with_name("volume")
                .arg(
                    Arg::with_name("lang")
                        .short("l")
                        .long("language")
                        .value_name("LANGUAGE")
                        .help("Downloads chapters for specific languages")
                        .takes_value(true),
                )
                .about("Downloads an entire volume")
                .arg(
                    Arg::with_name("id")
                        .help("Volume number to download")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();
    let client = reqwest::Client::new();

    // getting subcommand higynx
    if let Some(manga) = args.subcommand_matches("manga") {
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
        println!("Scraping '{}'", manga_data.manga.title);

        for (name, data) in &manga_data.chapter {
            if manga.is_present("lang") {
                if data.lang_code != manga.value_of("lang").unwrap() {
                    continue;
                }
            }
            download_chapter(&client, name.to_string(), data, &manga_data);
        }
    }

    if args.is_present("chapter") {
        let chapter_data = mangadex_api::get_chapter_data(&client, args.value_of("id").unwrap());
        let manga_data = mangadex_api::get_manga_data(&client, &chapter_data.manga_id.to_string());
        let data = manga_data.chapter.get(&chapter_data.id.to_string()).unwrap();
        println!("Scraping '{} Vol. {} Ch. {} in {} from {}'", manga_data.manga.title, data.volume, data.chapter, data.lang_code, data.group_name);

        download_chapter(&client, chapter_data.id.to_string(), data, &manga_data);
    }

    if let Some(volume) = args.subcommand_matches("volume") {
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
        println!("Scraping '{} Vol. {}'", manga_data.manga.title, volume.value_of("id").unwrap());

        for (name, data) in &manga_data.chapter {
            if data.volume != volume.value_of("id").unwrap() {
                continue;
            }
            if volume.is_present("lang") {
                if data.lang_code != volume.value_of("lang").unwrap() {
                    continue;
                }
            }

            download_chapter(&client, name.to_string(), &data, &manga_data);
        }
    }
    Ok(())
}

fn strip_characters(original: &str, to_strip: &str) -> String {
    original
        .chars()
        .filter(|&c| !to_strip.contains(c))
        .collect()
}

fn download_chapter(
    client: &reqwest::Client,
    name: String,
    data: &mangadex_api::Chapter,
    manga_data: &mangadex_api::MangaData,
) {
    println!(
        "{}: volume {} chapter {} in {} from {}",
        name, data.volume, data.chapter, data.lang_code, data.group_name
    );
    let chapter_data = mangadex_api::get_chapter_data(&client, &name);
    println!("{:#?}", chapter_data);
    let mut buffer = Vec::new();
    let options = zip::write::FileOptions::default();
    let mut archive = File::create(strip_characters(
        &*format!(
            "{} Vol. {} Ch. {} - {} ({}).cbz",
            manga_data.manga.title, data.volume, data.chapter, data.group_name, data.lang_code
        ),
        "/",
    ))
    .expect("failure to create archive");
    let mut writer = zip::write::ZipWriter::new(&mut archive);
    for page in chapter_data.page_array {
        let url = if chapter_data.server == "/data/" {
            reqwest::Url::parse(&*format!(
                "https://mangadex.org/data/{}/{}",
                chapter_data.hash, page
            ))
            .unwrap()
        } else {
            reqwest::Url::parse(&*format!(
                "{}{}/{}",
                chapter_data.server, chapter_data.hash, page
            ))
            .unwrap()
        };
        println!("downloading {}", &url);
        let mut resp = client.get(url).send().unwrap();
        fs::create_dir_all(strip_characters(
            &*format!(
                "{} Vol. {} Ch. {} - {} ({})",
                manga_data.manga.title, data.volume, data.chapter, data.group_name, data.lang_code
            ),
            "/",
        ))
        .unwrap();
        let mut out = File::create(
            std::path::Path::new(&*strip_characters(
                &*format!(
                    "{} Vol. {} Ch. {} - {} ({})",
                    manga_data.manga.title,
                    data.volume,
                    data.chapter,
                    data.group_name,
                    data.lang_code,
                ),
                "/",
            ))
            .join(&page),
        )
        .expect("failure to create image");
        let _copy = io::copy(&mut resp, &mut out);
        let _copy = match _copy {
            Ok(file) => file,
            Err(error) => {
                println!("Error Copying to File, trying again: {}", error);
                std::fs::remove_file(
                    std::path::Path::new(&*strip_characters(
                        &*format!(
                            "{} Vol. {} Ch. {} - {} ({})",
                            manga_data.manga.title,
                            data.volume,
                            data.chapter,
                            data.group_name,
                            data.lang_code,
                        ),
                        "/",
                    ))
                    .join(&page),
                )
                .unwrap();
                let url = if chapter_data.server == "/data/" {
                    reqwest::Url::parse(&*format!(
                        "https://mangadex.org/data/{}/{}",
                        chapter_data.hash, page
                    ))
                    .unwrap()
                } else {
                    reqwest::Url::parse(&*format!(
                        "{}{}/{}",
                        chapter_data.server, chapter_data.hash, page
                    ))
                    .unwrap()
                };
                println!("downloading {}", &url);
                let mut resp = client.get(url).send().unwrap();
                fs::create_dir_all(strip_characters(
                    &*format!(
                        "{} Vol. {} Ch. {} - {} ({})",
                        manga_data.manga.title,
                        data.volume,
                        data.chapter,
                        data.group_name,
                        data.lang_code
                    ),
                    "/",
                ))
                .unwrap();
                let mut out = File::create(
                    std::path::Path::new(&*strip_characters(
                        &*format!(
                            "{} Vol. {} Ch. {} - {} ({})",
                            manga_data.manga.title,
                            data.volume,
                            data.chapter,
                            data.group_name,
                            data.lang_code,
                        ),
                        "/",
                    ))
                    .join(&page),
                )
                .expect("failure to create image");
                io::copy(&mut resp, &mut out).expect("failure to copy to image a second time");
                0
            }
        };
        println!("compressing {}", &page);
        let mut image = File::open(
            std::path::Path::new(&*strip_characters(
                &*format!(
                    "{} Vol. {} Ch. {} - {} ({})",
                    manga_data.manga.title,
                    data.volume,
                    data.chapter,
                    data.group_name,
                    data.lang_code,
                ),
                "/",
            ))
            .join(&page),
        )
        .unwrap();
        image.read_to_end(&mut buffer).unwrap();
        writer.start_file(&*page, options).unwrap();
        writer.write_all(&*buffer).unwrap();
        buffer.clear();
        println!("compressed {}", &page);
    }
    writer.finish().unwrap();
}
