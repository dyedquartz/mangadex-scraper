extern crate clap;
extern crate reqwest;
extern crate termion;
mod mangadex_api;

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{stdout, Write};
//use termion::async_stdin;
use termion::raw::IntoRawMode;

fn main() -> Result<(), reqwest::UrlError> {
    // command line arguments
    let args = App::new("mangadex-scraper")
        .version("0.5.0")
        .author("dyedquartz <dyedquartz@gmail.com>")
        .about("Scapes manga off of mangadex.org")
        .arg(Arg::with_name("id")
            .help("ID of the item to download")
            .required(true)
            .index(1),
        )
        .arg(Arg::with_name("lang")
            .short("l")
            .long("language")
            .value_name("LANGUAGE")
            .help("Downloads chapters for specific langages")
            .takes_value(true),
        )
        .arg(Arg::with_name("chapter")
             .short("c")
             .long("chapter")
             .help("Downloads a single chapter")
        )
        .arg(Arg::with_name("volume")
             .short("e")
             .long("volume")
             .takes_value(true)
             .value_name("VOLUME")
             .help("Downloads an enture volume")
        )
        .arg(Arg::with_name("archive")
             .short("a")
             .long("archive")
             .help("archives into a zip")
         )
        .get_matches();

    if args.is_present("chapter") && args.is_present("volume") {
       println!("Both chapter and volume cannot be used at the same time");
       std::process::exit(1);
    }
    
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    //let stdin = async_stdin().bytes();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    let client = reqwest::Client::new();

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
        download_chapter(
            &client,
            &mut stdout,
            chapter_data.id.to_string(),
            data,
            &manga_data,
        );
    } else if args.is_present("volume") {
        let volume = args.value_of("volume").unwrap();
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
        println!(
            "Scraping '{} Vol. {}'",
            manga_data.manga.title,
            volume
        );
        for (name, data) in &manga_data.chapter {
            if data.volume != volume {
                continue;
            }
            if args.is_present("lang") {
                if data.lang_code != args.value_of("lang").unwrap() {
                    continue;
                }
            }

            download_chapter(&client, &mut stdout, name.to_string(), &data, &manga_data);
        }
    } else {
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
        write!(
            stdout,
            "Scraping '{}' with {} total chapters",
            manga_data.manga.title,
            manga_data.chapter.len()
        )
        .unwrap();
        let mut chapter_count = 0;
        let mut percentage;

        stdout.flush().unwrap();
        for (name, data) in &manga_data.chapter {
            if args.is_present("lang") {
                if data.lang_code != args.value_of("lang").unwrap() {
                    chapter_count += 1;
                    continue;
                }
            }
            percentage = chapter_count as f32 / manga_data.chapter.len() as f32;

            write!(
                stdout,
                "{}{}Chapter Count: {} / {}",
                termion::cursor::Goto(1, termion::terminal_size().unwrap().1 - 1),
                termion::clear::CurrentLine,
                chapter_count.to_string(),
                manga_data.chapter.len().to_string()
            )
            .unwrap();
            write!(
                stdout,
                "{}{}Manga Progress: {:.0}%       -[",
                termion::cursor::Goto(1, termion::terminal_size().unwrap().1),
                termion::clear::CurrentLine,
                percentage * 100.0,
            )
            .unwrap();

            for _ in 0..(percentage * (termion::terminal_size().unwrap().0 as f32 - 30.0)) as u32 {
                write!(stdout, "=").unwrap();
            }

            write!(
                stdout,
                "{}]-",
                termion::cursor::Goto(
                    termion::terminal_size().unwrap().0 - 1,
                    termion::terminal_size().unwrap().1
                )
            )
            .unwrap();
            stdout.flush().unwrap();

            download_chapter(&client, &mut stdout, name.to_string(), data, &manga_data);
            chapter_count += 1;
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
    stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock<'_>>,
    name: String,
    data: &mangadex_api::Chapter,
    manga_data: &mangadex_api::MangaData)
{
    /*
    println!(
        "{}: volume {} chapter {} in {} from {}",
        name, data.volume, data.chapter, data.lang_code, data.group_name
    );
    */

    let chapter_data = mangadex_api::get_chapter_data(&client, &name);
    //println!("{:#?}", chapter_data);
    let mut page_count = 0;
    let mut percentage;
    let page_length = &chapter_data.page_array.len();

    for page in chapter_data.page_array {
        percentage = page_count as f32 / *page_length as f32;
        write!(
            stdout,
            "{}{}Downloading {} Volume {} Chapter {} in {} from {}",
            termion::cursor::Goto(1, termion::terminal_size().unwrap().1 - 4),
            termion::clear::CurrentLine,
            manga_data.manga.title,
            data.volume, data.chapter, data.lang_code, data.group_name
        ).unwrap();

        write!(
            stdout,
            "{}{}Page Count: {} / {}",
            termion::cursor::Goto(1, termion::terminal_size().unwrap().1 - 3),
            termion::clear::CurrentLine,
            page_count,
            page_length
            ).unwrap();

        write!(
            stdout,
            "{}{}Chapter Progress: {:.0}%     -[",
            termion::cursor::Goto(1, termion::terminal_size().unwrap().1 - 2),
            termion::clear::CurrentLine,
            percentage * 100.0
          ).unwrap();

        for _ in 0..(percentage * (termion::terminal_size().unwrap().0 as f32 - 30.0)) as u32 {
            write!(stdout, "=").unwrap();
        }

        write!(
            stdout,
            "{}]-",
            termion::cursor::Goto(
                termion::terminal_size().unwrap().0 - 1,
                termion::terminal_size().unwrap().1 - 2
            )
        ).unwrap();
        stdout.flush().unwrap();

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
                manga_data.manga.title, format!("{:0>4}", data.volume), format!("{:0>4}", data.chapter), data.group_name, data.lang_code
            ),
            "/",
        ))
        .unwrap();
        let mut out = File::create(
            std::path::Path::new(&*strip_characters(
                &*format!(
                    "{} Vol. {} Ch. {} - {} ({})",
                    manga_data.manga.title,
                    format!("{:0>4}", data.volume),
                    format!("{:0>4}", data.chapter),
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
                            format!("{:0>4}", data.volume),
                            format!("{:0>4}", data.chapter),
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
                //println!("downloading {}", &url);
                let mut resp = client.get(url).send().unwrap();
                fs::create_dir_all(strip_characters(
                    &*format!(
                        "{} Vol. {} Ch. {} - {} ({})",
                        manga_data.manga.title,
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
                            manga_data.manga.title,
                            format!("{:0>4}", data.volume),
                            format!("{:0>4}", data.chapter),
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
        page_count += 1;
    }
}
