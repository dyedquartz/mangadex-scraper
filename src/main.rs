extern crate clap;
extern crate reqwest;
mod mangadex_api;

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::{io, thread, time};

fn main() -> Result<(), reqwest::Error> {
    // command line arguments
    let args = App::new("mangadex-scraper")
        .version("0.5.1")
        .author("dyedquartz <dyedquartz@gmail.com>")
        .about("Scrapes manga off of mangadex.org")
        .arg(
            Arg::with_name("id")
                .help("ID of the item to download")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("lang")
                .short("l")
                .long("language")
                .value_name("LANGUAGE")
                .help("Downloads chapters for specific languages")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("chapter")
                .short("c")
                .long("chapter")
                .help("Downloads a single chapter"),
        )
        .arg(
            Arg::with_name("volume")
                .short("e")
                .long("volume")
                .takes_value(true)
                .value_name("VOLUME")
                .help("Downloads an entire volume"),
        )
        /*
        .arg(
            Arg::with_name("archive")
                .short("a")
                .long("archive")
                .help("archives into a zip"),
        )
        */
        .get_matches();

    if args.is_present("chapter") && args.is_present("volume") {
        println!("Both chapter and volume cannot be used at the same time");
        std::process::exit(1);
    }

    let client = reqwest::blocking::Client::new();

    if args.is_present("chapter") {
        let chapter_data = mangadex_api::get_chapter_data(&client, args.value_of("id").unwrap());
        let manga_data = mangadex_api::get_manga_data(&client, &chapter_data.manga_id.to_string());
        let data = manga_data
            .chapter
            .get(&chapter_data.id.to_string())
            .unwrap();
        println!(
            "Scraping '{} Vol. {} Ch. {} in {} from {}'",
            manga_data.manga.title, data.volume, data.chapter, data.lang_code, data.group_name
        );
        download_chapter(&client, chapter_data.id.to_string(), data, &manga_data);
    } else if args.is_present("volume") {
        let volume = args.value_of("volume").unwrap();
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
        println!("Scraping '{} Vol. {}'", manga_data.manga.title, volume);
        for (name, data) in &manga_data.chapter {
            if data.volume != volume {
                continue;
            }
            if args.is_present("lang") {
                if data.lang_code != args.value_of("lang").unwrap() {
                    continue;
                }
            }

            download_chapter(&client, name.to_string(), &data, &manga_data);
        }
    } else {
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
        //let mut chapter_count = 0;
        println!(
            "Scraping '{}' in {}",
            manga_data.manga.title,
            if !args.is_present("lang") {
                "All"
            } else {
                args.value_of("lang").unwrap()
            }
        );

        for (name, data) in &manga_data.chapter {
            if args.is_present("lang") {
                if data.lang_code != args.value_of("lang").unwrap() {
                    //chapter_count += 1;
                    continue;
                }
            }

            download_chapter(&client, name.to_string(), data, &manga_data);
            //chapter_count += 1;
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

fn clean_title(original: &str) -> String {
    original.replace(":", "-")
}


fn download_chapter(
    client: &reqwest::blocking::Client,
    name: String,
    data: &mangadex_api::Chapter,
    manga_data: &mangadex_api::MangaData,
) {
    let chapter_data = mangadex_api::get_chapter_data(&client, &name);
    //let mut page_count = 0;
    //let page_length = &chapter_data.page_array.len();

    for page in chapter_data.page_array {
        let current_time = time::Instant::now();
        let page_name = format!("{:0>8}", page.trim_start_matches(char::is_alphabetic));

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
        //println!("downloading {}", &url);
        let mut resp = client.get(url).send().unwrap();
        fs::create_dir_all(strip_characters(
            &*format!(
                "{} Vol. {} Ch. {} - {} ({})",
                clean_title(&*manga_data.manga.title),
                format!("{:0>4}", data.volume),
                format!("{:0>4}", data.chapter),
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
                    clean_title(&*manga_data.manga.title),
                    format!("{:0>4}", data.volume),
                    format!("{:0>4}", data.chapter),
                    data.group_name,
                    data.lang_code,
                ),
                "/",
            ))
            .join(&page_name),
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
                            clean_title(&*manga_data.manga.title),
                            format!("{:0>4}", data.volume),
                            format!("{:0>4}", data.chapter),
                            data.group_name,
                            data.lang_code,
                        ),
                        "/",
                    ))
                    .join(&page_name),
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
                //println!("downloading {}", &url);
                let mut resp = client.get(url).send().unwrap();
                fs::create_dir_all(strip_characters(
                    &*format!(
                        "{} Vol. {} Ch. {} - {} ({})",
                        clean_title(&*manga_data.manga.title),
                        format!("{:0>4}", data.volume),
                        format!("{:0>4}", data.chapter),
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
                            clean_title(&*manga_data.manga.title),
                            format!("{:0>4}", data.volume),
                            format!("{:0>4}", data.chapter),
                            data.group_name,
                            data.lang_code,
                        ),
                        "/",
                    ))
                    .join(&page_name),
                )
                .expect("failure to create image");
                io::copy(&mut resp, &mut out).expect("failure to copy to image a second time");
                0
            }
        };
        //page_count += 1;
        while time::Instant::now()
            .duration_since(current_time)
            .as_millis()
            <= 1000
        {
            thread::sleep(time::Duration::from_millis(100));
        }
    }

    println!(
        "Downloaded '{} Vol. {} Ch. {} in {} from {}'",
        manga_data.manga.title, data.volume, data.chapter, data.lang_code, data.group_name
    );
}
