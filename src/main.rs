extern crate reqwest;
extern crate regex;

use reqwest::Url;
use std::env;
use std::fs;
use std::fs::File;
use std::io;


fn main() -> Result<(), reqwest::UrlError> {
    // command line arguments
    let args: Vec<String> = env::args().collect();

    let id: &str = &args[1];

    println!("{:?}", args);
    println!("{}", id);

    if args.len() != 4 { panic!("mangadex-scraper <id> <amount> <output>"); }

    let base_url = Url::parse("https://s2.mangadex.org/data/")?;
    let prefixes = vec!["", "x", "s"];
    let id: &str = &args[1];
    println!("{:?}", args);
    println!("{}", id);

    let client = reqwest::Client::new();
    
    // testing id directory
    let url = base_url.join(id)?;
    let resp = client.get(url).send().unwrap();
    match resp.status() {
        reqwest::StatusCode::FORBIDDEN => println!("Correct ID Path"),
        reqwest::StatusCode::NOT_FOUND => panic!("Incorrect ID Path"),
        _ => panic!("Unknown ID Path"),
    }

    let mut pre = String::new();
    
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
    for i in 1..args[2].parse::<i32>().unwrap()+1 {
		let re = regex::Regex::new(r"\b\d\b").unwrap();
        let i = &*i.to_string();
        let f = re.replace_all(i, "0$0");

        println!("{}", i);

        fs::create_dir_all(format!("{}", args[3])).unwrap();
        let url = base_url.join(&format!("{}/{}{}.png",id, pre, i))?;
        let mut resp = client.get(url).send().unwrap();
        let mut out = File::create(format!("{}/{}.png", args[3], f)).expect("failed to create file");
        io::copy(&mut resp, &mut out).expect("failed to copy");
    }
    Ok(())
}
