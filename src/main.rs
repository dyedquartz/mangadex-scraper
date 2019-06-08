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
        .version("0.3.0")
        .author("dyedquartz <dyedquartz@gmail.com>")
        .about("Scapes manga off of mangadex.org")
        .arg(
            Arg::with_name("id")
                .help("ID of the directory to download")
                .required(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("manga").arg(
                Arg::with_name("lang")
                    .short("l")
                    .long("lang")
                    .value_name("LANGUAGE")
                    .help("Downloads chapters for specific langages")
                    .takes_value(true),
            ),
        )
        .subcommand(SubCommand::with_name("chapter"))
        .arg(
            Arg::with_name("compress")
                .short("c")
                .long("compress")
                .value_name("ARCHIVE_OUTPUT")
                .help("Compresses into a .cbz")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("remove")
                .long("remove")
                .help("Remove file after downloading. Most useful for cleanup after compressing"),
        )
        .get_matches();
    let client = reqwest::Client::new();
    // getting subcommand higynx
    if let Some(manga) = args.subcommand_matches("manga") {
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
        println!("Scraping '{}'", manga_data.manga.title);

        for (name, data) in manga_data.chapter {
            if manga.is_present("lang") {
                if data.lang_code != manga.value_of("lang").unwrap() {
                    continue;
                }
            }
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
                    manga_data.manga.title,
                    data.volume,
                    data.chapter,
                    data.group_name,
                    data.lang_code
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
                let copy = io::copy(&mut resp, &mut out);
                let copy = match copy {
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
                        let copy = io::copy(&mut resp, &mut out);                        0
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
    }
    /*
    // downloading files
    let mut i = 1;
    loop {
        let re = regex::Regex::new(r"\b\d\b").unwrap();
        let f = &*i.to_string();
        let f = re.replace_all(f, "0$0");
        //fs::create_dir_all(format!("{}", args[3]])).unwrap();
        let url = base_url.join(&format!("{}/{}{}.png", id, pre, i))?;
        let mut resp = client.get(url).send().unwrap();
        if resp.status() == reqwest::StatusCode::OK {
            let mut out = File::create(format!("{}.png", f)).expect("failed to create file");
            io::copy(&mut resp, &mut out).expect("failed to copy");
        } else {
            let url = base_url.join(&format!("{}/{}{}.jpg", id, pre, i))?;
            let mut resp = client.get(url).send().unwrap();
            if resp.status() == reqwest::StatusCode::OK {
                let mut out = File::create(format!("{}.jpg", f)).expect("failed to create file");
                io::copy(&mut resp, &mut out).expect("failed to copy");
            } else {
                println!("{:?} no more files to download", resp.status());
                break;
            }
        }
        println!("Downloaded {}", f);
        i += 1;
    }
    if args.is_present("compress") {
        // create archive + buffer
        let mut buffer = Vec::new();
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);
        let mut archive =
            File::create(format!("{}.cbz", args.value_of("compress").unwrap())).unwrap();
        let mut writer = zip::write::ZipWriter::new(&mut archive);
        println!("created writer and archive");
        for archive_file in 1..i {
            let re = regex::Regex::new(r"\b\d\b").unwrap();
            let f = &*archive_file.to_string();
            let f = re.replace_all(f, "0$0");
            let mut path = format!("{}.png", f);            let image = File::open(&path);
            let mut image = match image {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => match File::open(format!("{}.jpg", f)) {
                        Ok(jpg) => {
                            path = format!("{}.jpg", f);
                            jpg
                        }
                        Err(e) => panic!("problem opening file for archiving {:?}", e),
                    },
                    other_error => panic!("problem opening file for archiving {:?}", other_error),
                },
            };
            image.read_to_end(&mut buffer).unwrap();
            writer.start_file(&*path, options).unwrap();
            writer.write_all(&*buffer).unwrap();
            buffer.clear();
            println!("Compressed {}", path);
            if args.is_present("remove") {
                std::fs::remove_file(&path).unwrap();
                println!("Removed {}", path);
            }
        }
        writer.finish().unwrap();
    }
    */
    Ok(())
}

fn strip_characters(original: &str, to_strip: &str) -> String {
    original
        .chars()
        .filter(|&c| !to_strip.contains(c))
        .collect()
}
