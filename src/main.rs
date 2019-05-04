extern crate reqwest;
extern crate regex;
extern crate clap;

use clap::{Arg, App};
use reqwest::Url;
use std::env;
use std::fs;
use std::fs::File;
use std::io;


fn main() -> Result<(), reqwest::UrlError> {
    // command line arguments
    let args = App::new("mangadex-scraper")
        .version("0.2.0")
        .author("dyedquartz <dyedquartz@gmail.com>")
        .about("Scapes manga off of mangadex.org")
        .arg(Arg::with_name("id")
             .help("ID of the directory to download")
             .required(true)
             .index(1))
        .arg(Arg::with_name("compress")
             .short("c")
             .long("compress")
             .help("Compresses into a .cbz"))
        .get_matches();


    let base_url = Url::parse("https://s2.mangadex.org/data/")?;
    let prefixes = vec!["", "x", "s"];
    let id: &str = &args.value_of("id").unwrap();
    let mut pre = String::new();

    let client = reqwest::Client::new();
    
    // testing id directory
    let url = base_url.join(id)?;
    let resp = client.get(url).send().unwrap();
    match resp.status() {
        reqwest::StatusCode::FORBIDDEN => println!("Correct ID Path"),
        reqwest::StatusCode::NOT_FOUND => panic!("Incorrect ID Path"),
        _ => panic!("Unknown ID Path"),
    }

    
    // grabbing correct file prefix
    for prefix in prefixes {
        let url = base_url.join(&format!("{}/{}1.png",id, prefix))?;
        println!("{:?}", url);
        let resp = client.get(url).send().unwrap();
        if resp.status() == reqwest::StatusCode::OK {
            pre = String::from(prefix);
            break;
        }
        println!("{:?}",resp.status());
    }

    println!("File Prefix: {}", pre);
   
    // downloading files
    let mut i = 1;
    loop {
		let re = regex::Regex::new(r"\b\d\b").unwrap();
        let f = &*i.to_string();
        let f = re.replace_all(f, "0$0");

        println!("{}", f);

        //fs::create_dir_all(format!("{}", args[3])).unwrap();
        let url = base_url.join(&format!("{}/{}{}.png",id, pre, i))?;
        let mut resp = client.get(url).send().unwrap();
        if resp.status() == reqwest::StatusCode::OK {
            let mut out = File::create(format!("{}.png", f)).expect("failed to create file");
            io::copy(&mut resp, &mut out).expect("failed to copy");
        } else {
            println!("{:?} no more files to download", resp.status());
            break;
        }
        i += 1;
    }
    Ok(())
}
